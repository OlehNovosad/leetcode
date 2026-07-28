#include <algorithm>
#include <cstring>
#include <iostream>
#include <string>

class Solution {
public:
    std::string smallestPalindrome(std::string s) {
        int len = s.length();

        if (1 == len) {
            return s;
        }

        char sArr[len + 1];
        s.copy(sArr, len / 2, 0);

        std::sort(sArr, sArr + len / 2);

        std::string revSArr(sArr, len / 2);
        std::reverse(revSArr.begin(), revSArr.end());

        if (len % 2 != 0) {
            sArr[len / 2] = s[len / 2];
            sArr[(len / 2) + 1] = '\0';
        } else {
            sArr[len / 2] = '\0';
        }

        std::strcat(sArr, revSArr.c_str());

        return sArr;
    }
};

int main(int, char**) {
    std::string s = "babab";
    std::string res = Solution().smallestPalindrome(s);
    std::cout << res << std::endl;

    return 0;
}
