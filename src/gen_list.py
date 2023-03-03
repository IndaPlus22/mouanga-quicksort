import random

NUMBERS = 10000

string = f"{NUMBERS}\n0"
for _ in range(NUMBERS - 1):
    RANDINT = random.randint(-NUMBERS, NUMBERS)
    string += f" {RANDINT}"

with open("test.txt", "w") as z:
    z.write(string)