#!/bin/bash

docker-compose -f bench/docker-compose.yml up --build -d

cd bench

cargo run
