file = open("input", "r")
input = file.read()

def is_safe(nums):
    op = 0
    if nums[0] < nums[1]:
        op = lambda x, y: x < y
    else:
        op = lambda x, y: x > y
    for i in range(len(nums) - 1):
        if not (1 <= abs(nums[i] - nums[i+1]) <= 3):
            return (False, i)
        elif not op(nums[i], nums[i+1]):
            return (False, i)
    return (True, 0)

count = 0
count2 = 0
for line in input.splitlines():
    nums = [int(x) for x in line.split()]
    safe, i = is_safe(nums)
    
    if safe:
        count += 1
    if not safe:
        for j in range(i-1, i+2):
            nums2 = nums[:j] + nums[(j+1):]
            if is_safe(nums2)[0]:
                safe = True
                break
    if safe:
        count2 += 1

print(count)
print(count2)
