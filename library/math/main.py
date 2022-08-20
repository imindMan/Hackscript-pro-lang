import math


class Math:
    pi = math.pi


def pow(value):
    if (not isinstance(value[0].value, int) and not isinstance(value[0].value, float)) or (not isinstance(value[1].value, int) and not isinstance(value[1].value, float)):
        return "<code> err: InvalidObject/Somehow the objects aren't numbers"
    return value[0].value ** value[1].value


def absolute(value):
    if not isinstance(value[0].value, int) and not isinstance(value[0].value, float):
        return "<code> err: InvalidObject/Somehow the object is not a number"
    return abs(value[0].value)


def nround(value):
    if not isinstance(value[0].value, int) and not isinstance(value[0].value, float):
        return "<code> err: InvalidObject/Somehow the object is not a number"
    return round(value[0].value)


def nfloor(value):
    if not isinstance(value[0].value, int) and not isinstance(value[0].value, float):
        return "<code> err: InvalidObject/Somehow the object is not a number"
    return math.floor(value[0].value)


def nupdate_floor(value):
    if not isinstance(value[0].value, int) and not isinstance(value[0].value, float):
        return "<code> err: InvalidObject/Somehow the object is not a number"
    return int(value[0].value)


def nsin(value):
    if not isinstance(value[0].value, int) and not isinstance(value[0].value, float):
        return "<code> err: InvalidObject/Somehow the object is not a number"
    return math.sin(value[0].value)


def nsinh(value):
    if not isinstance(value[0].value, int) and not isinstance(value[0].value, float):
        return "<code> err: InvalidObject/Somehow the object is not a number"
    return math.sinh(value[0].value)


def ncos(value):
    if (not isinstance(value[0].value, int)) and (not isinstance(value[0].value, float)):
        return "<code> err: InvalidObject/Somehow the object is not a number"
    return math.cos(value[0].value)


def ncosh(value):
    if not isinstance(value[0].value, int) and not isinstance(value[0].value, float):
        return "<code> err: InvalidObject/Somehow the object is not a number"
    return math.cosh(value[0].value)


def ntan(value):
    if not isinstance(value[0].value, int) and not isinstance(value[0].value, float):
        return "<code> err: InvalidObject/Somehow the object is not a number"
    return math.tan(value[0].value)


def ntanh(value):
    if not isinstance(value[0].value, int) and not isinstance(value[0].value, float):
        return "<code> err: InvalidObject/Somehow the object is not a number"
    return math.tanh(value[0].value)


def nlog(value):
    if not isinstance(value[0].value, int) and not isinstance(value[0].value, float) or not isinstance(value[1].value, int) and not isinstance(value[1].value, float):
        return "<code> err: InvalidObject/Somehow the objects aren't numbers"
    return math.log(value[0].value, value[1].value)


def cuberoot(value):
    if not isinstance(value[0].value, int) and not isinstance(value[0].value, float):
        return "<code> err: InvalidObject/Somehow the object is not a number"
    if value[0].value < 0:
        value[0].value = abs(value[0].value)
        cube_root = value[0].value**(1/3)*(-1)
    else:
        cube_root = value[0].value**(1/3)
    return cube_root


def square(value):
    if not isinstance(value[0].value, int) and not isinstance(value[0].value, float):
        return "<code> err: InvalidObject/Somehow the object is not a number"
    return value[0].value * value[0].value


def cube(value):
    if not isinstance(value[0].value, int) and not isinstance(value[0].value, float):
        return "<code> err: InvalidObject/Somehow the object is not a number"
    return value[0].value * value[0].value * value[0].value


def nhex(value):
    if not isinstance(value[0].value, int) and not isinstance(value[0].value, float):
        return "<code> err: InvalidObject/Somehow the object is not a number"
    return hex(value[0].value)
