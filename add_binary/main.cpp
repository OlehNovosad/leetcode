#include <algorithm>
#include <cstdlib>
#include <iostream>
#include <string>

using namespace std;

class Solution
{
  public:
	string addBinary(string a, string b)
	{
		string res;
		int carry = 0;
		int i = a.length() - 1;
		int j = b.length() - 1;

		while (i >= 0 || j >= 0 || carry)
		{
			int sum = carry;
			if (i >= 0)
			{
				sum += a[i] - '0';
				i--;
			}
			if (j >= 0)
			{
				sum += b[j] - '0';
				j--;
			}
			res += (sum % 2) + '0';
			carry = sum / 2;
		}

		reverse(res.begin(), res.end());
		return res;
	}
};

int main()
{
	string a = "10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101";
	string b = "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011";

	string res = Solution().addBinary(a, b);
	cout << res << endl;
}
