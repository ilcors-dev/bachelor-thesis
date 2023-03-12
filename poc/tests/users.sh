#!/bin/bash

# chats tests, depends on session

pwd=$(pwd)

# truncate the database before running the tests
/bin/bash "$pwd"/truncator.sh
session=$(/bin/bash "$pwd"/seeders/session_seeder.sh)

echo "$session"

scheme="http"
host="127.0.0.1:3000"

echo "Signaling server the user is online"
r=$(curl -s -o /dev/null -w "%{http_code}" -X POST $scheme://$host/api/users \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"$session"'')
[[ "$r" == "201" ]] && echo "OK" || echo "FAIL received $r"
