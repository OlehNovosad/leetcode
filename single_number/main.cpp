#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution
{
  public:
	int singleNumber(vector<int> &nums)
	{
		unordered_map<int, int> arr_map; // key: is value, value: occurance

		for (const auto &num : nums)
		{
			arr_map[num]++;
		}

		for (const auto &pair : arr_map)
		{
			if (pair.second == 1)
			{
				return pair.first;
			}
		}

		return -1;
	}
};

int main()
{
	vector<int> arr = {2, 2, 1};
	int res = Solution().singleNumber(arr);
	cout << res << endl;

	return 0;
}
