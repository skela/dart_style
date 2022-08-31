default: build run

.PHONY: build
build:
	dart compile exe bin/format.dart -o bin/sdartfmt

.PHONY: reset
reset:
	cp benchmark/test1.dart.txt benchmark/test1.dart

.PHONY: sdartfmt
sdartfmt: reset _format_with_sdartfmt

.PHONY: astyle
astyle: reset _format_with_astyle

.PHONY: uncrustify
uncrustify: reset _format_with_uncrustify

# Codebuff can be found in git@github.com:antlr/codebuff.git
.PHONY: codebuff
codebuff: reset _format_with_codebuff

.PHONY: blink
blink: reset _format_with_blink

.PHONY: both
both: reset _format_with_sdartfmt _format_with_astyle

.PHONY: _format_with_sdartfmt
_format_with_sdartfmt:
	bin/sdartfmt --fix -w benchmark/test1.dart

.PHONY: _format_with_astyle
_format_with_astyle:
	python3 format.py -m astyle

.PHONY: _format_with_uncrustify
_format_with_uncrustify:
	python3 format.py -m uncrustify

.PHONY: _format_with_codebuff
_format_with_codebuff:
	python3 format.py -m codebuff

.PHONY: _format_with_blink
_format_with_blink:
	python3 format.py -m blink

# all astyle options here:
# http://astyle.sourceforge.net/astyle.html
# i dont like these, but others might
#--add-braces  [Add braces to unbraced one line conditional statements]
# the opposite of that is this one here, which i also dont like, but others might prefer
# --remove-braces [Remove braces from braced one line conditional statements]
