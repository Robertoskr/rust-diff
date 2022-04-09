RUST-DIFF

Warning: This project is for learning purposes, you can use it safely, but for real work 
is recommended to use other tools.

With this small program you can find all the differences from two files. 
For running the program you need to have the rust compiler in your computer.

How to run it.
> rustc filediff.rs
> ./filediff test1.txt test2.txt
And then you will se the results in your terminal: (This is the output with the two files in the repo) 
	-----UPDATED LINE-----
	File 1: this is a test file 1
	File 2: this is a test file 2
	---------------
	File 1: this is intended to test diff algorithm and search the differences
	File 1: how to use it
	File 1: something
	File 1: blablabla
	File 1: a
	File 1: b
	File 1: b
	File 1: c
	-----ADDED LINE-----
	File 2: d
	-----------------

