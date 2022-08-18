def slice_part(value):
    if not isinstance(value[0].value, list):
        return "<code> err: InvalidObject/The object is not a list"
    return value[0].value[value[1].value:value[2].value]


def change_slice(value):
    if not isinstance(value[0].value, list):
        return "<code> err: InvalidObject/The object is not a list"
    value[0].value[value[1].value:value[2].value] = value[3]


def append(value):
    if not isinstance(value[0].value, list):
        return "<code> err: InvalidObject/The object is not a list"
    return value[0].value.append(value[1])


def insert(value):
    if not isinstance(value[0].value, list):
        return "<code> err: InvalidObject/The object is not a list"
    value[0].value.insert(value[1].value, value[2])


def remove(value):
    if not isinstance(value[0].value, list):
        return "<code> err: InvalidObject/The object is not a list"
    value[0].value.remove(value[1])


def reind(value):
    if not isinstance(value[0].value, list):
        return "<code> err: InvalidObject/The object is not a list"
    value[0].value.pop(value[1].value)


def empty(value):
    if not isinstance(value[0].value, list):
        return "<code> err: InvalidObject/The object is not a list"
    value[0].value.clear()


def sort(value):
    if not isinstance(value[0].value, list):
        return "<code> err: InvalidObject/The object is not a list"
    temp_list = sorted(list(map(lambda x: x.value, value[0].value)))
    for i in range(len(temp_list)):
        value[0].value[i].value = temp_list[i]
    return value[0].value


def sort_rev(value):
    if not isinstance(value[0].value, list):
        return "<code> err: InvalidObject/The object is not a list"
    temp_list = sorted(
        list(map(lambda x: x.value, value[0].value)), reverse=True)
    for i in range(len(temp_list)):
        value[0].value[i].value = temp_list[i]
    return value[0].value


def rev(value):
    if not isinstance(value[0].value, list):
        return "<code> err: InvalidObject/The object is not a list"
    value[0].value.reverse()
