#!/bin/bash

scheme="http"
host="127.0.0.1:3000"

res=$(curl -s GET $scheme://$host/api/sessions)

# {"session_id":"01GVB8CPXF53XESB7R9E33QC8H","expires_at":"2023-03-13T15:52:54","created_at":"2023-03-12T15:52:54"}

# extract the session_id
echo "$res" | grep -oP '(?<="session_id":")[^"]*'
