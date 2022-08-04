import math


def absolute(value):
    return abs(value[0].value)


def nround(value):
    return round(value[0].value)


def nfloor(value):
    return math.floor(value[0].value)


def nsin(value):
    return math.sin(value[0].value)


def nsinh(value):
    return math.sinh(value[0].value)


def ncos(value):
    return math.cos(value[0].value)


def ncosh(value):
    return math.cosh(value[0].value)


def ntan(value):
    return math.tan(value[0].value)


def ntanh(value):
    return math.tanh(value[0].value)


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
