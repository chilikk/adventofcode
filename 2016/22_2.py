import sys,re

node = []
for line in iter(sys.stdin.readline, b''):
    match = re.match(r"/dev/grid/node-x(\d*)-y(\d*)\s*\d*T\s*(\d*)T\s*(\d*)T",
            line)
    x = int(match.group(1))
    y = int(match.group(2))
    used = int(match.group(3))
    avail = int(match.group(4))
    node.append((x,y,used,avail))
prev = (-1, -1, -1, -1)
xmax = 34
for n in node:
    if prev[0]!=n[0]:
        prev = n
        print
    if n[2]>100 and 1.0*n[2]/n[3]>0.9:
        symbol = "#"
    elif n[0]==xmax and n[1]==0:
        symbol = "G"
    elif n[2] == 0:
        symbol = "_"
    else:
        symbol = "."
    sys.stdout.write(symbol+" ")

## final answer: shortest path from _ to G + 5 * (xmax-1)
