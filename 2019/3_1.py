import numpy

def parse(start, line):
    cur = start
    segment = [cur]
    for segmentstr in line.split(','):
        cur = parse1(cur, segmentstr)
        segment.append(cur)
    return segment

def parse1(cur, segment):
    x, y = cur
    if segment[0] == 'R':
        return (x+int(segment[1:]), y)
    elif segment[0] == 'L':
        return (x-int(segment[1:]), y)
    elif segment[0] == 'U':
        return (x, y+int(segment[1:]))
    elif segment[0] == 'D':
        return (x, y-int(segment[1:]))
    else:
        raise Exception()

def norm(segment_start, segment_stop):
    x1, y1 = segment_start
    x2, y2 = segment_stop
    shift = (-x1, -y1)
    slen = math.sqrt((x2-x1)**2 + (y2-y1)**2)
    rotation = 



center = (0,0)
with open("3.txt") as f:
    segment1 = parse(center, f.readline().strip())
    segment2 = parse(center, f.readline().strip())
print(segment1)
print(segment2)

