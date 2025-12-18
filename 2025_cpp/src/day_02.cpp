#include <cmath>
#include <cstddef>
#include <ctime>
#include <fstream>
#include <iostream>
#include <sstream>
#include <vector>

using namespace std;

bool is_safe(const int *report, size_t size) {
  if (size < 2)
    return true;

  bool increasing = report[1] > report[0];

  for (size_t i = 1; i < size; i++) {
    int diff = report[i] - report[i - 1];
    int adiff = abs(diff);

    if (adiff < 1 || adiff > 3 || (diff > 0) != increasing) {
      return false;
    }
  }

  return true;
}

int parse_line(vector<int> &temp_vec, string &line) {
  temp_vec.clear();
  stringstream ss(line);
  int num;
  while (ss >> num) {
    temp_vec.push_back(num);
  }

  return is_safe(temp_vec.data(), temp_vec.size());
}

int main() {
  ifstream file("inputs/input.txt");
  if (!file) {
    cerr << "Failed to open input file" << endl;
    return 1;
  }

  string line;
  vector<int> report;
  int count = 0;

  while (getline(file, line)) {
    count += parse_line(report, line);
  }

  cout << "Safe reports: " << count << endl;
  return 0;
}
