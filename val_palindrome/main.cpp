#include <algorithm>
#include <cctype>
#include <iostream>
#include <string>

using namespace std;

class Solution
{
  public:
	bool isPalindrome(string s)
	{
		if (s.size() == 1)
		{
			return true;
		}

		s.erase(remove_if(s.begin(), s.end(), [](char ch)
						  { return !isalnum(ch); }),
				s.end());

		if (s.size() == 0)
		{
			return true;
		}

		for (int i = 0, j = s.size() - 1; i < j; i++, j--)
		{
			if (tolower(s[i]) != tolower(s[j]))
			{
				return false;
			}
		}

		return true;
	}
};

int main()
{
	string s = "aa";
	bool res = Solution().isPalindrome(s);
	cout << res << endl;
}
