import numpy

def fuel(array):
    array = array[array>0]
    if len(array) == 0:
        return 0
    return numpy.sum(array) + fuel(numpy.floor(array/3)-2)

vector = numpy.loadtxt("1.txt")
print(int(fuel(vector)-numpy.sum(vector)))
