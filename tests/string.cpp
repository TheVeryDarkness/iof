// clang++ -std=c++17 -O2 -Wall string.cpp -o string && clear && ./string && rm ./string
#include <cassert>
#include <iostream>
#include <string>

int main() {
  std::uint64_t x;
  std::cin >> x;

  std::cin >> std::ws;
  assert(std::cin);

  std::string s;
  std::cin >> s;
  std::cout << '{' << x << '}' << '{' << s << '}';

  assert(std::cin);
  // std::getline(std::cin, s);
  // std::cout << '{' << s << '}';
  return 0;
}
