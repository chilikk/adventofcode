import sys
codetxt = sys.stdin.readlines()
code = []
## first the program computes d = a+7*362
## then, for each step in the loop a = d ; b = a mod 2 ; a = a div 2; transmit b
## if a = 0, then start over
## so d should be computed as ((((1*2)*2+1)*2)*2+1)...
## the minimum d >= a is 2730
## init a is 2730-7*362 = 196
vars = {"a":196, "b":0, "c":0, "d":0}
ops = ['cpy', 'inc', 'dec', 'jnz', 'tgl', 'out']
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
    #print p, code[p], vars
    if op == "cpy" and a2 in vars:
        vars[a2] = vget(a1)
    elif op == "inc" and a1 in vars:
        vars[a1] += 1
    elif op == "dec" and a1 in vars:
        vars[a1] -= 1
    elif op == "jnz" and vget(a1) != 0:
        p += vget(a2)-1
    elif op == "out":
        print "out", vget(a1)
    elif op == "tgl":
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
