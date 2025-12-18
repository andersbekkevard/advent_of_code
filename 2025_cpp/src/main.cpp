#include <iostream>
#include <sstream>
#include <string>
using namespace std;
int main() {
  string s = "1 12";
  // Fix: include <sstream> header for stringstream
  stringstream ss(s);
  int x = 10;
  int y;
  ss >> y;
  ss >> y;
  cout << x + y;
}
