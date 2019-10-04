# If n is a positive integer greater than one, what is the largest integer that
# will always be a factor of: (n^3 - n).

for i in range(2, 1000):
    value = i**3 - 3
    # print(value)
    factorList = []
    for j in range(j, 1000000000):
        if value % j == 0:
            factorList.append(j)
        else:
            factorList.remove(j)
print(factorList)