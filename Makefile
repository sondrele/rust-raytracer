RC = rustc
LIB = src/scene.rs
MAIN = main.rs

EXEC = $(MAIN:.rs=)
TEXEC = $(LIB:.rs=)

$(MAIN): lib
	$(RC) $@ -L .

lib: $(LIB)
	$(RC) --crate-type=lib $^

test:
	$(RC) $(LIB) --test -o $(TEXEC)$@
	./$(TEXEC)$@

clean:
	rm -f $(EXEC)
	rm -f *.rlib