#include <algorithm>
#include <climits>
#include <iostream>
#include <map>
#include <vector>

class Solution {
public:
    int minimumDistance(std::vector<int>& nums) {
        int min_res = INT_MAX;
        std::map<int, std::vector<int>> positions;
        for (int i = 0; i < (int)nums.size(); i++) {
            positions[nums[i]].push_back(i);
        }

        for (auto const& entry : positions) {
            const std::vector<int>& indices = entry.second;
            if (indices.size() < 3)
                continue;
            for (int i = 0; i + 2 < (int)indices.size(); i++) {
                int res = 2 * (indices[i + 2] - indices[i]);
                min_res = std::min(res, min_res);
            }
        }

        return (min_res != INT_MAX) ? min_res : -1;
    }
};

int main(int, char**) {
    std::vector<int> nums = {5, 3, 5, 5, 5};
    int result = Solution().minimumDistance(nums);
    std::cout << result << std::endl;

    return 0;
}
