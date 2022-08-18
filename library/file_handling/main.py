import os


def create(value):
    try:
        with open(value[0].value, "x") as f:
            pass
    except:
        return "<code> err: InvalidObject/The file is existed"


def read(value):
    try:
        with open(value[0].value, "r") as f:
            return f.read()
    except FileNotFoundError:
        return "<code> err: InvalidObject/Undefined file"


def write(value):
    try:
        with open(value[0].value, "w") as f:
            f.write(value[1].value)
    except FileNotFoundError:
        return "<code> err: InvalidObject/Undefined file"


def delete_file(value):
    try:

        os.remove(value[0].value)
    except FileNotFoundError:
        return "<code> err: InvalidObject/Undefined file"


def read_char(value):
    try:
        with open(value[0].value, "r") as f:
            return f.read(value[1].value)
    except FileNotFoundError:
        return "<code> err: InvalidObject/Undefined file"


def read_line(value):
    try:
        with open(value[0].value, "r") as f:

            return f.readline()
    except FileNotFoundError:
        return "<code> err: InvalidObject/Undefined file"
