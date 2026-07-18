#include <algorithm>
#include <iostream>
#include <vector>

class Solution {
public:
    int findGCD(std::vector<int>& nums) {
        std::sort(nums.begin(), nums.end());

        int smallest = nums[0];
        int biggest = nums[nums.size() - 1];

        while (biggest != 0) {
            int remainder = smallest % biggest;
            smallest = biggest;
            biggest = remainder;
        }

        return smallest;
    }
};

int main(int, char**) {
    std::vector<int> nums = {2, 6, 5, 9, 10};
    int res = Solution().findGCD(nums);
    std::cout << res << std::endl;

    return 0;
}
