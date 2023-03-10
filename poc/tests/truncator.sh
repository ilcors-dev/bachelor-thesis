#!/bin/bash

# doing it via the application because it may become useful during development
# + the database is running in a docker container, so we can't access it directly easily
curl -X GET $scheme://$host/api/truncator &>/dev/null

echo "truncated all tables from database!"
echo
