import re


def es(value):
    if not isinstance(value[0].value, str):
        return "<code> err: InvalidObject/The object is not a string"
    return re.escape(value[0].value)


def match(value):
    if not isinstance(value[0].value, str) or not isinstance(value[1].value, str):
        return "<code> err: InvalidObject/The object is not a string"

    try:
        full_match_or_not = value[2].value
        if full_match_or_not == 1:
            return re.fullmatch(value[0].value, value[1].value)

    except:
        return re.match(value[0].value, value[1].value)


def span(value):
    if not isinstance(value[0].value, re.Match):
        return "<code> err: InvalidObject/The parameter passed in isn't the Match object"

    return list(value[0].value.span())


def strcom(value):
    if not isinstance(value[0].value, re.Match):
        return "<code> err: InvalidObject/The parameter passed in isn't the Match object"

    return value[0].value.string


def detected(value):
    if not isinstance(value[0].value, re.Match):
        return "<code> err: InvalidObject/The parameter passed in isn't the Match object"

    return value[0].value.group()


def search(value):
    if not isinstance(value[0].value, str) or not isinstance(value[1].value, str):
        return "<code> err: InvalidObject/The object is not a string"

    return re.search(value[0].value, value[1].value)


def sub(value):
    if not isinstance(value[0].value, str) or not isinstance(value[1].value, str) or not isinstance(value[2].value, str):
        return "<code> err: InvalidObject/The object is not a string"
    try:
        sub_mode = value[4].value
        if sub_mode == 1:
            return list(re.subn(value[0].value, value[1].value, value[2].value))

    except:
        return re.sub(value[0].value, value[1].value, value[2].value)


def split(value):
    if not isinstance(value[0].value, str) or not isinstance(value[1].value, str):
        return "<code> err: InvalidObject/The object is not a string"
    return re.split(value[0].value, value[1].value)


def findall(value):
    if not isinstance(value[0].value, str) or not isinstance(value[1].value, str):
        return "<code> err: InvalidObject/The object is not a string"
    return re.findall(value[0].value, value[1].value)


def finditer(value):
    if not isinstance(value[0].value, str) or not isinstance(value[1].value, str):
        return "<code> err: InvalidObject/The object is not a string"

    return re.finditer(value[0].value, value[1].value)
