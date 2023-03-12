#!/bin/bash

# chats tests, depends on session

pwd=$(pwd)

# truncate the database before running the tests
/bin/bash "$pwd"/utils/truncator.sh
session=$(/bin/bash "$pwd"/seeders/session_seeder.sh)

echo "$session"

scheme="http"
host="127.0.0.1:3000"

echo "Signaling server the user is online (200)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X POST $scheme://$host/api/users \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"$session"'')
[[ "$r" == "200" ]] && echo "OK" || echo "FAIL received $r"

echo "Signaling server the user is online with invalid session (401)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X POST $scheme://$host/api/users \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"52325235235235"'')
[[ "$r" == "401" ]] && echo "OK" || echo "FAIL received $r"

echo "Fetching online users (200)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X GET $scheme://$host/api/users \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"$session"'')
[[ "$r" == "200" ]] && echo "OK" || echo "FAIL received $r"

echo "Fetching online users with invalid session (401)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X GET $scheme://$host/api/users \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"52325235235235"'')
[[ "$r" == "401" ]] && echo "OK" || echo "FAIL received $r"
