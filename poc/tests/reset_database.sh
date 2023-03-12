#!/bin/bash

# checks whether the database can be reset

scheme="http"
host="127.0.0.1:3000"

echo "Can reset database (200)"
r=$(curl -s -o /dev/null -w "%{http_code}" -X GET $scheme://$host/api/truncator \
    -H 'Content-Type: application/json')

[[ "$r" == "200" ]] && echo "OK" || echo "FAIL received $r"
