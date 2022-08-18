import codecs


def slice_part(value):
    return value[0].value[value[1].value:value[2].value]


def upper(value):
    return value[0].value.upper()


def lower(value):
    return value[0].value.lower()


def del_blank(value):
    return value[0].value.strip()


def rdel_blank(value):
    return value[0].value.rstrip()


def ldel_blank(value):
    return value[0].value.lstrip()


def replace(value):
    return value[0].value.replace(value[1].value, value[2].value)


def split(value):
    return value[0].value.split(value[1].value)


def capitalize(value):
    return value[0].value.capitalize()


def unraw(value):
    value[0].value = codecs.decode(value[0].value, 'unicode_escape')


def raw(value):
    value[0].value = '%r' % value[0].value
    value[0].value = value[0].value[1:-1]
