#!/bin/bash

scheme="http"
host="127.0.0.1:3000"

pwd=$(pwd)
session=$(/bin/bash "$pwd"/session_seeder.sh -s)

curl -s -o /dev/null -w "%{http_code}" -X POST $scheme://$host/api/chats \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"$session"'' \
    -d '{ "name": "test" }'

echo 1
