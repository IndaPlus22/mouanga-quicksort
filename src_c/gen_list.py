# note: you have to remove the leading space from the second line in test.txt
# after running this file

import random
string = "100000\n"
for _ in range(10000):
    RANDINT = random.randint(-10000, 10000)
    string += f" {RANDINT}"

with open("test.txt", "w") as z:
    z.write(string)