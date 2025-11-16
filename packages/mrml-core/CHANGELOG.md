# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [5.1.0](https://github.com/jdrouet/mrml/compare/mrml-v5.0.0...mrml-v5.1.0) - 2025-11-16

### Added

- local-loader path should not always include protocol ([#555](https://github.com/jdrouet/mrml/pull/555))
- enabling conditional comments ([#533](https://github.com/jdrouet/mrml/pull/533))

### Fixed

- mj preview with comments ([#580](https://github.com/jdrouet/mrml/pull/580))
- *(mrml-core)* mj-divider align property with values left/right take no effect ([#540](https://github.com/jdrouet/mrml/pull/540))
- update resources and adjust conditional comments ([#545](https://github.com/jdrouet/mrml/pull/545))

### Other

- remove deprecated type alias
- update readme with tested code ([#556](https://github.com/jdrouet/mrml/pull/556))
- fix clippy lint
- use dtolnay/rust-toolchain ([#542](https://github.com/jdrouet/mrml/pull/542))
- *(deps)* bump all dependencies ([#541](https://github.com/jdrouet/mrml/pull/541))

## [5.0.0](https://github.com/jdrouet/mrml/compare/mrml-v4.0.1...mrml-v5.0.0) - 2025-03-01

### Added

- *(mrml-core)* add x to the supported social elements (#469)
- improve error handling (#452)
- parsing warnings and improve errors (#451)

### Fixed

- *(mrml-core)* add missing attributes in mj-accordion-text and mj-button (#513)
- apply clippy suggestions (#495)
- *(mrml-core)* handle duplicate border in mj-column (#479)
- *(mj-core)* mj-text is an ending tag (#478)
- *(mrml-core)* comments in mrml component should be ignored (#477)

### Other

- format code
- *(deps)* bump htmlparser to 0.2.1 (#489)
- use package attributes from workspace (#488)
- change visibility of modules (#487)
- replace xmlparser by htmlparser (#485)
- *(mrml-core)* remove unwraps in code (#482)
- *(deps)* upgrade multiple dependencies (#476)
- *(mrml-core)* create alias for attributes map
- apply format changes
- rename json traits to json prefix
- stop returning impl (#449)
- *(mrml-core)* base all component on the same struct (#448)
- *(mrml-core)* introduce proper errors for size parsing (#447)
- remove to_owned call
- make size, pixel, percent copy (#445)

## [4.0.1](https://github.com/jdrouet/mrml/compare/mrml-v4.0.0...mrml-v4.0.1) - 2024-06-30

### Fixed
- keep whitespaces when need ([#442](https://github.com/jdrouet/mrml/pull/442))
- *(mrml-core)* Fix typo in cfg-features ([#440](https://github.com/jdrouet/mrml/pull/440))

### Other
- *(deps)* Update rustc-hash requirement in /packages/mrml-core ([#436](https://github.com/jdrouet/mrml/pull/436))
- *(mrml-core)* make MjFontAttributes fields public ([#433](https://github.com/jdrouet/mrml/pull/433))

## [4.0.0](https://github.com/jdrouet/mrml/compare/mrml-v3.1.5...mrml-v4.0.0) - 2024-06-13

### Added
- *(mrml-core)* allow to have comments at root level ([#414](https://github.com/jdrouet/mrml/pull/414))

### Fixed
- *(mrml-core)* remove unreachable code from doc

### Other
- *(mrml-core)* make MjAttributes children public ([#429](https://github.com/jdrouet/mrml/pull/429))
- *(mrml-core)* remove common macro
- *(deps)* Update itertools requirement from 0.12.1 to 0.13.0 in /packages/mrml-core ([#425](https://github.com/jdrouet/mrml/pull/425))
- *(mrml-core)* remove useless ToString
- *(mrml-core)* avoid calling self.attributes in loops
- *(mrml-core)* use fmt::Write in render buffer
- *(mrml-core)* move MjAccordionElementChild
- *(mrml-core)* remove unused code
- *(mrml-core)* avoid using dyn dispatch
- *(mrml-core)* reimplement print with a printer ([#423](https://github.com/jdrouet/mrml/pull/423))
- *(mrml-core)* only use indexmap
- *(mrml-core)* update spacing implementation
- *(mrml-core)* spacing should only handle pixels ([#422](https://github.com/jdrouet/mrml/pull/422))
- *(mrml-core)* make SocialNetwork use static str
- *(mrml-core)* make extra use &str
- *(mrml-core)* avoid closing when calling attribute
- *(mrml-core)* simplify lifetimes
- *(mrml-core)* move siblings and index to renderer
- *(mrml-core)* move container_width to renderer
- *(mrml-core)* use common renderer
- *(mrml-core)* move buffer and header in cursor
- *(mrml-core)* wrap render options and header in context
- *(mrml-core)* split header to avoir using Cell
- *(mrml-core)* add lifetime to tag
- *(mrml-core)* style are now built with Cow<'static, str>
- *(mrml-core)* remove unused code
- *(mrml-core)* remove extra panic info
- *(mrml-core)* move conditional to buffer
- *(mrml-core)* make RenderBuffer a struct
- *(mrml-core)* render with buffer
- *(mrml-core)* move tag to render prelude
- *(mrml-core)* make default_attribute return static str
- *(mrml-core)* return args as &str
- *(mrml-core)* styles are stored in Set<Cow<'static, str>> ([#420](https://github.com/jdrouet/mrml/pull/420))
- *(deps)* Bump rustls from 1.0.0-alpha.46 to 1.0.0-alpha.55

## [3.1.5](https://github.com/jdrouet/mrml/compare/mrml-v3.1.4...mrml-v3.1.5) - 2024-04-13

### Fixed
- *(mrml-core)* only void element should self close

## [3.1.4](https://github.com/jdrouet/mrml/compare/mrml-v3.1.3...mrml-v3.1.4) - 2024-04-04

### Fixed
- *(mrml-core)* make sure <br> can be parsed in mj-raw

## [3.1.3](https://github.com/jdrouet/mrml/compare/mrml-v3.1.2...mrml-v3.1.3) - 2024-03-21

### Other
- *(deps)* Update reqwest requirement in /packages/mrml-core ([#395](https://github.com/jdrouet/mrml/pull/395))

## [3.1.2](https://github.com/jdrouet/mrml/compare/mrml-v3.1.1...mrml-v3.1.2) - 2024-03-21

### Other
- switch license to MIT ([#388](https://github.com/jdrouet/mrml/pull/388))

## [3.1.1](https://github.com/jdrouet/mrml/compare/mrml-v3.1.0...mrml-v3.1.1) - 2024-03-19

### Other
- add python usage
- update readme and documentation
- *(mrml-core)* move everythin in Arc ([#392](https://github.com/jdrouet/mrml/pull/392))
- *(mrml-core)* keep public build_attributes_all
- *(mrml-core)* make sure mj-head children are considered in order ([#390](https://github.com/jdrouet/mrml/pull/390))
- make sure chaining include and mj-attributes are taken in order ([#389](https://github.com/jdrouet/mrml/pull/389))

## [3.1.0](https://github.com/jdrouet/mrml/compare/mrml-v3.0.4...mrml-v3.1.0) - 2024-03-15

### Added
- make sure mj-style content is trimed

### Fixed
- *(mrml-core)* make sure mj-include type css are used

## [3.0.4](https://github.com/jdrouet/mrml/compare/mrml-v3.0.3...mrml-v3.0.4) - 2024-03-04

### Fixed
- *(mrml-core)* ensure fonts are rendered once ([#383](https://github.com/jdrouet/mrml/pull/383))

## [3.0.3](https://github.com/jdrouet/mrml/compare/mrml-v3.0.2...mrml-v3.0.3) - 2024-02-28

### Fixed
- *(mrml-core)* apply mj-attributes inside mj-include tags ([#378](https://github.com/jdrouet/mrml/pull/378))

## [3.0.2](https://github.com/jdrouet/mrml/compare/mrml-v3.0.1...mrml-v3.0.2) - 2024-02-24

### Fixed
- *(mrml-core)* add missing comment parser for mj-head ([#374](https://github.com/jdrouet/mrml/pull/374))

## [3.0.1](https://github.com/jdrouet/mrml/compare/mrml-v3.0.0...mrml-v3.0.1) - 2024-02-10

### Fixed
- *(mrml-core)* update template and fix mj-section with comment ([#372](https://github.com/jdrouet/mrml/pull/372))
- *(mrml-core)* alias remove to shift_remove when using orderedmap ([#371](https://github.com/jdrouet/mrml/pull/371))

### Other
- *(deps)* Bump h2 from 0.3.21 to 0.3.24 ([#365](https://github.com/jdrouet/mrml/pull/365))

## [3.0.0](https://github.com/jdrouet/mrml/compare/mrml-v2.1.1...mrml-v3.0.0) - 2023-12-19

### Fixed
- *(mrml-core)* make sure we can parse <mj-font></mj-font> ([#359](https://github.com/jdrouet/mrml/pull/359))
- *(mrml-core)* remove Sync and Send from include loader ([#357](https://github.com/jdrouet/mrml/pull/357))

## [2.1.1](https://github.com/jdrouet/mrml/compare/mrml-v2.1.0...mrml-v2.1.1) - 2023-12-07

### Other
- *(mrml-core)* rename Options to RenderOptions ([#352](https://github.com/jdrouet/mrml/pull/352))

## [2.1.0](https://github.com/jdrouet/mrml/compare/mrml-v2.0.0...mrml-v2.1.0) - 2023-12-04

### Added
- *(mrml-core)* make the include_loader be Send and Sync ([#347](https://github.com/jdrouet/mrml/pull/347))
- handle properly async mj-include ([#346](https://github.com/jdrouet/mrml/pull/346))
- implement an async parser ([#338](https://github.com/jdrouet/mrml/pull/338))
- *(mrml-core)* create multi include loader ([#322](https://github.com/jdrouet/mrml/pull/322))
- *(mrml-wasm)* add mj-include feature ([#316](https://github.com/jdrouet/mrml/pull/316))
- *(mjml-core)* remove randomness when generating component ids ([#308](https://github.com/jdrouet/mrml/pull/308))
- *(mrml-core)* handle non closing elements ([#276](https://github.com/jdrouet/mrml/pull/276))
- use indexmap instead of hashmap ([#216](https://github.com/jdrouet/mrml/pull/216))
- *(mrml-core)* render mj-raw in mj-head
- *(core)* add lang to mjml element
- *(mrml-core)* read attributes from mj-attributes > mj-class
- *(mrml-core)* update to stick to mjml 4.9.3
- *(core)* add missing mj-table element
- *(mj-style)* implement missing mj-style component
- *(json)* omit attributes and children when empty
- *(json)* add serializer and deserializer for others
- *(json)* add serializer and deserializer for mj-accordion
- *(json)* add serializer and deserializer for mj-accordion-element
- *(json)* add serializer and deserializer for mj-accordion-text
- *(json)* add serializer and deserializer for mj_spacer
- *(json)* add serializer and deserializer for mj_raw
- *(json)* add serializer and deserializer for node
- *(json)* add serializer and deserializer for mj-navbar
- *(json)* add serializer and deserializer for mj-navbar-link
- *(json)* add serializer and deserializer for mj-social
- *(json)* add serializer and deserializer for mj-social-element
- *(json)* add serializer and deserializer for mj-carousel
- *(json)* add serializer and deserializer for mj-carousel-image
- *(json)* add serializer and deserializer for mj-accordion-title
- *(json)* add serializer and deserializer for mj-head
- *(json)* add serializer and deserializer for mj-attributes
- *(json)* add serializer and deserializer for mj-attributes-element
- *(json)* add serializer and deserializer for mj-attributes-class
- *(json)* add serializer and deserializer for mj-attributes-all
- *(json)* add serializer and deserializer for mj-breakpoint
- *(json)* add serializer and deserializer for mj-image
- *(json)* add serializer and deserializer for mj-font
- *(json)* add serializer and deserializer for mj-divider
- *(json)* add serializer and deserializer for mj-preview
- *(json)* add serializer and deserializer for mj-title
- *(json)* add serializer and deserializer for text
- *(json)* add serializer and deserializer for comment
- *(core)* update templates

### Fixed
- *(mrml-core)* script elements should remain with open and close element ([#320](https://github.com/jdrouet/mrml/pull/320))
- *(mrml-core)* handle all void elements
- *(mrml-core)* simplify mj-wrapper renderer ([#269](https://github.com/jdrouet/mrml/pull/269))
- *(mrml-core)* align with mjml upgrades
- *(mrml-core)* mj-navbar-link can now have inner elements
- *(mrml-core)* mj-social-element can now have inner elements
- *(core)* apply clippy's recommandations
- *(mrml-core)* update following clippy error ([#176](https://github.com/jdrouet/mrml/pull/176))
- *(mrml-core)* add missing sort helper for print feature
- *(mrml-core)* keep style content order
- *(mrml-core)* make sure mj-style components output style elements
- *(core)* replace value attribute by width

### Other
- *(mrml-core)* release version 2.0.0
- *(mrml-common-macros)* update readme and license file
- *(mrml-macros)* update readme and license file
- *(mrml-print-macros)* update readme and license file
- *(mrml-json-macros)* update readme and license file
- bump mrml version in readme
- release ([#340](https://github.com/jdrouet/mrml/pull/340))
- bump mrml-core to 2.0.0-rc.7
- bump mrml-core to 2.0.0-rc.6
- *(mrml-core)* update repository url
- *(mrml-core)* bump syn and darling ([#330](https://github.com/jdrouet/mrml/pull/330))
- bump deps ([#328](https://github.com/jdrouet/mrml/pull/328))
- *(deps)* Update serde-wasm-bindgen requirement from 0.5 to 0.6 in /packages/mrml-wasm ([#327](https://github.com/jdrouet/mrml/pull/327))
- *(mrml-core)* bump to 2.0.0-rc4
- *(html-compare)* update cargo.toml to publish
- *(css-compare)* update cargo.toml to publish
- *(mrml-core)* update code in example
- *(mrml-core)* use a single MrmlParser structure with a visitor pattern ([#317](https://github.com/jdrouet/mrml/pull/317))
- *(mrml-core)* apply clippy suggestions
- format code using rustfmt and create config
- *(mrml-core)* Update indexmap to 2.0
- move license files
- *(mrml-core)* apply clippy suggestions
- prepare for monotrepo
- move code source to only keep mrml-core
- apply clippy suggestions
- *(mrml-core)* release version 1.2.10
- *(mrml-core)* release version 1.2.9
- *(mrml-core)* release version 1.2.8
- Merge pull request [#267](https://github.com/jdrouet/mrml/pull/267) from jdrouet/support-owa-tag
- *(mrml-core)* release version 1.2.7
- *(mrml-core)* release version 1.2.6
- *(mrml-core)* release version 1.2.5
- *(mrml-core)* release version 1.2.4
- *(mrml-core)* make sure feature combinations work
- *(mrml-core)* release version 1.2.3
- *(mrml-core)* release version 1.2.2
- *(mrml-core)* release version 1.2.1
- release version 1.2.0
- release version 1.1.0
- *(mj-carousel)* increase coverage for parse and print
- *(mj-head)* test printing all components
- *(mj-head)* handle parsing with non expected element
- *(mj-title)* increate parse code coverage
- *(mj-preview)* increate parse code coverage
- *(mj-breakpoint)* increate parse code coverage
- *(mj-font)* use macro for json serializing
- *(mj-font)* increate parse code coverage
- *(comment)* test use of is_raw
- *(comment)* check if rendering comments works
- *(comment)* use macro for comment deserializer
- *(print)* use macro to print
- *(json)* use macro for standard deserializers
- *(json)* use macro for standard serializers
- *(json)* use macro for standard serializers
- *(feature)* split into features
- *(core)* simplify print trait
- *(print)* use strings to print elements
- rename packages folder
