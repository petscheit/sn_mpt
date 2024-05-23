.PHONY: build test coverage
cairo_files = $(shell find ./tests/cairo_programs -name "*.cairo")
VENV_PATH ?= venv

build:
	$(MAKE) clean
	./scripts/build.sh

setup:
	./scripts/setup.sh ${VENV_PATH}

run-profile:
	@echo "A script to select, compile, run & profile one Cairo file"
	./scripts/launch_cairo_files.py -profile

run:
	@echo "A script to select, compile & run one Cairo file"
	@echo "Total number of steps will be shown at the end of the run." 
	./scripts/launch_cairo_files.py