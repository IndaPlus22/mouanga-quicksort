# note: you have to remove the leading space from the second line in test.txt
# after running this file

import random
string = "100000\n"
for _ in range(100000):
    RANDINT = random.randint(-100000, 100000)
    string += f" {RANDINT}"

with open("test.txt", "w") as z:
    z.write(string)