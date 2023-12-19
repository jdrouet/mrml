# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
