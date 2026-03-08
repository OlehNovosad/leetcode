#include <iostream>
#include <vector>

class Solution
{
public:
    std::string findDifferentBinaryString(std::vector<std::string>& nums)
    {
        std::vector<int> local_nums_int;

        local_nums_int.reserve(nums.size());
        for (const auto& num : nums)
        {
            local_nums_int.push_back(std::stoi(num, nullptr, 2));
        }

        for (int i = 0; i < local_nums_int.size(); i++)
        {
            if (std::find(local_nums_int.begin(), local_nums_int.end(), i) == local_nums_int.end())
            {
                return std::bitset<32>(i).to_string().substr(32 - nums[0].size());
            }
        }

        return std::bitset<32>(local_nums_int.back() + 1).to_string().substr(32 - nums[0].size());
    }
};

int main()
{
    std::vector<std::string> nums = {"01", "10"};
    std::string result = Solution().findDifferentBinaryString(nums);
    std::cout << "Result: " << result << std::endl;

    return 0;
}
