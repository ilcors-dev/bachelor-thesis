// create express app
const express = require("express");
const app = express();
app.use(express.json());

const mysql = require('mysql2');
const { ulid } = require("ulid");
const connection = mysql.createConnection({
    host: '127.0.0.1',
    user: 'root',
    password: 'bachelor',
    database: 'wasi-chat'
})

connection.connect()

app.listen(3000, () => {
    console.log("server started on port 3000");
});

// define a route handler for the default home page
app.get("/", (req, res) => {
    console.log("Hello World!")
    res.send("Hello World!");
});

app.post('/api/messages', (req, res) => {
    var text = req.body.text;

    console.log(text)

    connection.query(
        'INSERT INTO messages (ulid, text) VALUES (?, ?)', [ulid(), text],
        (error, results, fields) => {
            if (error) {
                throw error;
            }
        });

    console.log("inserted message");

    res.status(201).send();
});