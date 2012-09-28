
all: levenshtein

levenshtein: %.so

%.so: levenshtein.rs levenshtein.rc
	rustc --lib levenshtein.rc

clean:
	-rm liblevenshtein-*.so
	-rm levenshtein

test: levenshtein.rs levenshtein.rc
	rustc --test levenshtein.rc
	./levenshtein
	rm levenshtein

.PHONY: all clean test
