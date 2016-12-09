#!/usr/bin/env python

import sys
pos = (0,2)
lines = sys.stdin.readlines()
for line in lines:
    for c in line:
        x,y = pos
        if c=='U':
            pos = (x, y-1 if x+y>2 and y-x>-2 else y)
        elif c=='D':
            pos = (x, y+1 if y-x<2 and x+y<6 else y)
        elif c=='L':
            pos = (x-1 if x+y>2 and y-x<2 else x, y)
        elif c=='R':
            pos = (x+1 if y-x>-2 and x+y<6 else x, y)
    print pos
    #x,y = pos
    #sys.stdout.write("%s" % (3*y+x+1))
print
