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
  - [ ] clean by removing consecutive conditions
- components
  - [ ] mjml
    - [x] without attributes
    - TBD
  - [ ] mj-head
    - [x] without attributes
    - TBD
  - [x] mj-body
    - [x] without attributes
    - [x] with background-color
    - [x] with css-class
    - [x] with width
  - [ ] mj-section
    - [x] without attributes
    - [x] with background-color
    - [x] with background-repeat (default: repeat)
    - [x] with background-size (default: auto)
    - [x] with background-url
    - [x] with border, border-(top|right|bottom|left)
    - [x] with border-radius
    - [ ] with css-class
    - [ ] with direction (default: ltr)
    - [ ] with full-width
    - [ ] with padding (default: 20px 0)
    - [ ] with padding-(top|right|bottom|left)
    - [ ] with text-align
  - [ ] mj-column
    - [ ] without attributes
    - [ ] with background-color
    - [ ] with border, border-(top|right|bottom|left)
    - [ ] with border-radius
    - [ ] with css-class
    - [ ] with padding, padding-(top|right|bottom|left)
    - [ ] with vertical-align
    - [ ] with width (default: (100 / number of non-raw elements in section)%)
