#!/bin/sh

set -e

echo "Send random request"
curl --location '127.0.0.1:8000/v1/random' -X POST -s
