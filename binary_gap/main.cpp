#include <algorithm>
#include <iostream>
#include <string>

class Solution {
  public:
    int binaryGap(int n) {
        std::string bin = "";
        int res         = 0;
        int counter     = 0;
        bool started    = false;

        while (n > 0) {
            int bit = n % 2;
            bin.push_back('0' + bit);
            n /= 2;
        }
        std::reverse(bin.begin(), bin.end());

        for (const auto ch : bin) {
            if (ch == '1') {
                ++counter;
                if (started) {
                    res = std::max(res, counter);
                }
                started = true;
                counter = 0;
            } else {
                ++counter;
            }
        }

        return res;
    }
};

int main(int, char**) {
    int n   = 22;
    int res = Solution().binaryGap(n);
    std::cout << res << std::endl;
}
