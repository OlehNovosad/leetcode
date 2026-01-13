class Solution:
    def removeElement(self, nums: list[int], val: int) -> int:
        res = 0

        for i in range(0, len(nums)):
            for j in range(1, len(nums)):
                if nums[j - 1] == val:
                    nums[j - 1], nums[j] = nums[j], nums[j - 1]

        for i in nums:
            if i != val:
                res += 1

        return res


nums = [0, 1, 2, 2, 3, 0, 4, 2]
val = 2
res = Solution().removeElement(nums, val)
print(nums)
print(f"{res}")
