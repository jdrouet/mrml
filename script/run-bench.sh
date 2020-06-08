#!/bin/bash

docker-compose -f bench/docker-compose.yml build
docker-compose -f bench/docker-compose.yml up -d

cd bench

cargo run
