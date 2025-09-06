#include <bits/stdc++.h>
using namespace std;

class TimePont {
 private:
  chrono::steady_clock::time_point _start;

 public:
  TimePont() {
    _start = chrono::steady_clock::now();
    cout << "------- Now is 0 ......" << endl;
  };
  ~TimePont() {};
  void time_here() {
    auto now = chrono::steady_clock::now();
    auto ret = now - _start;
    auto duration = chrono::duration_cast<chrono::milliseconds>(ret);
    cout << "------- Now is " << duration.count() << " ms ......" << endl;
  }
};

void test1(int count) {
  cout << "======= k-v 个数:" << count << endl;
  uint64_t first = 10000000;
  uint64_t txt = 10000000000000;
  unordered_map<uint64_t, uint64_t> h1;
  TimePont time_point;
  for (size_t i = 0; i < count; i++) {
    auto ret = h1.insert(make_pair(first + i, txt));
  }
  cout << "下面是创建 hashmap 时间戳....." << endl;
  time_point.time_here();
}
int main(char** argv, int argc) {
  cout << "--------- test c++ hashmap ----------" << endl;
  int count = 5000 * 10000;
  test1(count);
  return 0;
}