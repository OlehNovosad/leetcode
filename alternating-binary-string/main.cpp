#include <iostream>
#include <string>

class Solution
{
   public:
    int minOperations(std::string& s)
    {
        int count = 0;
        for (int i = 0; i < s.size(); i++)
        {
            if (s[i] - '0' != i % 2)
            {
                count++;
            }
        }
        return std::min(count, (int)s.size() - count);
    }
};

int main(int, char**)
{
    std::string s = "0100";
    int res = Solution().minOperations(s);
    std::cout << "Result: " << res << std::endl;

    s = "1111";
    res = Solution().minOperations(s);
    std::cout << "Result: " << res << std::endl;

    s = "10010100";
    res = Solution().minOperations(s);
    std::cout << "Result: " << res << std::endl;

    return 0;
}
