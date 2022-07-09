import string

INT_TYPE = "int"
FLOAT_TYPE = "float"
POINTER = "pointer"

PLUS_OPE = "plus"
MINUS_OPE = "minus"
MULT_OPE = "mult"
DIV_OPE = "div"
POW_OPE = "pow"
SQRT_OPE = "sqrt"
SLASH = "slash"
THEN = "then"

KEYWORD = "keyword"
IDENTIFIER = "identifier"
COMMA = "comma"

LEFT_PAREN = "left_paren"
RIGHT_PAREN = "right_paren"

EOF_TYPE = "eof"
# SYMBOLS = r"`~!@#$%^&*()–_=+[]{}\|;:‘“,./<>?"
SYMBOLS = r"<>-=!"
LETTERS = string.ascii_letters
DIGITS = "0123456789"

KEYWORDS = {
    # "!": "change_status",
    "<-": "assign",
    "and": "and",
    "or": "or",
    "not": "not",
    # ".": "end",
    # "$": "launch",
    "c": "curr_data",
    "->": "assign_out",
    "=": "equal",
    "<": "less",
    ">": "greater",
    "<=": "gre_equ",
    ">=": "less_equ",
    "!=": "not_equ",
    "check": "check",
    "while": "while",
    "do": "do",
    "inst": "set_instruction",
}
