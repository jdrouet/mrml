format:
	rustfmt --check src/{**/,}*.rs

ci-coverage:
	cargo tarpaulin --out Xml
	bash <(curl -s https://codecov.io/bash)
