#!/bin/bash

scheme="http"
host="127.0.0.1:3000"

echo "Can create a session (201)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X GET $scheme://$host/api/sessions \
    -H 'Content-Type: application/json')

[[ "$r" == "201" ]] && echo "OK" || echo "FAIL received $r"
