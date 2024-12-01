from collections import Counter

f = open("input", "r")
input = f.read().strip()

l1 = []
l2 = []
counts = Counter()

for line in input.split("\n"):
    a, b = line.split()
    l1.append(int(a))
    l2.append(int(b))
    counts[int(b)] += 1

l1.sort()
l2.sort()

sum = 0

for a, b in zip(l1, l2):
    sum += abs(b - a)

print(sum)

sum2 = 0

for a in l1:
    sum2 += counts[a] * a

print(sum2)