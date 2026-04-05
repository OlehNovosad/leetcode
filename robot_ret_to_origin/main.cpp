#include <array>
#include <ios>
#include <iostream>
#include <map>
#include <string>
#include <unordered_map>

class Solution {
public:
    bool judgeCircle(std::string moves) {
        std::unordered_map<char, std::array<int, 2>> directions = {
            {'L', {-1, 0}}, {'R', {1, 0}}, {'U', {0, 1}}, {'D', {0, -1}}};
        std::array<int, 2> path = {0, 0};

        for (auto const& move : moves) {
            path[0] = path[0] + directions[move][0];
            path[1] = path[1] + directions[move][1];
        }

        std::cout << path[0] << " | " << path[1] << std::endl;

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
