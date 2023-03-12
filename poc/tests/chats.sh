#!/bin/bash

# chats tests, depends on session

pwd=$(pwd)

# truncate the database before running the tests
/bin/bash "$pwd"/utils/truncator.sh
session=$(/bin/bash "$pwd"/seeders/session_seeder.sh)

echo "$session"

scheme="http"
host="127.0.0.1:3000"

echo "Creating a chat with name (201)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X POST $scheme://$host/api/chats \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"$session"'' \
    -d '{ "name": "test" }')
[[ "$r" == "201" ]] && echo "OK" || echo "FAIL received $r"

echo "Creating a chat with description (201)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X POST $scheme://$host/api/chats \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"$session"'' \
    -d '{ "name": "test", "description": "test" }')
[[ "$r" == "201" ]] && echo "OK" || echo "FAIL received $r"

echo "Creating a chat without description (201)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X POST $scheme://$host/api/chats \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"$session"'' \
    -d '{ "name": "test" }')
[[ "$r" == "201" ]] && echo "OK" || echo "FAIL received $r"

echo "Creating a chat without session (401)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X POST $scheme://$host/api/chats \
    -H 'Content-Type: application/json' \
    -d '{ "name": "test" }')
[[ "$r" == "401" ]] && echo "OK" || echo "FAIL received $r"

echo "Updating a chat name (200)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X PUT $scheme://$host/api/chats/ \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"$session"'' \
    -d '{ "id": 1, "name": "update" }')
[[ "$r" == "200" ]] && echo "OK" || echo "FAIL received $r"

echo "Updating a chat name and description (200)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X PUT $scheme://$host/api/chats/ \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"$session"'' \
    -d '{ "id": 1, "name": "update", "description": "test" }')
[[ "$r" == "200" ]] && echo "OK" || echo "FAIL received $r"

echo "Updating a chat without changing anything (400)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X PUT $scheme://$host/api/chats/ \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"$session"'' \
    -d '{}')
[[ "$r" == "400" ]] && echo "OK" || echo "FAIL received $r"

echo "Updating a chat without session (401)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X PUT $scheme://$host/api/chats/ \
    -H 'Content-Type: application/json' \
    -d '{ "id": 1, "name": "update", "description": "test" }')
[[ "$r" == "401" ]] && echo "OK" || echo "FAIL received $r"

echo "Fetching chats (200)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X GET $scheme://$host/api/chats -H 'session_id: '"$session"'')
[[ "$r" == "200" ]] && echo "OK" || echo "FAIL received $r"

# can not update a chat that does not belong
echo "Updating a chat not belonging to current user (401)"
curl -v -X POST $scheme://$host/api/chats \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"$session"'' \
    -d '{ "name": "test" }' &>/dev/null
r=$(curl -s -o /dev/null -w "%{http_code}" -X PUT $scheme://$host/api/chats/ \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"$(/bin/bash $pwd/seeders/session_seeder.sh -s)"'' \
    -d '{ "id": 1, "name": "update", "description": "test" }')
[[ "$r" == "401" ]] && echo "OK" || echo "FAIL received $r"

echo "Deleting a chat (200)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE $scheme://$host/api/chats/1 \
    -H 'session_id: '"$session"'')
[[ "$r" == "200" ]] && echo "OK" || echo "FAIL received $r"

echo "Deleting a chat not belonging to current user (401)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE $scheme://$host/api/chats/1 \
    -H 'session_id: '"$(/bin/bash $pwd/seeders/session_seeder.sh -s)"'')
[[ "$r" == "401" ]] && echo "OK" || echo "FAIL received $r"
