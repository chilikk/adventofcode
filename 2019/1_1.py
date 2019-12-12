import numpy

vector = numpy.loadtxt("1.txt")
fuel = numpy.floor(vector/3)-2
print(int(numpy.sum(fuel)))
