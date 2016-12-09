#!/usr/bin/env python

import sys, re

def find_abba(line):
    for i in range(0, len(line)-3):
        x = line[i:i+4]
        if x[0]==x[3] and x[1]==x[2] and x[0]!=x[1]:
            return True
    return False

def find_aba(line):
    list_abas = []
    for i in range(0, len(line)-2):
        x = line[i:i+3]
        if x[0]==x[2] and x[0]!=x[1]:
            list_abas += [(x[0], x[1])]
    return list_abas

def find_bab(line, list_abas):
    for i in range(0, len(line)-2):
        x = line[i:i+3]
        for a, b in list_abas:
            if x[0]==b and x[1]==a and x[2]==b:
                return True
    return False

ctr = 0
ctr2 = 0
for ip in iter(sys.stdin.readline, b''):
    ip = ip.strip()
    print "Line: ", ip
    no_abba_inside = True
    list_abas = []
    for inside in re.findall(r'\[([a-z]*)\]', ip):
        print "Inside: ", inside
        no_abba_inside = no_abba_inside and not find_abba(inside)
        list_abas += find_aba(inside)
    abba_outside = False
    bab_outside = False
    before = re.findall(r"^([a-z]*)\[", ip)[0]
    after = re.findall(r"\]([a-z]*)$", ip)[0]
    print "Before: ", before
    print "After: ", after
    abba_outside = abba_outside or find_abba(before) or find_abba(after)
    bab_outside = bab_outside or find_bab(before, list_abas) or find_bab(after, list_abas)
    for outside in re.findall(r"\]([a-z]*)\[", ip):
        print "Outside: ", outside
        abba_outside = abba_outside or find_abba(outside)
        bab_outside = bab_outside or find_bab(outside, list_abas)
    if abba_outside and no_abba_inside:
        ctr += 1
    if bab_outside:
        ctr2 += 1
print ctr
print ctr2
