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
ctr = 0
for n in node:
    for z in node:
        if n!=z and n[2]!=0 and n[2]<z[3]:
            ctr += 1
print ctr

