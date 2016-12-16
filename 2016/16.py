def extend(s):
    s1 = s + "0"
    for d in s[::-1]:
        s1 += "0" if d == '1' else "1"
    return s1

init = "10111100110001111"
#xlen = 272
xlen = 35651584
while len(init) < xlen:
    init = extend(init)
init = init[:xlen]
checksum = ""
while len(init) % 2 == 0:
    for i in range(0, len(init)/2):
        pair=init[2*i:2*i+2]
        checksum += "1" if pair[0]==pair[1] else "0"
    init = checksum
    checksum = ""
print init
