from copy import deepcopy
import re
file = open("input", "r")
input = file.read().strip()

input = [list(l) for l in input.splitlines()]

pos = None
for i, line in enumerate(input):
    for j, c in enumerate(line):
        if c in {"^", ">", "v", "<"}:
            pos = (i, j)
            break

dirs = {
    "^": (-1, 0),
    ">": (0, 1),
    "v": (1, 0),
    "<": (0, -1),
}
turn = {
    "^": ">",
    ">": "v",
    "v": "<",
    "<": "^",
}

def check_map(input, pos):
    while True:
        x, y = pos
        guard = input[x][y]
        old_guard = guard
        dir_x, dir_y = dirs[guard]
        new_x, new_y = (x + dir_x, y + dir_y)

        if new_x not in range(0, len(input)) or new_y not in range(0, len(input[0])):
            break

        while input[new_x][new_y] == "#":
            guard = turn[guard]
            dir_x, dir_y = dirs[guard]
            new_x, new_y = (x + dir_x, y + dir_y)

        if input[new_x][new_y] == guard:
            return None

        pos = (new_x, new_y)
        input[x][y] = old_guard
        input[new_x][new_y] = guard
    return input

count = 0
loop_count = 0
for i, line in enumerate(check_map(deepcopy(input), pos)):
    for j, c in enumerate(line):
        if c in {"^", ">", "v", "<"}:
            count += 1

            if (i, j) == pos:
                continue
            test_input = deepcopy(input)
            test_input[i][j] = "#"
            if check_map(test_input, pos) is None:
                loop_count += 1


print(count)
print(loop_count)
