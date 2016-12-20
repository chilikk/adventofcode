import sys

min = 0
num = 0
max = 4294967295
## input must be sorted with sort -n
for line in iter(sys.stdin.readline, b''):
    minmax = line.strip().split('-')
    if int(minmax[0]) > min:
        num += int(minmax[0])-min
    if int(minmax[1])+1>min:
        min = int(minmax[1])+1
if max >= min:
    num += max-min+1
print num
