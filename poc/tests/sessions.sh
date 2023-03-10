#!/bin/bash

scheme="http"
host="127.0.0.1:3000"

echo "Creating a session (201)"
curl -s -o /dev/null -w "%{http_code}" GET $scheme://$host/api/sessions
