// input method written in C++
// so, basically, this function will get the input from the user in C++
// to make it easier to got the input 
// idea: get the input in string literal and convert it to char *
#include <iostream>
#include <string>
#include <bits/stdc++.h>
#include <cstdio>
extern "C" {

	#include "../include/input.h"
}

const char *input(char *what_to_print)
{
	printf("%s", what_to_print);	
	std::string input_from_user;
	std::getline(std::cin, input_from_user);
	std::cout << input_from_user;	

	const char *final_input = input_from_user.c_str();
	return final_input;
}
