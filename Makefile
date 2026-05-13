.PHONY: raw
raw:
	pandoc -f markdown -t json -o target/caption.json caption.md
	pandoc -f markdown -t html -o target/caption.html caption.md

.PHONY: filter
filter:
	pandoc -f markdown -t json --filter ./target/debug/pandoc-caption -o ./target/caption.json tests/caption.md
	pandoc -f markdown -t html --filter ./target/debug/pandoc-caption -o ./target/caption.html tests/caption.md

	