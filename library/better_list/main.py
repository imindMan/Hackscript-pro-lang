def slice_part(value):
    return value[0].value[value[1].value:value[2].value]


def change_slice(value):
    value[0].value[value[1].value:value[2].value] = value[3]


def append(value):
    return value[0].value.append(value[1])


def insert(value):
    value[0].value.insert(value[1].value, value[2])


def remove(value):
    value[0].value.remove(value[1])


def reind(value):
    value[0].value.pop(value[1].value)


def empty(value):
    value[0].value.clear()


def sort(value):
    value[0].value.sort()


def sort_rev(value):
    value[0].value.sort(reverse=True)


def rev(value):
    value[0].value.reverse()
