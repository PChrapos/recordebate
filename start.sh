#!/bin/bash
service redis-server start
nodejs index.js &
./backend