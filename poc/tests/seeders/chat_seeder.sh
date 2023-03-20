#!/bin/bash

# creates a chat and returns the chat id and the session id

scheme="http"
host="127.0.0.1:3000"

if [[ -z "$1" ]]; then
    pwd=$(pwd)
else
    pwd="$1"
fi

session=$(/bin/bash "$pwd"/session_seeder.sh)

curl -s -o /dev/null -w "%{http_code}" -X POST $scheme://$host/api/chats \
    -H 'Content-Type: application/json' \
    -H 'Session-Id: '"$session"'' \
    -d '{ "name": "test" }' &>/dev/null

# return both the chat id and the session id
echo "1-$session"
