import os


class OS:
    name = os.name
    arg = os.argv


def name():
    return os.name


def command(value):
    os.system(value[0].value)


def create_a_new_dir(value):
    os.makedirs(value[0].value, exist_ok=True)


def delete_dir(value):
    os.rmdir(value[0].value)


def list_files():
    if os.name == "nt":
        os.system("dir")

    else:
        os.system("ls")


def trace_fullpath(value):
    return os.path.abspath(value[0].value)


def exist(value):
    return os.path.exists(value[0].value)
