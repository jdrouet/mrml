# Changelog

All notable changes to this project will be documented in this file.

## [2.0.0-rc4] - 2023-08-17

### Bug Fixes

- Disable_comments is in camelCase
- Script elements should remain with open and close element (#320)

### Documentation

- Update links in readme
- Update code in example

### Features

- Bump mrml version
- Remove randomness when generating component ids (#308)
- Create wasm package (#312)
- Add mj-include feature (#316)
- Create multi include loader (#322)

### Miscellaneous Tasks

- Allow mastodon notification to fail
- Implement real tool for comparing html (#107)
- Move license files
- Update cargo.toml to publish
- Update cargo.toml to publish
- Bump to 2.0.0-rc4

### Refactor

- Put cows in tag implementation 🐮  (#111)
- Prepare for monotrepo
- Move back mrml-cli to repository
- Format code using rustfmt and create config
- Apply clippy suggestions
- Use a single MrmlParser structure with a visitor pattern (#317)

### Styling

- Apply clippy suggestions

### Testing

- Add node example with a test (#313)
- Add browser example (#315)
- Update node example to have test command
- Add test with disabling comments

### Build

- Bump quote from 1.0.28 to 1.0.29 (#101)
- Bump serde from 1.0.164 to 1.0.166 (#102)
- Bump thiserror from 1.0.40 to 1.0.41 (#103)
- Bump serde_json from 1.0.99 to 1.0.100 (#104)
- Bump colored from 2.0.0 to 2.0.4 (#110)
- Bump serde from 1.0.166 to 1.0.167 (#108)
- Bump serde from 1.0.167 to 1.0.168 (#112)
- Bump thiserror from 1.0.41 to 1.0.43 (#109)
- Bump proc-macro2 from 1.0.63 to 1.0.64 (#113)
- Bump serde_json from 1.0.100 to 1.0.102 (#114)
- Bump semver from 5.7.1 to 5.7.2 in /scripts (#115)
- Update dependabot configuration
- Update env_logger requirement in /packages/mrml-cli (#309)
- Bump thiserror from 1.0.43 to 1.0.44 (#305)
- Update clap to 4.3
- Update indexmap to 2.0

### Ci

- Move code-checking out of every workflow (#314)
- Report mrml-cli metric to codebench (#321)

### Doc

- Update readme

## [2.0.0-rc3] - 2023-06-27

### Features

- Add fonts option (#90)

### Build

- Bump serde from 1.0.158 to 1.0.159 (#69)
- Bump indexmap from 1.9.2 to 1.9.3 (#68)
- Bump mockito from 1.0.0 to 1.0.1 (#66)
- Bump proc-macro2 from 1.0.53 to 1.0.54 (#67)
- Bump serde_json from 1.0.94 to 1.0.95 (#71)
- Bump reqwest from 0.11.15 to 0.11.16 (#70)
- Bump mockito from 1.0.1 to 1.0.2 (#72)
- Bump proc-macro2 from 1.0.54 to 1.0.56 (#73)
- Bump serde from 1.0.159 to 1.0.160 (#74)
- Bump serde_json from 1.0.95 to 1.0.96 (#75)
- Bump h2 from 0.3.15 to 0.3.17 (#76)
- Bump reqwest from 0.11.16 to 0.11.17 (#77)
- Bump serde from 1.0.160 to 1.0.162 (#78)
- Bump quote from 1.0.26 to 1.0.27 (#79)
- Bump serde from 1.0.162 to 1.0.163 (#80)
- Bump proc-macro2 from 1.0.56 to 1.0.57 (#81)
- Bump reqwest from 0.11.17 to 0.11.18 (#82)
- Bump proc-macro2 from 1.0.57 to 1.0.58 (#83)
- Bump quote from 1.0.27 to 1.0.28 (#86)
- Bump proc-macro2 from 1.0.58 to 1.0.59 (#85)
- Bump criterion from 0.4.0 to 0.5.1 (#87)
- Bump url from 2.3.1 to 2.4.0 (#89)
- Bump serde from 1.0.163 to 1.0.164 (#91)
- Bump proc-macro2 from 1.0.59 to 1.0.60 (#92)
- Bump ureq from 2.6.2 to 2.7.0 (#93)
- Bump serde_json from 1.0.96 to 1.0.97 (#94)
- Bump mockito from 1.0.2 to 1.1.0 (#95)
- Bump ureq from 2.7.0 to 2.7.1 (#96)
- Bump proc-macro2 from 1.0.60 to 1.0.63 (#99)
- Bump serde_json from 1.0.97 to 1.0.99 (#98)

## [2.0.0-rc2] - 2023-03-25

### Bug Fixes

- Replace deprecated methods

### Documentation

- Add link to mjml

### Features

- Mark IncludeLoaderError cayse as Send and Sync (#52)

### Miscellaneous Tasks

- Enable the orderedmap feature by default

### Build

- Bump mockito from 0.32.0 to 0.32.1 (#40)
- Bump mockito from 0.32.1 to 0.32.2 (#41)
- Bump mockito from 0.32.2 to 0.32.3 (#42)
- Bump syn from 1.0.107 to 1.0.108 (#43)
- Bump syn from 1.0.108 to 1.0.109 (#44)
- Bump mockito from 0.32.3 to 0.32.4 (#46)
- Bump thiserror from 1.0.38 to 1.0.39 (#48)
- Bump serde_json from 1.0.93 to 1.0.94 (#47)
- Bump serde from 1.0.152 to 1.0.153 (#49)
- Bump serde from 1.0.153 to 1.0.154 (#50)
- Bump darling from 0.14.3 to 0.14.4 (#51)
- Bump quote from 1.0.23 to 1.0.25 (#54)
- Bump serde from 1.0.154 to 1.0.155 (#55)
- Bump mockito from 0.32.4 to 1.0.0 (#53)
- Bump quote from 1.0.25 to 1.0.26 (#58)
- Bump serde from 1.0.155 to 1.0.156 (#57)
- Bump thiserror from 1.0.39 to 1.0.40 (#60)
- Bump serde from 1.0.156 to 1.0.157 (#61)
- Bump reqwest from 0.11.14 to 0.11.15 (#62)
- Bump serde from 1.0.157 to 1.0.158 (#63)
- Bump proc-macro2 from 1.0.52 to 1.0.53 (#65)

## [2.0.0-rc1] - 2023-02-12

### Documentation

- Add link to python library

### Features

- First implementation of mj-include (#23)
- Create an include-loader to load from the filesystem (#32)
- Create an http-loader (#34)

### Miscellaneous Tasks

- Publish on mastodon after releasing a version
- Replace print boilerplate with a procedural macro (#10)
- Check features with cargo hack (#12)
- Create a macro to generate From and as_element (#13)
- Replace parse boilerplate with a proc macro (#11)
- Macro cleanup (#14)
- Implement a proc macro to handle json conversion (#18)
- Replace some existing code with macro for parsing (#20)
- Rename components using camel case (#21)
- Camelcase  (#22)
- Create a struct to simplify code (#24)
- Split in multiple modules (#25)
- Improve code coverage (#27)
- Update semantic commit checker (#33)
- Remove feature powerset for code checking
- Implement stderr for errors (#38)
- Rename and set versions
- Update cargo.toml files with required fields

### Build

- Bump proc-macro2 from 1.0.50 to 1.0.51 (#30)
- Bump serde_json from 1.0.91 to 1.0.92 (#29)
- Bump darling from 0.14.2 to 0.14.3 (#31)
- Add deps scope to semantic configuration
- Bump serde_json from 1.0.92 to 1.0.93 (#35)
- Bump mockito from 0.31 to 0.32

## [1.2.11] - 2023-01-11

### Bug Fixes

- Run and push coverage
- Upgrade typescript from 3.7.5 to 3.9.7
- Examples/mrml-browser/package.json & examples/mrml-browser/package-lock.json to reduce vulnerabilities
- Upgrade @types/react from 16.9.52 to 16.9.53
- Upgrade @types/react from 16.9.53 to 16.9.55
- Update mj-accordion and apply changes
- Update mj-carousel
- Update mj-navbar
- Update mj-social
- Update mj-button
- Update mj-section
- Update mj-column
- Update mj-divider
- Update mj-group
- Update mj-hero
- Update mj-image
- Update mj-raw
- Update mj-spacer
- Update mj-table
- Update mj-text
- Update mj-wrapper
- Update mj-attributes
- Update mj-breakpoint
- Update mj-font
- Update mj-style
- Update mj-title
- Fix linting errors
- Update coverage command
- Upgrade react-app-rewired from 2.1.6 to 2.1.7
- Use nightly tarpaulin image
- Apply clippy suggestions
- Apply clippy suggestions
- Update rand usage
- Satisfy clippy proposal
- Satisfy clippy
- Allow upper case acronym
- Satisfy clippy
- Satisfy clippy
- Format code
- Add serde to mrml-wasm
- Replace value attribute by width
- Add missing child for mj-body
- Add missing child for mj-head
- Add Cargo.lock to versionning
- Make sure mj-style components output style elements
- Keep style content order
- Add missing sort helper for print feature
- Update following clippy error (#176)
- Apply clippy's recommandations
- Apply clippy's recommandations
- Installation procedure with locked deps
- Mj-social-element can now have inner elements
- Mj-navbar-link can now have inner elements
- Align with mjml upgrades
- Simplify mj-wrapper renderer (#269)
- Handle all void elements
- Update dependabot directory

### Documentation

- Add more badges to readme
- Add section for mrml users
- Add sponsor section
- Add missing implementations section
- Update performance section
- Update the install section
- Add "what is using" section
- Add issue tracker badges
- Add mrml-ruby library
- Fix typos (#174)

### Features

- Add ability to specify origin url for mj-social icons
- Allow to pass social icon origin to cli
- Allow to render to html with options
- Update browser example to use options
- Give position for invalid format parser error
- Add generator id function in options
- Extract to core module
- Create parser for existing components
- Add more head components
- Add render prelude
- Move macros to prelude files
- Create buffer object
- Implement mj-section
- Implement mj-column and fix mj-body
- Implement mj-image
- Implement mj-divider
- Create parsable trait
- Implement mj-social
- Implement mj-navbar
- Implement mj-carousel
- Implement mj-accordion
- Add options for render
- Add tests for mj-navbar
- Generate random id for elements
- Add test on amario template
- Remove previous version
- Update templates
- Add serializer and deserializer for comment
- Add serializer and deserializer for text
- Add serializer and deserializer for mj-title
- Add serializer and deserializer for mj-preview
- Add serializer and deserializer for mj-divider
- Add serializer and deserializer for mj-font
- Add serializer and deserializer for mj-image
- Add serializer and deserializer for mj-breakpoint
- Add serializer and deserializer for mj-attributes-all
- Add serializer and deserializer for mj-attributes-class
- Add serializer and deserializer for mj-attributes-element
- Add serializer and deserializer for mj-attributes
- Add serializer and deserializer for mj-head
- Add serializer and deserializer for mj-accordion-title
- Add serializer and deserializer for mj-carousel-image
- Add serializer and deserializer for mj-carousel
- Add serializer and deserializer for mj-social-element
- Add serializer and deserializer for mj-social
- Add serializer and deserializer for mj-navbar-link
- Add serializer and deserializer for mj-navbar
- Add serializer and deserializer for node
- Add serializer and deserializer for mj_raw
- Add serializer and deserializer for mj_spacer
- Add serializer and deserializer for mj-accordion-text
- Add serializer and deserializer for mj-accordion-element
- Add serializer and deserializer for mj-accordion
- Add serializer and deserializer for others
- Omit attributes and children when empty
- Add validate, render, format-json and format-mjml subcommands
- Add toJson, toMjml and validate
- Implement missing mj-style component
- Add missing mj-table element
- Create typings for typescript
- Use stdin to read input
- Update to stick to mjml 4.9.3
- Read attributes from mj-attributes > mj-class
- Add lang to mjml element
- Render mj-raw in mj-head
- Use indexmap instead of hashmap (#216)
- Handle non closing elements (#276)
- Handle inner attributes (#9)

### Miscellaneous Tasks

- Release version 0.4.0
- Release version 0.4.0
- Move resources to root folder
- Release version 0.5.0
- Release version 0.4.0
- Release version 0.5.0
- Update mrml-browser dependencies
- Update mjml-bench dependencies
- Release version 1.0.0
- Release version 1.1.0
- Release version 1.2.0
- Release version 1.2.0
- Release version 1.2.1
- Release version 1.2.2
- Add semantics to lint commits
- Release version 1.2.1
- Release version 1.2.1
- Release version 1.2.1
- Release version 1.3.0
- Release version 1.2.2
- Release version 1.3.1
- Release version 1.2.2
- Add librerapay to funding file
- Add hyperfine test
- Release version 1.2.3
- Release version 1.3.2
- Release version 1.2.3
- Release version 1.2.4
- Release version 1.2.5
- Release version 1.2.6
- Release version 1.2.7
- Release version 1.2.8
- Release version 1.2.9
- Update dependencies
- Release version 1.2.10
- Release version 1.3.3
- Test and lint on github
- Apply clippy suggestions
- Remove gitlab ci config file
- Move code source to only keep mrml-core
- Compute code coverage
- Update repository url
- Automatic publishing
- Update resources using mjml (#1)
- Add dependabot config

### Refactor

- Remove unused is_raw function
- Remove node from components
- Split raw component
- Remove lifetimes
- Ensure options used
- Use breakpoint from options on header
- Move body and head components
- Remove unused function
- Update properties
- Remove macros
- Format code
- Remove Style element
- Remove hashmap from context
- Remove use of hashmap instead of attributes
- Update imports
- Simplify sort function
- Merge traits to simplify code
- Reformat html element
- Create proper errors
- Implement default
- Apply clippy cleaning proposals
- Remove useless getters
- Remove useless lifetimes
- Move templates at root folder and merge integration tests
- Create module for each element
- Split head parser code
- Split body parser code
- Split head renderer code
- Split body renderer code
- Use name for each element
- Split raw element
- Remove use less lifetime
- Use tag alias everywhere
- Remove useless code and comments
- Split mj-attributes
- Make sure feature combinations work

### Testing

- Add code coverage
- Increase mjml prelude coverage
- Increate coverage on Context struct
- Increase coverage on Spacing
- Only do a basic diff on CI
- Use rust nigthly dev image
- Run benchmark of core
- Merge integration tests and parse html before comparing
- Update template for mjml
- Update mj-body templates
- Check if rendering comments works
- Test use of is_raw
- Increate parse code coverage
- Increate parse code coverage
- Increate parse code coverage
- Increate parse code coverage
- Handle parsing with non expected element
- Test printing all components
- Increase coverage for parse and print

### Bench

- Create script to run benchmark

### Build

- Bump websocket-extensions in /example/mrml-wasm
- Bump lodash in /example/mjml-express
- Bump lodash from 4.17.15 to 4.17.19 in /example/mrml-wasm
- Bump elliptic from 6.5.2 to 6.5.3 in /example/mrml-wasm
- Bump http-proxy from 1.18.0 to 1.18.1 in /example/mrml-wasm
- Update bench path
- Split repo in packages
- Install dependabot
- Update html_parser requirement in /packages/library
- Update rand requirement in /packages/library
- Update getrandom requirement in /packages/wasm
- Use clippy image
- Bump lodash in /examples/mrml-browser
- Bump hosted-git-info in /examples/mrml-browser
- Bump @testing-library/react in /examples/mrml-browser
- Bump @types/react-dom in /examples/mrml-browser
- Add check linter step
- Bump url-parse in /examples/mrml-browser
- Bump xmlparser from 0.13.3 to 0.13.5 (#2)
- Bump indexmap from 1.8.0 to 1.9.2 (#4)
- Bump serde from 1.0.136 to 1.0.152 (#5)
- Bump serde_json from 1.0.79 to 1.0.91 (#6)
- Bump criterion from 0.3.5 to 0.4.0 (#3)

### Ci

- Run linter
- Run clippy
- Create travis file
- Run every script
- Deploy on tag
- Stop running coverage twice
- Move to gitlab
- Publish bench to codebench
- Remove examples from dependabot
- Disable coverage

### Cleanup

- Remove commented code

### Cli

- Create candid main.rs
- Build options from args

### Doc

- Update readme
- Add more components
- Fix check
- Create license file
- Update readme and cargo.toml
- Add howto section to readme
- Replace circleci badge by travis badge
- Update bench section
- Add link to readme and license
- Add contributing and code of conduct
- Add code climate badge
- Add funding informations
- Add github social representation
- Update readme
- Mention mrml-cli

### Example

- Create a wasm example
- Create actix server
- Update to use multipart
- Fix mrml-actix example
- Move mjml bench to proper folder

### Init

- First commit

### Lib

- Add function to export to email structure

### Library

- Support -noshare suffix in social element
- Remove bool_to_option feature

### Mj-accordion

- Update doc
- Handle basic
- Handle other attributes
- Handle icon attributes
- Handle font and padding

### Mj-attributes

- Pass header at parse time
- Generate default attributes
- Handle mj-class

### Mj-body

- Handle background color
- Make sure container width is propagated
- Make sure width is given with unit
- Ensure css-class is handled
- Move style to separate function
- Use tag builder

### Mj-breakpoint

- Handle basic
- Make sure it handles the options

### Mj-button

- Update doc
- Handle basic
- Handle example
- Handle align
- Handle background-color
- Handle border
- Handle border-radius
- Handle color
- Handle container-background-color
- Handle css-class
- Handle font-family
- Handle font-size
- Handle font-style
- Handle font-weight
- Handle height
- Handle href
- Handle inner-padding
- Handle line-height
- Handle padding
- Handle rel
- Handle text-decoration
- Handle text-transform
- Handle vertical-align
- Handle width
- Move style to separate function
- Use tag builder

### Mj-carousel

- Update doc
- Handle basic
- Handle align, border-radius and css-class
- Handle icon related attributes
- Handle tb-*
- Handle thumbnails hidden

### Mj-column

- Handle the basic
- Handle background-color
- Handle borders
- Handle border radius
- Handle css-class
- Handle paddings
- Handle vertical-align
- Handle width
- Move style to separate function
- Use tag builder
- Use tag builder

### Mj-divider

- Update readme
- Implement basic behavior
- Handle border attributes
- Handle container-background-color
- Handle css-class
- Handle padding
- Handle width
- Use tag builder

### Mj-font

- Handle basic

### Mj-group

- Update readme
- Handle without attributes
- Handle background-color
- Handle css-class
- Handle direction
- Handle vertical-align
- Handle width
- Use tag builder

### Mj-head

- Use tag builder

### Mj-hero

- Update readme
- Handle without attributes
- Handle background-color
- Handle background-height
- Handle background-position
- Handle background-url
- Handle background-width
- Handle css-class
- Handle height
- Handle mode and paddings
- Handle vertical-align
- Handle width
- Use tag builder

### Mj-image

- Handle base
- Handle align
- Handle border
- Handle border-radius
- Handle container-background-color
- Handle css-class
- Handle fluid-on-mobile
- Handle height
- Handle href
- Handle padding
- Handle rel
- Handle target
- Handle title
- Move style to separate function
- Use tag builder

### Mj-navbar

- Update readme
- Handle with base-url and hamburger
- Handle with align and css-class
- Handle ico-*
- Use tag builder

### Mj-preview

- Handle component

### Mj-raw

- Implement component
- Use raw element

### Mj-section

- Add background url options
- Handle borders
- Handle border-radius
- Handle css-class
- Handle direction
- Handle full-width
- Handle padding
- Handle text-align
- Move style to separate function
- Use tag builder

### Mj-social

- Update readme
- Handle without attributes
- Handle align
- Handle border-radius
- Handle color
- Handle class
- Handle container-background-color
- Handle font-family
- Handle font-size, font-style and font-weight
- Handle icon-height, icon-padding and icon-size
- Handle line-height and text-decoration
- Handle inner-padding, padding, padding-else and text-padding
- Handle mode
- Move in a dedicated folder
- Use tag builder

### Mj-social-element

- Use tag builder

### Mj-spacer

- Update readme
- Handle all attributes
- Use tag builder

### Mj-style

- Update readme
- Handle basic
- Increase coverage

### Mj-table

- Update readme
- Handle basic
- With text attributes
- With table attributes
- Handle other attributes
- Use tag builder

### Mj-text

- Update doc
- Handle basic
- Handle doc example
- Handle color
- Handle font-family
- Handle font-size
- Handle font-style
- Handle font-weight
- Handle line-height
- Handle letter-spacing
- Handle height
- Handle text-decoration
- Handle text-transform
- Handle align
- Handle container-background-color
- Handle padding
- Handle css-class
- Move style to separate function
- Use tag builder

### Mj-title

- Handle basic

### Mj-wrapper

- Update readme
- Handle without attributes
- Handle background-*
- Handle border-*
- Handle padding-*
- Handle css-class, direction, full-width and text-align
- Use tag builder

### Parser

- Disable options

### Raw

- Use tag builder

### Refacto

- Implement tag builder
- Parse using node reference
- Split html util
- Generate random ids for carousel and navbar
- Create default attribute map for each component
- Use lazy-static to create default attributes
- Create parser structure
- Parse directly the elements
- Move children element to submodule
- Move body children to submodule
- Avoid using children as vec
- Move MJBodyChild to dyn BodyComponent
- Use generic enum for children
- Remove unused code
- Update error handling
- Use pointer for options
- Remove Clone from elements
- Use BodyChild instead of MJBodyChild
- Clean generic structs
- Remove useless code
- Use sort_by_key in header
- Remove usuned code
- Use default string
- Use iterators
- Split mjml element
- Use default as constructor for MJBody
- Change single length string to char
- Please clippy
- Parse even commented data
- Rename packages folder
- Use strings to print elements
- Simplify print trait
- Split into features
- Use macro for standard serializers
- Use macro for standard serializers
- Use macro for standard deserializers
- Use macro to print
- Use macro for comment deserializer
- Use macro for json serializing

### Release

- 0.1.0
- 0.1.1
- 0.1.2
- 0.1.3

### Version

- 0.2.0
- 0.3.0
- 0.3.1
- Bump to 0.3.3

### Wasm

- Create npm package
- Make sure running tests
- Upgrade to version 0.3.4
- Run on node and in browser
- Release version 0.3.6

<!-- generated by git-cliff -->
