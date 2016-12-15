#!/usr/bin/env python

#discs = [(10,13), (15,17), (17,19), (1,7), (0,5), (1,3)]
discs = [(10,13), (15,17), (17,19), (1,7), (0,5), (1,3), (0,11)]

def check(discs, time):
    d = [0]
    def check_one(acc, x):
        d[0]-=1
        pos, siz = x
        return acc and (pos+time)%siz == d[0]%siz
    return reduce(check_one, discs, True)

time = 0
while True:
    if not check(discs, time):
        time += 1
    else:
        break
print time
