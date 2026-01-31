#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

class Solution
{
  public:
	char nextGreatestLetter(vector<char> &letters, char target)
	{
		sort(letters.begin(), letters.end());
		for (const auto &ch : letters)
		{
			if (ch > target)
			{
				return ch;
			}
		}

		return letters[0];
	}
};

int main()
{
	vector<char> letters = {'c', 'f', 'j'};
	char target = 'a';
	cout << Solution().nextGreatestLetter(letters, target) << endl;
}
