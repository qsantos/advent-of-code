
a = 2
while True:
    d = a + 4 * 643
    bits = f'{d:b}'[::-1]
    if all(bits[i] == '01'[i % 2] for i in range(len(bits))):
        print(a)
        break
    a += 2
