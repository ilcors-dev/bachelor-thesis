const express = require("express");
const mysql = require('mysql2');
const { ulid } = require("ulid");

// create express app
const app = express();
app.use(express.json());

// create mysql connection
const connection = mysql.createConnection({
    host: '127.0.0.1',
    user: 'root',
    password: 'bachelor',
    database: 'wasi-chat'
})

connection.connect()

// start express server on port 3000
app.listen(3000, () => {
    console.log("server started on port 3000");
});

// insert a new message into the database
app.post('/api/messages', (req, res) => {
    var text = req.body.text;

    connection.query(
        'INSERT INTO messages (ulid, text) VALUES (?, ?)', [ulid(), text],
        (error, results, fields) => {
            if (error) {
                throw error;
            }
        }
    );

    console.log("inserted message");

    res.status(201).send();
});

// read message by id
app.get('/api/messages/:id', (req, res) => {
    var id = req.params.id;

    connection.query(
        'SELECT * FROM messages WHERE id = ?', [id],
        (error, results, fields) => {
            if (error) {
                throw error;
            }

            res.send(results);
        }
    );
});

// get the last 10 messages from the database
app.get('/api/messages', (req, res) => {
    connection.query(
        'SELECT * FROM messages ORDER BY id DESC LIMIT 10',
        (error, results, fields) => {
            if (error) {
                throw error;
            }

            res.send(results);
        }
    );
});

// update a message
app.put('/api/messages', (req, res) => {
    var id = req.body.id;
    var text = req.body.text;

    connection.query(
        'UPDATE messages SET text = ? WHERE id = ?', [text, id],
        (error, results, fields) => {
            if (error) {
                throw error;
            }
        }
    );

    console.log("updated message");

    res.status(201).send();
});

// delete a message
app.delete('/api/messages', (req, res) => {
    var id = req.body.id;

    connection.query(
        'DELETE FROM messages WHERE id = ?', [id],
        (error, results, fields) => {
            if (error) {
                throw error;
            }
        }
    );

    console.log("deleted message");

    res.status(201).send();
});