MAGIC = 1358

def is_wall(x, y):
    global MAGIC
    if x<0 or y<0:
        return 1
    num = (x+3)*x + (2*x+y+1)*y + MAGIC
    bits = 0
    while num != 0:
        bits += num % 2
        num /= 2
    return bits % 2

class Pos:
    def __init__(self, cur, step):
        self.cur = cur
        self.step = step

pos = Pos((1,1), 0)
target = (31,39)
visited = [(1,1)]
visitedBefore50 = [(1,1)]
queue = []
while pos.cur != target:
    x, y = pos.cur
    for x1, y1 in [(x-1, y), (x+1, y), (x, y-1), (x, y+1)]:
        if not is_wall(x1, y1) and not (x1, y1) in visited:
            visited.append((x1,y1))
            if pos.step < 50:
                visitedBefore50.append((x1,y1))
            queue.append(Pos((x1,y1), pos.step+1))
    pos = queue[0]
    queue = queue[1:]
print pos.step
print len(visitedBefore50)
