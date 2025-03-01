# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.5.0](https://github.com/jdrouet/mrml/compare/mrml-wasm-v1.4.12...mrml-wasm-v1.5.0) - 2025-03-01

### Added

- improve error handling (#452)
- parsing warnings and improve errors (#451)

### Fixed

- *(mrml-wasm)* remove wee_alloc dependency (#472)

### Other

- use package attributes from workspace (#488)
- *(deps)* upgrade multiple dependencies (#476)
- *(mrml-core)* base all component on the same struct (#448)

## [1.4.12](https://github.com/jdrouet/mrml/compare/mrml-wasm-v1.4.11...mrml-wasm-v1.4.12) - 2024-06-30

### Other
- updated the following local packages: mrml

## [1.4.11](https://github.com/jdrouet/mrml/compare/mrml-wasm-v1.4.10...mrml-wasm-v1.4.11) - 2024-06-13

### Other
- *(deps)* Update itertools requirement from 0.12.1 to 0.13.0 in /packages/mrml-core ([#425](https://github.com/jdrouet/mrml/pull/425))

## [1.4.10](https://github.com/jdrouet/mrml/compare/mrml-wasm-v1.4.9...mrml-wasm-v1.4.10) - 2024-04-13

### Other
- updated the following local packages: mrml

## [1.4.9](https://github.com/jdrouet/mrml/compare/mrml-wasm-v1.4.8...mrml-wasm-v1.4.9) - 2024-04-04

### Other
- *(mrml-wasm)* update build script

## [1.4.8](https://github.com/jdrouet/mrml/compare/mrml-wasm-v1.4.7...mrml-wasm-v1.4.8) - 2024-03-21

### Other
- updated the following local packages: mrml

## [1.4.7](https://github.com/jdrouet/mrml/compare/mrml-wasm-v1.4.6...mrml-wasm-v1.4.7) - 2024-03-21

### Other
- switch license to MIT ([#388](https://github.com/jdrouet/mrml/pull/388))

## [1.4.6](https://github.com/jdrouet/mrml/compare/mrml-wasm-v1.4.5...mrml-wasm-v1.4.6) - 2024-03-19

### Other
- *(mrml-core)* move everythin in Arc ([#392](https://github.com/jdrouet/mrml/pull/392))

## [1.4.5](https://github.com/jdrouet/mrml/compare/mrml-wasm-v1.4.4...mrml-wasm-v1.4.5) - 2024-03-15

### Other
- updated the following local packages: mrml

## [1.4.4](https://github.com/jdrouet/mrml/compare/mrml-wasm-v1.4.3...mrml-wasm-v1.4.4) - 2024-03-04

### Other
- updated the following local packages: mrml

## [1.4.3](https://github.com/jdrouet/mrml/compare/mrml-cli-v1.4.2...mrml-python-v1.4.3) - 2024-02-28

### Other
- updated the following local packages: mrml

## [1.4.2](https://github.com/jdrouet/mrml/compare/mrml-wasm-v1.4.1...mrml-wasm-v1.4.2) - 2024-01-09

### Other
- *(mrml-wasm)* update package building ([#362](https://github.com/jdrouet/mrml/pull/362))

## [1.4.1](https://github.com/jdrouet/mrml/compare/mrml-wasm-v1.4.0...mrml-wasm-v1.4.1) - 2023-12-20

### Other
- release ([#360](https://github.com/jdrouet/mrml/pull/360))

## [1.4.0](https://github.com/jdrouet/mrml/compare/mrml-wasm-v1.3.0...mrml-wasm-v1.4.0) - 2023-12-07

### Added
- *(mrml-core)* make the include_loader be Send and Sync ([#347](https://github.com/jdrouet/mrml/pull/347))

### Other
- *(mrml-wasm)* update Cargo.toml
- *(mrml-core)* rename Options to RenderOptions ([#352](https://github.com/jdrouet/mrml/pull/352))
- update github actions configuration ([#351](https://github.com/jdrouet/mrml/pull/351))
- release ([#348](https://github.com/jdrouet/mrml/pull/348))
- *(mrml-core)* release version 2.0.0

## [1.3.0](https://github.com/jdrouet/mrml/compare/mrml-wasm-v1.2.3...mrml-wasm-v1.3.0) - 2023-12-01

### Added
- handle properly async mj-include ([#346](https://github.com/jdrouet/mrml/pull/346))
- implement an async parser ([#338](https://github.com/jdrouet/mrml/pull/338))
- *(mrml-wasm)* add mj-include feature ([#316](https://github.com/jdrouet/mrml/pull/316))
- *(mrml-wasm)* create wasm package ([#312](https://github.com/jdrouet/mrml/pull/312))
- *(wasm)* add toJson, toMjml and validate

### Fixed
- *(mrml-wasm)* disable_comments is in camelCase
- *(json)* add serde to mrml-wasm

### Other
- bump mrml-core to 2.0.0-rc.7
- bump mrml-core to 2.0.0-rc.6
- bump deps ([#328](https://github.com/jdrouet/mrml/pull/328))
- *(deps)* Update serde-wasm-bindgen requirement from 0.5 to 0.6 in /packages/mrml-wasm ([#327](https://github.com/jdrouet/mrml/pull/327))
- *(mrml-core)* bump to 2.0.0-rc4
- *(mrml-core)* use a single MrmlParser structure with a visitor pattern ([#317](https://github.com/jdrouet/mrml/pull/317))
- format code using rustfmt and create config
- *(mrml-wasm)* add test with disabling comments
- *(mrml-wasm)* update node example to have test command
- *(mrml-wasm)* add browser example ([#315](https://github.com/jdrouet/mrml/pull/315))
- *(mrml-wasm)* add node example with a test ([#313](https://github.com/jdrouet/mrml/pull/313))
- *(mrml-wasm)* update readme
- move code source to only keep mrml-core
- apply clippy suggestions
- *(mrml-wasm)* release version 1.2.3
- *(mrml-wasm)* release version 1.2.2
- *(mrml-wasm)* release version 1.2.1
- release version 1.2.0
- release version 1.1.0
- rename packages folder
