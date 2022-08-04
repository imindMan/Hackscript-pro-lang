import math


def absolute(value):
    return abs(value[0].value)


def nround(value):
    return round(value[0].value)


def nfloor(value):
    return math.floor(value[0].value)


def nsin():
    return math.sin()


def nsinh():
    return math.sinh()


def ncos():
    return math.cos()


def ncosh():
    return math.cosh()


def ntan():
    return math.tan()


def ntanh():
    return math.tanh()


def nlog(value):
    return math.log(value[0].value, value[1].value)


def cuberoot(value):
    if value[0].value < 0:
        value[0].value = abs(value[0].value)
        cube_root = value[0].value**(1/3)*(-1)
    else:
        cube_root = value[0].value**(1/3)
    return cube_root


def square(value):
    return value[0].value * value[0].value


def cube(value):
    return value[0].value * value[0].value * value[0].value


def hex(value):
    return hex(value[0].value)
