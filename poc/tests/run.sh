#!/bin/bash

# runs all tests

pwd=$(pwd)

/bin/bash "$pwd"/reset_database.sh
echo "----------------------"
/bin/bash "$pwd"/users.sh
echo "----------------------"
/bin/bash "$pwd"/chats.sh
echo "----------------------"
/bin/bash "$pwd"/messages.sh
echo "----------------------"
/bin/bash "$pwd"/sessions.sh
