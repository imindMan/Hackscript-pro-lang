def display(value):
    if len(value) == 1:
        print(value[0])
    else:
        print(value[0], end=value[1].value)


def chr():
    print("""Fundametal of HackScript:
HackScript is a Turing-complete, esoteric-style, the \"easy Malbolge\", and high-level programming language.
Rules of HackScript: (or Theory of HackScript)
    Note: By following the rules, HackScript makes its own style of code, known as HackScript-like syntax
#1: Esoteric-style mixs with high-level-style become HackScript
#2: Completion can be done with replacement 
#3: No complete support, instead, deal with everything in HackScript, with HackScript no way escape.
#4: No empty code blocks
#5: No curly braces and semicolons (C-like), but instead, parentheses and semicolons
#6: Readable can be done with confusion  
#7: No shorter code, instead, deal with nested thing and long code
#8: Everything has to be organized, meaning it should be wrapped in parentheses
#9: Everything, as the name of \"Hack\", is a challenge in HackScript, meaning error maybe doesn't show to you 
#10: The purpose of HackScript is to be a fun programming language with no way escape (like esotoric), but also 
    support high-level features to make it useful at some points.
Happy coding!""")


def inside(value):
    temp_list = list(map(lambda x: x.value, value[1].value))
    if value[0].value in temp_list:
        return True
    return False
