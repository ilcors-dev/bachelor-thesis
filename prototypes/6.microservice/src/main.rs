#[macro_use] extern crate log;
extern crate simplelog;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, StatusCode, Server};
pub use mysql_async::prelude::*;
pub use mysql_async::*;
use serde_json::json;
use simplelog::*;
use std::collections::HashMap;
use std::convert::Infallible;
use std::fs::File;
use std::net::SocketAddr;
use std::result::Result;
use ulid::{self, Ulid};

mod message;

// using planescale database (not working, ssl / tls not supported by wasi mysql module yet)
// fn get_db_url() -> String {
//     let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
//         "mysql://ypmltdihr6v3aq1ag3hv:pscale_pw_nEKjjpm4FVkXoqwQsCiWxPJEIYK8qq7hItMxMK9bAwp@pscale_pw_nEKjjpm4FVkXoqwQsCiWxPJEIYK8qq7hItMxMK9bAwp/bachelor-thesis".into()
//     });

//     if url.is_empty() {
//         panic!("DATABASE_URL is empty");
//     }

//     let opts = Opts::from_url(&url).expect("DATABASE_URL invalid");

//     if opts
//         .db_name()
//         .expect("a database name is required")
//         .is_empty()
//     {
//         panic!("database name is empty");
//     }

//     print!("Database connection setup with: {}", url);

//     url
// }

fn get_db_url() -> String {
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "mysql://root:bachelor@127.0.0.1:3306/wasi-chat".into()
    });

    if url.is_empty() {
        panic!("DATABASE_URL is empty");
    }

    let opts = Opts::from_url(&url).expect("DATABASE_URL invalid");

    if opts
        .db_name()
        .expect("a database name is required")
        .is_empty()
    {
        panic!("database name is empty");
    }

    info!("Database connection setup with: {}", url);

    url
}

async fn handle_request(req: Request<Body>, pool: Pool) -> Result<Response<Body>, anyhow::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "The valid endpoints are /init /create_message /create_messages /update_message /messages /delete_message",
        ))),

        (&Method::POST, "/echo") => Ok(Response::new(req.into_body())),

        // CORS OPTIONS
        (&Method::OPTIONS, "/init") => Ok(response_ok(String::from(""))),
        (&Method::OPTIONS, "/create_message") => Ok(response_ok(String::from(""))),
        (&Method::OPTIONS, "/create_messages") => Ok(response_ok(String::from(""))),
        (&Method::OPTIONS, "/update_message") => Ok(response_ok(String::from(""))),
        (&Method::OPTIONS, "/delete_message") => Ok(response_ok(String::from(""))),
        (&Method::OPTIONS, "/messages") => Ok(response_ok(String::from(""))),
        
        (&Method::GET, "/init") => {
            info!("Initializing database");
            let mut conn = pool.get_conn().await.unwrap();
            
            "DROP TABLE IF EXISTS messages;".ignore(&mut conn).await?;
            "CREATE TABLE messages (id INT AUTO_INCREMENT PRIMARY KEY, ulid CHAR(26), text LONGTEXT, created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP);".ignore(&mut conn).await?;
            drop(conn);

            info!("Database initialized successfully");
            Ok(response_ok(json!({"status": true}).to_string()))
        }

        (&Method::POST, "/create_message") => {
            let mut conn = pool.get_conn().await.unwrap();

            let byte_stream = hyper::body::to_bytes(req).await?;
            let message: message::InsertMessage = serde_json::from_slice(&byte_stream).unwrap();

            "INSERT INTO messages (ulid, text) VALUES (:ulid, :text)"
                .with(params! {
                    "ulid" => Ulid::new().to_string(),
                    "text" => &message.text,
                })
                .ignore(&mut conn)
                .await?;

            info!("inserted message");

            drop(conn);
            Ok(response_ok(json!({"status": true}).to_string()))
        }

        (&Method::POST, "/create_messages") => {
            let mut conn = pool.get_conn().await.unwrap();

            let byte_stream = hyper::body::to_bytes(req).await?;
            let messages: Vec<message::InsertMessage> = serde_json::from_slice(&byte_stream).unwrap();
            info!("messages: {:?}", messages);
            "INSERT INTO messages (ulid, text) VALUES (:ulid, :text)"
                .with(messages.iter().map(|message| {
                    params! {
                        "ulid" => Ulid::new().to_string(),
                        "text" => &message.text,
                    }
                }))
                .batch(&mut conn)
                .await?;

            drop(conn);
            Ok(response_ok(json!({"status": true}).to_string()))
        }

        (&Method::POST, "/update_message") => {
            let mut conn = pool.get_conn().await.unwrap();

            let byte_stream = hyper::body::to_bytes(req).await?;
            let message: message::Message = serde_json::from_slice(&byte_stream).unwrap();

            "UPDATE messages SET text=:text WHERE id=:id"
                .with(params! {
                    "text" => &message.text,
                })
                .ignore(&mut conn)
                .await?;

            drop(conn);
            Ok(response_ok(json!({"status": true}).to_string()))
        }

        (&Method::GET, "/messages") => {
            let mut conn = pool.get_conn().await.unwrap();

            let messages = "SELECT * FROM messages"
                .with(())
                .map(&mut conn, |(id, ulid, text, created_at, updated_at)| {
                    message::Message::new(
                        id,
                        ulid,
                        text,
                        created_at,
                        updated_at,
                    )},
                ).await?;

            drop(conn);

            info!("total messages count: {}", messages.iter().count());

            Ok(response_ok(serde_json::to_string(&messages)?.to_string()))
        }        
        
        (&Method::GET, "/delete_message") => {
            let mut conn = pool.get_conn().await.unwrap();

            let params: HashMap<String, String> = req.uri().query().map(|v| {
                url::form_urlencoded::parse(v.as_bytes()).into_owned().collect()
            }).unwrap_or_else(HashMap::new);
            let id = params.get("id");

            "DELETE FROM messages WHERE id=:id"
                .with(params! { "id" => id, })
                .ignore(&mut conn)
                .await?;

            drop(conn);
            Ok(response_ok(json!({"status": true}).to_string()))
            // Ok(Response::new(Body::from("{\"status\":true}")))
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

// CORS headers
fn response_ok(body: String) -> Response<Body> {
    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "api,Keep-Alive,User-Agent,Content-Type")
        .body(Body::from(body.to_owned()))
        .unwrap()
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    CombinedLogger::init(vec![WriteLogger::new(simplelog::LevelFilter::Info, Config::default(), File::create("log.txt").unwrap())]).unwrap();

    info!("Connecting to MySQL...");
    let opts = Opts::from_url(get_db_url().as_str()).unwrap();
    
    let builder = OptsBuilder::from_opts(opts);
    // The connection pool will have a min of 5 and max of 10 connections.
    let constraints = PoolConstraints::new(5, 10).unwrap();
    let pool_opts = PoolOpts::default().with_constraints(constraints);
    let pool = Pool::new(builder.pool_opts(pool_opts));

    info!("Connected to MySQL to database");
    info!("Starting server...");

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let make_svc = make_service_fn(|_| {
        let pool = pool.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let pool = pool.clone();
                handle_request(req, pool)
            }))
        }
    });
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())

    /*
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(handle_request)).await {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
    */
}
