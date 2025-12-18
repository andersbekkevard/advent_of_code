#include <fstream>
#include <iostream>
#include <vector>

using namespace std;

vector<string> read_file(string filename) {
  fstream file(filename);
  vector<string> lines;
  string line;
  while (getline(file, line)) {
    lines.push_back(line);
  }
  return lines;
}

int parse_line(string &line) {
  string sub = line.substr(1);
  return line.front() == 'R' ? stoi(sub) : -stoi(sub);
}

int main() {
  vector<string> lines = read_file("inputs/input.txt");

  int count = 0;
  int val = 50;
  const int mod = 100;

  for (string line : lines) {
    val = (val + parse_line(line)) % mod;

    count += (val == 0);
  }
  cout << count;
  return 0;
}
