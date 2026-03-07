#include <iostream>
#include <vector>

class Solution
{
public:
    bool checkOnesSegment(const std::string& s)
    {
        std::vector<char> segments;
        segments.push_back(s[0]);

        for (size_t i = 1; i < s.size(); ++i)
        {
            if (s[i] != s[i - 1])
            {
                segments.push_back(s[i]);
            }
        }

        if (segments.size() > 2)
        {
            return false;
        }

        return true;
    }
};

int main()
{
    std::string s = "1001";
    bool result = Solution().checkOnesSegment(s); // Output: false
    std::cout << std::boolalpha << result << std::endl;

    s = "1000";
    result = Solution().checkOnesSegment(s); // Output: true
    std::cout << std::boolalpha << result << std::endl;

    return 0;
}
