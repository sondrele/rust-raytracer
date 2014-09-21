RC = rustc
LIB = src/tracer.rs
MAIN = main.rs

EXEC = $(MAIN:.rs=)
TEXEC = $(LIB:.rs=)

.PHONY: libs main test clean

main: libs
	$(RC) -L . main.rs -o main

libs:
	$(RC) -L . --crate-type=lib lib/bmp/src/bmp.rs
	$(RC) -L . --crate-type=lib $(LIB)
	@echo "Crates compiled"

test:
	$(RC) $(LIB) --test -L . -o $(TEXEC)
	./$(TEXEC)

clean:
	rm -f $(EXEC)
	rm -f $(TEXEC)
	rm -f *.rlib