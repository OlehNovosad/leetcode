#include <algorithm>
#include <climits>
#include <iostream>
#include <vector>

using namespace std;

class Solution
{
  public:
	int minimumDifference(vector<int> &nums, int k)
	{
		if (k <= 1)
		{
			return 0;
		}

		if (nums.size() <= 1)
		{
			return 0;
		}

		sort(nums.begin(), nums.end());

		int min_val = INT_MAX;

		for (int i = 0; i + k - 1 < (int)nums.size(); i++)
		{
			min_val = min(min_val, nums[i + k - 1] - nums[i]);
		}

		return min_val;
	}
};

int main()
{
	vector<int> nums = {87063, 61094, 44530, 21297, 95857, 93551, 9918};
	int k = 6;
	cout << Solution().minimumDifference(nums, k) << endl;

	return 0;
}
