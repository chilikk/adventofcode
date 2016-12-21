import sys, re

z = "fbgdceah"

lines = sys.stdin.readlines()

for line in lines[::-1]:
    if line.startswith("swap position"):
        match = re.match(r"swap position (\d*) with position (\d*)", line)
        i1 = int(match.group(1))
        i2 = int(match.group(2))
        x = z[i1]
        z = z[:i1] + z[i2] + z[i1+1:]
        z = z[:i2] + x + z[i2+1:]
    elif line.startswith("swap letter"):
        match = re.match(r"swap letter ([a-z]) with letter ([a-z])", line)
        i1 = z.find(match.group(1))
        i2 = z.find(match.group(2))
        x = z[i1]
        z = z[:i1] + z[i2] + z[i1+1:]
        z = z[:i2] + x + z[i2+1:]
    elif line.startswith("reverse positions"):
        match = re.match(r"reverse positions (\d*) through (\d*)", line)
        i1 = int(match.group(1))
        i2 = int(match.group(2))
        imin = min(i1, i2)
        imax = max(i1, i2)
        z = z[:imin] + z[imax:imin:-1] + z[imin] + z[imax+1:]
    elif line.startswith("rotate left"):
        match = re.match(r"rotate left (\d*) step", line)
        x = int(match.group(1))
        x = (-x) % len(z)
        z = z[x:] + z[:x]
    elif line.startswith("rotate right"):
        match = re.match(r"rotate right (\d*) step", line)
        x = int(match.group(1))
        x = x % len(z)
        z = z[x:] + z[:x]
    elif line.startswith("move position"):
        match = re.match(r"move position (\d*) to position (\d*)", line)
        i2 = int(match.group(1))
        i1 = int(match.group(2))
        x = z[i1]
        z = z[:i1]+z[i1+1:]
        z = z[:i2]+x+z[i2:]
    elif line.startswith("rotate based on position"):
        match = re.match(r"rotate based on position of letter ([a-z])", line)
        l = match.group(1)
        for i1 in range(0,len(z)):
            z1 = z[i1:] + z[:i1]
            x = z1.find(l)
            x = 1 + x + (1 if x >= 4 else 0)
            x = (-x) % len(z)
            if z == z1[x:] + z1[:x]:
                z = z1
                break
    print line.strip(), z
