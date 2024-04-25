// Build:
// g++ -std=c++11 -O3 -march=native main.cpp -o diff_cpp

#include <iostream>
#include <cmath>
#include <limits>
#include <iomanip> 

double difficulty(const unsigned bits) {
  const unsigned exponent_diff  = 8 * (0x1D - ((bits >> 24) & 0xFF));
  const double significand = bits & 0xFFFFFF;
  return std::ldexp(0x00FFFF / significand, exponent_diff);
}

int main()
{
  double d = difficulty(0x1c0eba64);
  std::cout << std::setprecision(std::numeric_limits<double>::max_digits10) << d << std::endl;

  return 0;
}
