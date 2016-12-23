import sys
codetxt = sys.stdin.readlines()
code = []
## easy to see that the input program calculates a!+N
## so the case for a=12 can be calculated using the formula
vars = {"a":7, "b":0, "c":0, "d":0}
ops = ['cpy', 'inc', 'dec', 'jnz', 'tgl']
p = -1

def vget(x):
    global vars
    if x in vars:
        return vars[x]
    else:
        return x

for line in codetxt:
    t = line.strip().split(" ")
    if not t[0] in ops:
        raise "not an op"
    op = t[0]
    a1 = int(t[1]) if not t[1] in vars else t[1]
    if len(t)<3:
        a2 = None
    else:
        a2 = int(t[2]) if not t[2] in vars else t[2]
    code.append((op,a1,a2))

while p<len(code)-1:
    p+=1
    op, a1, a2 = code[p]
    if op == "cpy" and a2 in vars:
        vars[a2] = vget(a1)
    elif op == "inc" and a1 in vars:
        vars[a1] += 1
    elif op == "dec" and a1 in vars:
        vars[a1] -= 1
    elif op == "jnz" and vget(a1) != 0:
        p += vget(a2)-1
    elif op == "tgl":
        print p, code[p], vars
        p1 = p+vget(a1)
        if p1 < 0 or p1 >= len(code):
            continue
        op1, a11, a21 = code[p1]
        op2 = { "inc":"dec",
                "dec":"inc",
                "tgl":"inc",
                "cpy":"jnz",
                "jnz":"cpy" }[op1]
        code[p1] = (op2, a11, a21)
print vars["a"]
