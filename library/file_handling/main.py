import os


def create(value):
    with open(value[0].value, "x") as f:
        pass


def read(value):
    with open(value[0].value, "r") as f:
        return f.read()


def write(value):
    with open(value[0].value, "w") as f:
        f.write(value[1].value)


def delete_file(value):
    if os.path.exists(value[0].value):
        os.remove(value[0].value)


def read_char(value):
    with open(value[0].value, "r") as f:
        return f.read(value[1].value)


def read_line(value):
    with open(value[0].value, "r") as f:

        return f.readline()
