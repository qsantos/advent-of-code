with open('input') as f:
    digits = f.read().strip()

n = len(digits)

# part 1
print(sum(
    int(digits[i])
    for i in range(n)
    if digits[i] == digits[(i + 1) % n]
))

# part 1
print(sum(
    int(digits[i])
    for i in range(n)
    if digits[i] == digits[(i + n // 2) % n]
))
