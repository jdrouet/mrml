update-resources:
	docker run --rm -v $(shell pwd):/code -w /code node:lts sh scripts/update-resources.sh

