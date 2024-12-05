from collections import defaultdict

file = open("input", "r")
input = file.read().strip()

rules, orders = input.split("\n\n");
rules_map = defaultdict(lambda: set())

for rule in rules.splitlines():
    a, b = rule.split("|")
    rules_map[int(a)].add(int(b))

incorrect = []
sum = 0
for order in orders.splitlines():
    order = [int(x) for x in order.split(",")]

    def check_order(order):
        for i in range(1, len(order)):
            for j in range(i):
                if order[j] in rules_map[order[i]]:
                    return False
        return True

    if check_order(order):
        sum += order[len(order) // 2]
    else:
        incorrect.append(order)
print(sum)

sum = 0
for order in incorrect:
    for i in range(1, len(order)):
        for j in range(i):
            if order[j] in rules_map[order[i]]:
                temp = order[j]
                order[j] = order[i]
                order[i] = temp
    sum += order[len(order) // 2]
print(sum)
