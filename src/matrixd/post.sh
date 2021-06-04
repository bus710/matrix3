#!/bin/sh

set -e

echo "hello"

curl -v --location --request POST 'localhost:8080/v1/data/a' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "r0": [0, 0, 0],
    "r1": [0, 0, 0],
    "g0": [0, 0, 0],
    "g1": [0, 0, 0],
    "b0": [0, 0, 0],
    "b1": [0, 0, 0]
}'

echo "bye"