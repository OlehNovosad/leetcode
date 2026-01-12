class Solution:
    def isValid(self, s: str) -> bool:
        
        parantheses_map = {
            "}": "{",
            ")": "(",
            "]": "[",
        }
        stack = []

        for ch in s:
            if ch in parantheses_map:
                if len(stack) == 0:
                    return False
                if stack.pop() != parantheses_map[ch]:
                    return False
            else:
                stack.append(ch)

        return not (bool(len(stack)))


s = "()"
res = Solution().isValid(s)
print(res)


s = "]"
res = Solution().isValid(s)
print(res)
