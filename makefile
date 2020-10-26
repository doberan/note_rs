MAKEFILE_DIR:=$(dir $(abspath $(lastword $(MAKEFILE_LIST))))
watch-run:
	cargo watch -x run
resource-server-start:
	php -S localhost:8000 -t ${MAKEFILE_DIR}resources
