# Rust
rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt 

run:
	cargo run

lint:
	cargo clippy 

test:
	cargo test 

check:
	cargo check

release:
	cargo build --release

all: format lint test run

# Python

python_install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

python_test:
	python -m pytest -vv --cov=main --cov=mylib test_*.py

python_format:	
	black *.py 

python_lint:
	ruff check *.py mylib/*.py

python_container-lint:
	docker run --rm -i hadolint/hadolint < Dockerfile

python_refactor: format lint

python_all: install lint test format