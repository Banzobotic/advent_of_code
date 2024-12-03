import regex as re

file = open("input", "r")
input = file.read().strip()

sum = 0
for match in re.finditer(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)", input):
    sum += int(match.group(1)) * int(match.group(2))

print(sum)

start = -1
deleted = 0
for match in re.finditer(r"(do|don't)\(\)", input):
    if match.group(1) == "do" and start != -1:
        l = len(input)
        input = input[:(start - deleted)] + input[(match.start() - deleted):]
        deleted += l - len(input)
        start = -1
    elif match.group(1) == "don't" and start == -1:
        start = match.start()
if start != -1:
    input = input[:(start - deleted)]

sum = 0
for match in re.finditer(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)", input):
    sum += int(match.group(1)) * int(match.group(2))

print(sum)
