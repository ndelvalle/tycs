#!/usr/bin/env bash

day="$1"
year="${2:-2022}"

curl "https://adventofcode.com/$year/day/$day/input" -s \
  -H 'accept: text/html' \
  -H "cookie: session=$AOC_COOKIE;" \
  --compressed
