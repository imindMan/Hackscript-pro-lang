import hacktypes.datatypes as datatypes
from error import error
from hacktypes.impor_type import *


class ParserResult:
    def __init__(self):
        self.error = None
        self.node = None

    def register(self, res):
        if isinstance(res, ParserResult):
            if res.error:
                self.error = res.error
            return res.node

        return res

    def success(self, node):
        self.node = node
        return self

    def failure(self, error):
        self.error = error
        return self


class Parser:
    def __init__(self, tokens):
        self.tokens = tokens
        self.curr_idx = -1
        self.advance()

    def advance(self):
        self.curr_idx += 1
        if self.curr_idx < len(self.tokens):
            self.curr_token = self.tokens[self.curr_idx]
        return self.curr_token

    def parse(self):
        res = self.expr()
        if not res.error and self.curr_token.type != datatypes.EOF_TYPE:
            res.failure(error.SyntaxError(
                self.curr_token.pos_start, self.curr_token.pos_end,
                "Expected '+', '-', '*', '/', '=', '<', '>', '<=', '>=', '!=', '<-', ^, `, and, or"
            ))

        return res

    def atom(self):
        res = ParserResult()
        token = self.curr_token
        if token.type in (datatypes.INT_TYPE, datatypes.FLOAT_TYPE):
            res.register(self.advance())
            return res.success(NumberNode(token))
        elif token.type in (datatypes.IDENTIFIER, ):
            res.register(self.advance())
            return res.success(IdentifierNode(token))
        elif token.type == datatypes.LEFT_PAREN:
            res.register(self.advance())
            expr = res.register(self.expr())
            if res.error:
                return res
            if self.curr_token.type == datatypes.RIGHT_PAREN:
                res.register(self.advance())
                return res.success(expr)
            else:
                return res.failure(
                    error.SyntaxError(
                        self.curr_token.pos_start, self.curr_token.pos_end,
                        "Expected ')'"
                    )
                )
        elif token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["check"]):
            return self.check()
        elif token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["while"]) or token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["do"]):
            return self.while_loop()
        elif token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["inst"]):
            return self.ins()

        return res.failure(error.SyntaxError(
            token.pos_start, token.pos_end,
            "Expected int or float, pointer, expression, '+', '-', '(', ')', '=', '<', '>', '<=', '>=', '!=', '<-', ':', ^, `, and, or"
        ))

    def power(self):
        return self.bin_op(self.call_ins, (datatypes.POW_OPE,), self.factor)

    def call_ins(self):
        res = ParserResult()
        arg_list = []
        atom = res.register(self.atom())
        if res.error:
            return res

        if self.curr_token.type == datatypes.ADD_PARA:
            res.register(self.advance())
            if self.curr_token.type == datatypes.ADD_PARA:
                res.register(self.advance())
            else:
                expr = res.register(self.expr())
                arg_list.append(expr)
                if res.error:
                    return res.failure(
                        error.SyntaxError(
                            self.curr_token.pos_start, self.curr_token.pos_end,
                            "Expected int or float, pointer, expression, '+', '-', '(', ')', '=', '<', '>', '<=', '>=', '!=', '<-', ':', ^, `, and, or"
                        )
                    )
                while self.curr_token.type == datatypes.COMMA:
                    res.register(self.advance())
                    para = res.register(self.expr())
                    arg_list.append(para)
                    if res.error:
                        return res
            return res.success(CallNode(atom, arg_list))

        return res.success(atom)

    def factor(self):
        res = ParserResult()
        token = self.curr_token
        if token.type in (datatypes.MINUS_OPE, datatypes.PLUS_OPE):
            res.register(self.advance())
            factor = res.register(self.factor())
            if res.error:
                return res
            return res.success(UnaryOpNode(token, factor))
        elif token.type in (datatypes.POINTER, ):
            return self.data()
        elif token.type == datatypes.SQRT_OPE:
            return self.sqrt()

        return self.power()

    def data(self):
        res = ParserResult()
        token = self.curr_token

        if token.type == datatypes.POINTER:
            res.register(self.advance())
            atom = res.register(self.atom())
            if res.error:
                return res
            if self.curr_token.type in (datatypes.PLUS_OPE, datatypes.MINUS_OPE, ):
                left = PointerNode(token, atom)
                while self.curr_token.type in (datatypes.PLUS_OPE, datatypes.MINUS_OPE, ):
                    op = self.curr_token
                    res.register(self.advance())

                    right = res.register(self.factor())
                    if res.error:
                        return res
                    left = BinOpNode(left, op, right)
                return res.success(left)
            else:
                if res.error:
                    return res
                if atom.token.type == datatypes.INT_TYPE and atom.token.value == 2:
                    return res.success(PointerNode(token, atom))
                elif atom.token.type == datatypes.IDENTIFIER:
                    return res.success(ConstantPointerNode(token, atom))
                else:
                    return res.failure(error.UndefinedObject(
                        token.pos_start, atom.pos_end, "Pointer type must be 2 or identifier"
                    ))

    def sqrt(self):
        res = ParserResult()
        token = self.curr_token

        if token.type in (datatypes.SQRT_OPE,):
            res.register(self.advance())
            factor = res.register(self.factor())
            if res.error:
                return res
            return res.success(UnaryOpNode(token, factor))

    def term(self):
        return self.bin_op(self.factor, ((datatypes.MULT_OPE, None), (datatypes.DIV_OPE, None), (datatypes.SLASH, None), ))

    def expr(self):

        return self.bin_op(self.comp_expr, ((datatypes.KEYWORD, datatypes.KEYWORDS["and"]), (datatypes.KEYWORD, datatypes.KEYWORDS["or"])))

    def comp_expr(self):
        res = ParserResult()
        token = self.curr_token
        if token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["not"]):
            res.register(self.advance())
            expr = res.register(self.arith_expr())
            if res.error:
                return res
            return res.success(UnaryOpNode(token, expr))

        return self.bin_op(self.arith_expr, (
            (datatypes.KEYWORD, datatypes.KEYWORDS["="]),
            (datatypes.KEYWORD, datatypes.KEYWORDS["<"]),
            (datatypes.KEYWORD, datatypes.KEYWORDS[">"]),
            (datatypes.KEYWORD, datatypes.KEYWORDS["<="]),
            (datatypes.KEYWORD, datatypes.KEYWORDS[">="]),
            (datatypes.KEYWORD, datatypes.KEYWORDS["!="]),
        ))

    def arith_expr(self):
        return self.bin_op(self.term, ((datatypes.PLUS_OPE, None), (datatypes.MINUS_OPE, None), (datatypes.KEYWORD, datatypes.KEYWORDS["<-"])))

    def check(self):
        res = ParserResult()
        token = self.curr_token

        if token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["check"]):
            res.register(self.advance())
            condition = res.register(self.atom())
            if res.error:
                return res

            if not self.curr_token.type == datatypes.THEN:
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected ':'"
                ))
            res.register(self.advance())
            if res.error:
                return res
            if not self.curr_token.type == datatypes.LEFT_PAREN:
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected '('"
                ))
            if res.error:
                return res
            res.register(self.advance())
            if not self.curr_token.matches(datatypes.IDENTIFIER, "true"):
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected true condition"
                ))
            res.register(self.advance())

            if res.error:
                return res
            if not self.curr_token.type == datatypes.THEN:
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected ':'"
                ))
            res.register(self.advance())
            cond_true = res.register(self.atom())
            if res.error:
                return res

            if not self.curr_token.matches(datatypes.IDENTIFIER, "false"):
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected false condition"
                ))
            res.register(self.advance())

            if res.error:
                return res
            if not self.curr_token.type == datatypes.THEN:
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected ':'"
                ))
            res.register(self.advance())
            cond_false = res.register(self.atom())
            if res.error:
                return res
            if not self.curr_token.type == datatypes.RIGHT_PAREN:
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected ')'"
                ))
            res.register(self.advance())
            return res.success(CondNode(condition, cond_true, cond_false))

    def while_loop(self):
        res = ParserResult()
        token = self.curr_token

        if token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["while"]):
            res.register(self.advance())
            condition = res.register(self.atom())
            if res.error:
                return res

            if not self.curr_token.type == datatypes.THEN:

                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected ':'"
                ))

            res.register(self.advance())
            if not self.curr_token.type == datatypes.LEFT_PAREN:
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected '('"
                ))
            res.register(self.advance())
            if not self.curr_token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["do"]):
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected 'do'"
                ))

            res.register(self.advance())
            if not self.curr_token.type == datatypes.THEN:
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected ':'"
                ))

            res.register(self.advance())
            what_to_do = res.register(self.atom())
            if not self.curr_token.type == datatypes.RIGHT_PAREN:
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected ')'"
                ))
            res.register(self.advance())
            return res.success(WhileNode(condition, what_to_do))

        elif token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["do"]):
            res.register(self.advance())
            if not self.curr_token.type == datatypes.THEN:
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected ':'"
                ))
            res.register(self.advance())
            do = res.register(self.atom())

            if not self.curr_token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["while"]):
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected 'while'"
                ))
            res.register(self.advance())
            cond = res.register(self.atom())
            return res.success(DoNode(cond, do))

    def ins(self):
        res = ParserResult()
        token = self.curr_token
        arg_list = []
        if token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["inst"]):
            res.register(self.advance())
            if not self.curr_token.type == datatypes.IDENTIFIER:
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected identifier"
                ))
            name = res.register(self.atom())
            if res.error:
                return res
            if not self.curr_token.type == datatypes.LEFT_PAREN:
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected '('"
                ))
            res.register(self.advance())

            if self.curr_token.type == datatypes.IDENTIFIER:

                para1 = res.register(self.atom())
                if res.error:
                    return res
                arg_list.append(para1)

                while self.curr_token.type == datatypes.COMMA:

                    res.register(self.advance())
                    if not self.curr_token.type == datatypes.IDENTIFIER:
                        return res.failure(error.SyntaxError(
                            self.curr_token.pos_start, self.curr_token.pos_end,
                            "Expected identifier"
                        ))
                    else:
                        para = res.register(self.atom())
                        if res.error:
                            return res
                        arg_list.append(para)

                if not self.curr_token.type == datatypes.RIGHT_PAREN:
                    return res.failure(error.SyntaxError(
                        self.curr_token.pos_start, self.curr_token.pos_end,
                        "Expected ')'"
                    ))

            else:

                if not self.curr_token.type == datatypes.RIGHT_PAREN:
                    return res.failure(error.SyntaxError(
                        self.curr_token.pos_start, self.curr_token.pos_end,
                        "Expected ')'"
                    ))
            res.register(self.advance())
            if not self.curr_token.type == datatypes.THEN:
                return res.failure(error.SyntaxError(
                    "Expected ':'"
                ))
            res.register(self.advance())
            body_node = res.register(self.atom())
            return res.success(InsNode(name, arg_list, body_node))

    def bin_op(self, func, ops, func2=None):
        if func2 == None:
            func2 = func

        res = ParserResult()
        left = res.register(func())
        if res.error:
            return res
        while (self.curr_token.type, self.curr_token.value) in ops:
            op = self.curr_token
            res.register(self.advance())

            right = res.register(func2())
            if res.error:
                return res
            left = BinOpNode(left, op, right)
        return res.success(left)
