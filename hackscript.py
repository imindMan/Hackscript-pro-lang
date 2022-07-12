import sys
import os
import main_hackscript


list_of_files = sys.argv


if len(list_of_files) == 1:
    while True:

        user = input("hackscript>")

        if user in " \t\n":
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
    if user == "":
        print("File is empty")
    else:
        if user in " \t\n":
            pass
        elif user == "exit":
            exit()
        result, error = main_hackscript.run(user, "<stdin>")
        if error:
            print(error.as_string())
        else:
            print(result)
