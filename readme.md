# MRML

[![CircleCI](https://circleci.com/gh/jdrouet/mrml.svg?style=shield)](https://app.circleci.com/pipelines/github/jdrouet/mrml)

## TODO

- Testing
  - [ ] compare properly the generated HTML
    - [x] not take in account empty class/style attributes
    - [ ] not care about orders of attributes
- CI
  - [ ] add code coverage
  - [ ] automatic deploy to crates.io
- Core
  - [x] expose the `to_html` method
  - [ ] add options to minify/not minify
- components
  - [ ] mjml
    - [x] without attributes
    - TBD
  - [ ] mj-head
    - [x] without attributes
    - TBD
  - [ ] mj-body
    - [x] without attributes
    - TBD
  - [ ] mj-section
    - [x] without attributes
    - [x] with background-color
    - [x] with background-url
