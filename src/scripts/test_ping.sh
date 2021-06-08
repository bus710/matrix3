#!/bin/sh

set -e

echo "Send ping request"
curl --location '127.0.0.1:8080/v1/ping' -X POST -s
