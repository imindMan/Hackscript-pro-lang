// This is the main file of this project. If you are reading to this project and you try
// to find the main file, then yeah, this is your right place.
//
// The start of Hackscript, one of my hobby project when I have free time :)))
// Nothing special - imindMan
// @author  imindMan, a solo developer
// @date Mon February 6th, 2023 
//
// This is the rebuilt project of Hackscript written in Python, due to its slow speed
// This language will rebuild again to C, a very fast programming language
// 
// Powered by C/C++/Rust mainly.
//
// @info: Hackscript is a Turing-complete, esoteric-style
// high-level-style, the "easy Malbolge" programming language. Its purpose is to be 
// a fun programming language like other esoteric programming language.
// but I will plan to make it can be used for real working.
//
// Feel free to contribute, more information about this project are all on the github repo
//
// https://github.com/imindMan/Hackscript-pro-lang
// 

#include <stdio.h>
#include "include/input.h"
#include <stdlib.h>
#include <string.h>

// the main function 
int main(int argc, char **argv) 
{
	// test the command input firstly
	const char *final = input("Enter your input: ");
	printf("%s\n", final);
	return 0;

}
