#include <array>
#include <iostream>
#include <string>

class Solution {
public:
    bool judgeCircle(std::string moves) {
        std::array<int, 2> path = {0, 0};

        for (auto const& move : moves) {
            switch (move) {
                case 'R':
                    path[0]++;
                    break;
                case 'L':
                    path[0]--;
                    break;
                case 'U':
                    path[1]++;
                    break;
                case 'D':
                    path[1]--;
                    break;
                default:
                    break;
            }
        }

        if (path[0] != 0 || path[1] != 0) {
            return false;
        }

        return true;
    }
};

int main(int, char**) {
    std::string moves = "LL";
    bool res = Solution().judgeCircle(moves);
    std::cout << std::boolalpha << res << std::endl;

    return 0;
}
