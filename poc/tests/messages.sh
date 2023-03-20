#!/bin/bash

# messages tests, depends on session and chat

pwd=$(pwd)

# truncate the database before running the tests
/bin/bash "$pwd"/utils/truncator.sh
chat_and_session=$(/bin/bash "$pwd"/seeders/chat_seeder.sh "$pwd"/seeders)
chat=$(echo "$chat_and_session" | cut -d '-' -f 1)
session=$(echo "$chat_and_session" | cut -d '-' -f 2)

scheme="http"
host="127.0.0.1:3000"

echo "Creating a message in chat (201)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X POST $scheme://$host/api/messages \
    -H 'Content-Type: application/json' \
    -H 'Session-Id: '"$session"'' \
    -d '{ "chat_id": '"$chat"', "text": "test" }')
[[ "$r" == "201" ]] && echo "OK" || echo "FAIL received $r"

echo "Creating a message in chat without session (401)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X POST $scheme://$host/api/messages \
    -H 'Content-Type: application/json' \
    -d '{ "chat_id": '"$chat"', "text": "test" }')
[[ "$r" == "401" ]] && echo "OK" || echo "FAIL received $r"

echo "Creating a message in chat without text (400)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X POST $scheme://$host/api/messages \
    -H 'Content-Type: application/json' \
    -H 'Session-Id: '"$session"'' \
    -d '{ "chat_id": '"$chat"' }')
[[ "$r" == "400" ]] && echo "OK" || echo "FAIL received $r"

echo "Creating a message in chat without chat_id (400)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X POST $scheme://$host/api/messages \
    -H 'Content-Type: application/json' \
    -H 'Session-Id: '"$session"'' \
    -d '{ "text": "test" }')
[[ "$r" == "400" ]] && echo "OK" || echo "FAIL received $r"

echo "Updating a message (200)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X PUT $scheme://$host/api/messages/ \
    -H 'Content-Type: application/json' \
    -H 'Session-Id: '"$session"'' \
    -d '{ "id": 1, "text": "update" }')
[[ "$r" == "200" ]] && echo "OK" || echo "FAIL received $r"

echo "Updating a message without session (401)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X PUT $scheme://$host/api/messages/ \
    -H 'Content-Type: application/json' \
    -d '{ "id": 1, "text": "update" }')
[[ "$r" == "401" ]] && echo "OK" || echo "FAIL received $r"

echo "Updating a message without id (400)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X PUT $scheme://$host/api/messages/ \
    -H 'Content-Type: application/json' \
    -H 'Session-Id: '"$session"'' \
    -d '{ "text": "update" }')
[[ "$r" == "400" ]] && echo "OK" || echo "FAIL received $r"

echo "Updating a message without text (400)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X PUT $scheme://$host/api/messages/ \
    -H 'Content-Type: application/json' \
    -H 'Session-Id: '"$session"'' \
    -d '{ "id": 1 }')
[[ "$r" == "400" ]] && echo "OK" || echo "FAIL received $r"

echo "Updating a message of another user (401)"
curl -s -o /dev/null -w "%{http_code}" -X POST $scheme://$host/api/messages \
    -H 'Content-Type: application/json' \
    -H 'Session-Id: '"$session"'' \
    -d '{ "chat_id": '"$chat"', "text": "test" }' &>/dev/null
r=$(curl -s -o /dev/null -w "%{http_code}" -X PUT $scheme://$host/api/messages/ \
    -H 'Content-Type: application/json' \
    -H 'Session-Id: '"$(/bin/bash $pwd/seeders/session_seeder.sh -s)"'' \
    -d '{ "id": 2, "text": "update" }')
[[ "$r" == "401" ]] && echo "OK" || echo "FAIL received $r"

echo "Deleting a message (200)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE $scheme://$host/api/messages/1 \
    -H 'Session-Id: '"$session"'')
[[ "$r" == "200" ]] && echo "OK" || echo "FAIL received $r"

echo "Deleting a message without session (401)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE $scheme://$host/api/messages/1)
[[ "$r" == "401" ]] && echo "OK" || echo "FAIL received $r"

echo "Deleting a message that does not exist (401)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE $scheme://$host/api/messages/12432423 \
    -H 'Session-Id: '"$session"'')
[[ "$r" == "401" ]] && echo "OK" || echo "FAIL received $r"

echo "Deleting a message of another user (401)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE $scheme://$host/api/messages/1 \
    -H 'Session-Id: '"$(/bin/bash $pwd/seeders/session_seeder.sh -s)"'')
[[ "$r" == "401" ]] && echo "OK" || echo "FAIL received $r"

echo "Get last messages of chat (200)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X GET $scheme://$host/api/messages?chat_id=1 \
    -H 'Session-Id: '"$session"'')
[[ "$r" == "200" ]] && echo "OK" || echo "FAIL received $r"

echo "Get last messages of chat starting from a certain message id (200)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X GET "$scheme://$host/api/messages?chat_id=1&fetch_from_message_id=2" \
    -H 'Session-Id: '"$session"'')
[[ "$r" == "200" ]] && echo "OK" || echo "FAIL received $r"

echo "Get last 10 messages of chat without session (401)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X GET $scheme://$host/api/messages?chat_id=1)
[[ "$r" == "401" ]] && echo "OK" || echo "FAIL received $r"
