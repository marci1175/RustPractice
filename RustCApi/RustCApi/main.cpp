#include <iostream>
#include "main.h"

void hello() {
	std::cout << "Hello from C++\n";
}

int times_two(int number) {
	return number * 2;
}

std::string reverse_string(std::string string) {
	std::string buf;
	for (int i = 0; i < string.length(); i++) {
		buf.push_back(string.at(string.length() - i - 1));
	};

	return buf;
}
