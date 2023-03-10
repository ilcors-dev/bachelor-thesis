#!/bin/bash

pwd=$(pwd)

/bin/bash "$pwd"/truncator.sh
session=$(/bin/bash "$pwd"/seeders/session_seeder.sh)
chat=$(/bin/bash "$pwd"/seeders/chat_seeder.sh)

scheme="http"
host="127.0.0.1:3000"

echo "Creating a chat with name (201)"
curl -s -o /dev/null -w "%{http_code}" -X POST $scheme://$host/api/chats \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"$session"'' \
    -d '{ "name": "test" }'
echo

echo "Creating a chat with description (201)"
curl -s -o /dev/null -w "%{http_code}" -X POST $scheme://$host/api/chats \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"$session"'' \
    -d '{ "name": "test", "description": "test" }'
echo

echo "Creating a chat without description (201)"
curl -s -o /dev/null -w "%{http_code}" -X POST $scheme://$host/api/chats \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"$session"'' \
    -d '{ "name": "test" }'
echo

echo "Creating a chat without session (401)"
curl -s -o /dev/null -w "%{http_code}" -X POST $scheme://$host/api/chats \
    -H 'Content-Type: application/json' \
    -d '{ "name": "test" }'
echo

echo "Updating a chat name (200)"
curl -s -o /dev/null -w "%{http_code}" -X PUT $scheme://$host/api/chats/ \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"$session"'' \
    -d '{ "id": 1, "name": "update" }'
echo

echo "Updating a chat name and description (200)"
curl -s -o /dev/null -w "%{http_code}" -X PUT $scheme://$host/api/chats/ \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"$session"'' \
    -d '{ "id": 1, "name": "update", "description": "test" }'
echo

echo "Updating a chat without changing anything (400)"
curl -s -o /dev/null -w "%{http_code}" -X PUT $scheme://$host/api/chats/ \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"$session"'' \
    -d '{}'
echo

echo "Updating a chat without session (401)"
curl -s -o /dev/null -w "%{http_code}" -X PUT $scheme://$host/api/chats/ \
    -H 'Content-Type: application/json' \
    -d '{ "id": 1, "name": "update", "description": "test" }'
echo

echo "Fetching chats (200)"
curl -s -o /dev/null -w "%{http_code}" -X GET $scheme://$host/api/chats -H 'session_id: '"$session"''
echo

# can not update a chat that does not belong
echo "Updating a chat not belonging to current user (401)"
curl -v -X POST $scheme://$host/api/chats \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"$session"'' \
    -d '{ "name": "test" }' &>/dev/null
curl -s -o /dev/null -w "%{http_code}" -X PUT $scheme://$host/api/chats/ \
    -H 'Content-Type: application/json' \
    -H 'session_id: '"$(/bin/bash "$pwd"/sessions.sh -s)"'' \
    -d '{ "id": 1, "name": "update", "description": "test" }'
echo

echo "Deleting a chat (200)"
curl -s -o /dev/null -w "%{http_code}" -X DELETE $scheme://$host/api/chats/1 \
    -H 'session_id: '"$session"''
echo

echo "Deleting a chat not belongin to current user (401)"
curl -s -o /dev/null -w "%{http_code}" -X DELETE $scheme://$host/api/chats/1 \
    -H 'session_id: '"$(/bin/bash "$pwd"/sessions.sh -s)"''
echo
