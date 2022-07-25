import sys
import os
import main_hackscript
from hacktypes.impor_type import *
from hacktypes.datatypes import *

list_of_files = sys.argv


def checkempty(string):
    for i in string:
        if i in FULL_SYMBOLS + DIGITS + LETTERS:
            return False
    return True


if len(list_of_files) == 1:
    while True:

        user = input("hackscript>")

        if checkempty(user) == True:
            continue
        result, error = main_hackscript.run(user, "<stdin>")
        if error:
            print(error.as_string())
        else:
            print(result)

elif len(list_of_files) >= 2:
    f = open(list_of_files[1], "r")
    user = f.read()
    f.close()
    if checkempty(user) == True:
        print("File is empty")
        exit()
    else:
        result, error = main_hackscript.run(
            user, os.path.abspath(list_of_files[1]))
        if error:
            print(error.as_string())
        else:
            print("=>", Number.null)
