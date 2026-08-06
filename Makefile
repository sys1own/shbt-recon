.PHONY: all rust test cargo-test clean

all: rust test

.venv:
	python3 -m venv .venv
	. .venv/bin/activate && pip install -q -r requirements.txt

rust: .venv
	. .venv/bin/activate && maturin develop --release

test: rust
	. .venv/bin/activate && pytest tests/ -q

cargo-test:
	cargo test -q

clean:
	rm -rf .venv target python/shbt_recon/*.so python/shbt_recon/__pycache__ tests/__pycache__
