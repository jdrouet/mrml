#!/bin/bash

npm install -g mjml

for f in $(find resources/compare/success -name '*.mjml'); do
  mjml $f -o ${f%.*}.html
done
