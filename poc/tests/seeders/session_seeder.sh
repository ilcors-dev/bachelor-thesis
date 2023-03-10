#!/bin/bash

scheme="http"
host="127.0.0.1:3000"

res=$(curl -s GET $scheme://$host/api/sessions)

echo "$res"
