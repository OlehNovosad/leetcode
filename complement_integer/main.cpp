#include <bitset>
#include <iostream>

class Solution {
public:
    int bitwiseComplement(int n) {
        if (n == 0)
            return 1;

        unsigned int mask = 1;

        while (mask < n) {
            mask = (mask << 1) | 1;
        }

        return n ^ mask;
    }
};

int main(int, char**) {
    int n = 5;
    int result = Solution().bitwiseComplement(n);
    std::cout << result << std::endl;
    return 0;
}
