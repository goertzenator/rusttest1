


main: main.o liblrust.so
	gcc -o $@ $<  -llrust -ldl -lpthread -lrt -lgcc_s -lc -lm -L . 

liblrust.so: lrust.rs
	rustc $<
