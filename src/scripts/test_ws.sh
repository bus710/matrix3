#!/bin/sh

set -e

echo "WebSocket connection"
curl --location "127.0.0.1:8000/v1/ws" -X GET -v \
    -i \
    -N \
    -H "connection: upgrade" \
    -H "upgrade: websocket" \
    -H "Host: 127.0.0.1:8000" \
    -H "Origin: http://127.0.0.1:8000" \
    -H "sec-websocket-key: SGVsbG8sIHdvcmxkIQ==" \
    -H "sec-websocket-version: 13" \
