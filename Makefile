.PHONY: all rust test cargo-test figures macros paper clean

all: rust test paper

.venv:
	python3 -m venv .venv
	. .venv/bin/activate && pip install -q -r requirements.txt

rust: .venv
	. .venv/bin/activate && maturin develop --release

figures: rust
	. .venv/bin/activate && python -m shbt_recon.plots

macros: rust
	. .venv/bin/activate && python -m shbt_recon.latex

paper: figures macros
	pdflatex -interaction=nonstopmode main.tex
	pdflatex -interaction=nonstopmode main.tex

test: rust
	. .venv/bin/activate && pytest tests/ -q

cargo-test:
	cargo test -q

clean:
	rm -rf .venv target python/shbt_recon/*.so python/shbt_recon/__pycache__ tests/__pycache__ recon_results.tex figures/*.pdf *.aux *.log *.out *.toc *.synctex.gz
