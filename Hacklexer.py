from hacktypes import datatypes
from error import error
from hacktypes.impor_type import *


class Token:
    def __init__(self, type, value=None, pos_start=None, pos_end=None):
        self.type = type
        self.value = value
        if pos_start:
            self.pos_start = pos_start.copy()
            self.pos_end = pos_start.copy()
            self.pos_end.advance()

        if pos_end:
            self.pos_end = pos_end

    def matches(self, type_, value):
        return self.type == type_ and self.value == value

    def __repr__(self):
        return f"{self.type}:{self.value}" if self.value else self.type

# necessary classes


class Lexer:
    def __init__(self, fname, fcontent):
        self.fname = fname
        self.fcontent = fcontent

        self.position = Position(-1, 0, -1, fname, fcontent)
        self.curr_char = None
        self.advance()

    def advance(self):
        self.position.advance(self.curr_char)
        self.curr_char = self.fcontent[self.position.index] if self.position.index < len(
            self.fcontent) else None

    def make_tokens(self):
        tokens = []

        while self.curr_char is not None:
            if self.curr_char in " \t":
                self.advance()
            elif self.curr_char == "+":
                tokens.append(Token(datatypes.PLUS_OPE,
                              pos_start=self.position))
                self.advance()

            elif self.curr_char == "-":

                self.advance()
                if self.curr_char == ">":
                    tokens.append(
                        Token(datatypes.KEYWORD, datatypes.KEYWORDS["->"], pos_start=self.position))
                else:
                    tokens.append(Token(datatypes.MINUS_OPE,
                                  pos_start=self.position))
                self.advance()
            elif self.curr_char == "~":
                tokens.append(Token(datatypes.SLASH, pos_start=self.position))
                self.advance()
            elif self.curr_char == "*":
                tokens.append(Token(datatypes.MULT_OPE,
                              pos_start=self.position))
                self.advance()
            elif self.curr_char == "/":
                tokens.append(
                    Token(datatypes.DIV_OPE, pos_start=self.position))
                self.advance()
            elif self.curr_char.isdigit():
                tokens.append(self.make_number())
            elif self.curr_char == "^":
                tokens.append(Token(datatypes.POW_OPE,
                              pos_start=self.position))
                self.advance()

            elif self.curr_char == "`":
                tokens.append(Token(datatypes.SQRT_OPE,
                              pos_start=self.position))
                self.advance()

            elif self.curr_char == "(":
                tokens.append(Token(datatypes.LEFT_PAREN,
                              pos_start=self.position))
                self.advance()
            elif self.curr_char == ")":
                tokens.append(Token(datatypes.RIGHT_PAREN,
                              pos_start=self.position))
                self.advance()
            elif self.curr_char == "$":
                self.advance()
                # if self.curr_char is None or not self.curr_char.isdigit():
                #     tokens.append(
                #         Token(datatypes.KEYWORD, datatypes.KEYWORDS["$"], pos_start=self.position))
                # else:
                tokens.append(
                    Token(datatypes.POINTER, pos_start=self.position))
                continue
            elif self.curr_char == ",":
                tokens.append(Token(datatypes.COMMA, pos_start=self.position))
                self.advance()
            elif self.curr_char == ":":
                tokens.append(Token(datatypes.THEN, pos_start=self.position))
                self.advance()
            elif self.curr_char == "#":
                tokens.append(Token(datatypes.ADD_PARA,
                              pos_start=self.position))
                self.advance()
            elif self.curr_char in datatypes.LETTERS + datatypes.SYMBOLS:
                tokens.append(self.make_word())

            else:
                pos = self.position.copy()
                char = self.curr_char
                self.advance()
                return [], error.IllegalCharacter(pos, self.position, char)
        tokens.append(Token(datatypes.EOF_TYPE, pos_start=self.position))
        return tokens, None

    def make_number(self):
        num_str = ''
        dot_count = 0
        pos_start = self.position.copy()

        while self.curr_char != None and self.curr_char in "0123456789.":
            if self.curr_char == '.':
                if dot_count == 1:
                    break
                dot_count += 1
                num_str += '.'
            else:
                num_str += self.curr_char
            self.advance()

        if dot_count == 0:
            return Token(datatypes.INT_TYPE, int(num_str), pos_start, self.position)
        else:
            return Token(datatypes.FLOAT_TYPE, float(num_str), pos_start, self.position)

    def make_word(self):
        word = ""
        while self.curr_char is not None and self.curr_char in datatypes.LETTERS + datatypes.SYMBOLS:
            word += self.curr_char
            self.advance()
        if word in datatypes.KEYWORDS:
            return Token(datatypes.KEYWORD,
                         datatypes.KEYWORDS[word], pos_start=self.position)

        else:
            return Token(datatypes.IDENTIFIER, word, pos_start=self.position)
