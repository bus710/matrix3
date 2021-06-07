#!/bin/sh

set -e

echo "WebSocket connection"
curl 127.0.0.1:8080/ws -X GET -v \
    -i \
    -N \
    -H "connection: upgrade" \
    -H "upgrade: websocket" \
    -H "Host: 127.0.0.1:8080" \
    -H "Origin: http://127.0.0.1:8080" \
    -H "sec-websocket-key: SGVsbG8sIHdvcmxkIQ==" \
    -H "sec-websocket-version: 13" \