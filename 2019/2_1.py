import numpy

class Prog(object):
    def __init__(self, txt):
        self.vector = numpy.loadtxt(txt, delimiter=',', dtype=int)
        self.aux = {}
        self.max = len(self.vector)
        self.index = 0

    def get(self, index):
        if index >= 0 and index < self.max:
            return self.vector[index]
        else:
            return self.aux[index]

    def set(self, index, val):
        if index >= 0 and index < self.max:
            self.vector[index] = val
        else:
            self.aux[index] = val

    def cmd(self):
        cmd1 = self.get(self.index)
        if cmd1 == 99:
            return None
        else:
            cmd = (self.get(self.index), self.get(self.index+1),
                   self.get(self.index+2), self.get(self.index+3))
            self.index += 4
            return cmd 

    def eval(self):
        cmd = self.cmd()
        if cmd is None:
            return False
        if cmd[0] == 1:
            self.set(cmd[3], self.get(cmd[1])+self.get(cmd[2]))
        elif cmd[0] == 2:
            self.set(cmd[3], self.get(cmd[1])*self.get(cmd[2]))
        else:
            raise(Exception())
        return True

    def eval_loop(self):
        while self.eval():
            pass
        return self.get(0)

prog = Prog("2.txt")
prog.set(1, 12)
prog.set(2, 2)
out = prog.eval_loop()
print(out)
print(prog.vector)
print(prog.aux)
