.PHONY: all compile test-existing-env test-docker-env test-env test help

all: compile

clean:
	@cargo clean

compile:
	@cargo build

test-existing-env:
	@build/yokozuna
	@cargo test -- --nocapture

test-docker-env:
	@build/yokozuna
	@build/docker
	@cargo test -- --nocapture
	@build/docker cleanup

test:
	@build/yokozuna
	@cargo test --lib -- --nocapture

test-env:
	@build/docker

protogen:
	@git submodule init
	@git submodule update
	@$(PROJDIR)/build/protogen $(PROJDIR)

help:
	@echo ''
	@echo ' Targets:'
	@echo '-----------------------------------------------------------------------------'
	@echo ' all                  - Run everything                                       '
	@echo ' clean                - Do a cargo cleanup                                   '
	@echo ' compile              - Compile the code                                     '
	@echo ' test                 - Run only the unit tests, no Riak required            '
	@echo ' test-env             - Build a persistent docker Env for tests              '
	@echo ' test-existing-env    - Run the tests, provided your own Riak environment    '
	@echo ' test-docker-env      - Run the tests, using a docker based Riak environment '
	@echo ' protogen             - Generate the protocol buffers                        '
	@echo '-----------------------------------------------------------------------------'
	@echo ''
