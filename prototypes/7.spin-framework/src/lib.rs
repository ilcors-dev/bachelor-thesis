use anyhow::Result;
use serde_json::json;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn test(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    print!("{:?}", req.body());

    let j = json!({
        "status": true
    });

    Ok(http::Response::builder()
        .status(200)
        .body(Some(j.to_string().into()))?)
}
