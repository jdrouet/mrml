#!/bin/bash

grcov ./target/debug -t lcov --llvm --branch --ignore-not-existing -s . --token $CODECOV_TOKEN -o lcov.info
bash <(curl -s https://codecov.io/bash) -f lcov.info
