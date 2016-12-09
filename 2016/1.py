import sys

c=(0,0)
d=(0,1)
steps = sys.stdin.readline()
steps = map(lambda x: x.strip(), steps.split(','))
visited = [c]
done=0
for step in steps:
    print 
    print step
    turn = step[0]
    dist = int(step[1:])
    if turn == 'R':
        dx, dy = d
        d = dy, -dx
    elif turn == 'L':
        dx, dy = d
        d = -dy, dx
    dx, dy = d
    cx, cy = c
    for _ in range(0,dist):
        cx, cy = c
        c = (cx+dx, cy+dy)
        print c
        if c not in visited:
            visited += [c]
        else:
            cx, cy = c
            print abs(cx)+abs(cy)
            done=1
            break
    if done==1:
        break
