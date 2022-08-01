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
        res = self.statements(datatypes.EOF_TYPE)
        if not res.error and self.curr_token.type != datatypes.EOF_TYPE:
            res.failure(error.SyntaxError(
                self.curr_token.pos_start, self.curr_token.pos_end,
                "Expected '+', '-', '*', '/', '=', '<', '>', '<=', '>=', '!=', '<-', ^, `, and, or, instruction, method"
            ))

        return res

    def statements(self, keyword, list_of_attribute=None):
        res = ParserResult()
        token = self.curr_token
        list_of_statements = []
        while self.curr_token.type == datatypes.NEWLINE:
            res.register(self.advance())

        expr = res.register(self.expr(list_of_attribute))
        if res.error:
            return res
        list_of_statements.append(expr)

        while True:
            while self.curr_token.type == datatypes.NEWLINE:
                res.register(self.advance())

            if self.curr_token.type == keyword:
                break
            else:
                expr2 = res.register(self.expr(list_of_attribute))
                if res.error:
                    return res
                list_of_statements.append(expr2)
        return res.success(StatementNode(token, list_of_statements))

    def atom(self, list_of_attribute=None):
        res = ParserResult()
        token = self.curr_token
        if token.type in (datatypes.INT_TYPE, datatypes.FLOAT_TYPE):
            res.register(self.advance())
            num = NumberNode(token)
            if list_of_attribute != None:
                list_of_attribute.append(num)
            return res.success(num)
        elif token.type in (datatypes.IDENTIFIER, ):
            return self.identifier(list_of_attribute)
        elif token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["phd"]):
            res.register(self.advance())
            return res.success(PlaceHolderNode(token))
        elif token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["interrupt"]):
            res.register(self.advance())
            return res.success(InterruptNode(token))
        elif token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["pass"]):
            res.register(self.advance())
            return res.success(PassNode(token))
        elif token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["super"]):
            list_of_para = []
            res.register(self.advance())
            if not self.curr_token.type == datatypes.LEFT_PAREN:
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected '('"
                ))
            res.register(self.advance())
            if self.curr_token.type == datatypes.RIGHT_PAREN:
                res.register(self.advance())
                return res.success(SuperNode(token, list_of_para))
            para = res.register(self.atom())
            list_of_para.append(para)
            pass_loop = False
            while self.curr_token.type == datatypes.COMMA:
                pass_loop = True
                res.register(self.advance())
                para = res.register(self.atom())
                list_of_para.append(para)
            if pass_loop:
                if not self.curr_token.type == datatypes.RIGHT_PAREN:
                    return res.failure(error.SyntaxError(
                        self.curr_token.pos_start, self.curr_token.pos_end,
                        "Expected ')'"
                    ))
                res.register(self.advance())
                return res.success(SuperNode(token, list_of_para))
            else:
                if not self.curr_token.type == datatypes.RIGHT_PAREN:
                    return res.failure(error.SyntaxError(
                        self.curr_token.pos_start, self.curr_token.pos_end,
                        "Expected ')'"
                    ))
                res.register(self.advance())
                return res.success(SuperNode(token, list_of_para))

        elif token.type == datatypes.LEFT_PAREN:
            res.register(self.advance())
            expr = res.register(self.statements(
                datatypes.RIGHT_PAREN, list_of_attribute))
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
            return self.check(list_of_attribute)
        elif token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["while"]) or token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["do"]):
            return self.while_loop(list_of_attribute)
        elif token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["inst"]):
            ins = self.ins(list_of_attribute)
            return ins
        elif token.type == datatypes.STRING:
            return self.string(list_of_attribute)
        elif token.type == datatypes.L_CURLY:
            return self.list_(list_of_attribute)
        elif token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["class"]):
            return self.class_(list_of_attribute)
        elif token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["this"]):
            res.register(self.advance())
            if not self.curr_token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["->"]):
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected '->'"
                ))
            res.register(self.advance())
            name = res.register(self.atom(list_of_attribute))
            if self.curr_token.type == datatypes.THEN:
                res.register(self.advance())
                value = res.register(self.expr(list_of_attribute))
                if list_of_attribute != None:
                    list_of_attribute.append(AttributeNode(name, value))
                return res.success(AttributeNode(name, value))
            else:
                if list_of_attribute != None:
                    list_of_attribute.append(AttributeNode(name))
                return res.success(AttributeNode(name))
        return res.failure(error.SyntaxError(
            token.pos_start, token.pos_end,
            "Expected int or float, pointer, expression, '+', '-', '(', ')', '=', '<', '>', '<=', '>=', '!=', '<-', ':', ^, `, and, or"
        ))

    def power(self, list_of_attribute=None):
        power = self.bin_op(self.call_ins, (datatypes.POW_OPE,), self.factor)
        if list_of_attribute != None:
            list_of_attribute.append(power)
        return power

    def call_ins(self, list_of_attribute=None):
        res = ParserResult()
        arg_list = []
        atom = res.register(self.atom(list_of_attribute))
        if res.error:
            return res

        if self.curr_token.type == datatypes.ADD_PARA:
            res.register(self.advance())
            if self.curr_token.type == datatypes.ADD_PARA:
                res.register(self.advance())
            else:
                expr = res.register(self.expr(list_of_attribute))
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
                    para = res.register(self.expr(list_of_attribute))
                    arg_list.append(para)
                    if res.error:
                        return res
            call_ins = CallNode(atom, arg_list)
            if list_of_attribute is not None:
                list_of_attribute.append(call_ins)
            return res.success(call_ins)
        if list_of_attribute is not None:
            list_of_attribute.append(atom)
        return res.success(atom)

    def factor(self, list_of_attribute=None):
        res = ParserResult()
        token = self.curr_token
        if token.type in (datatypes.MINUS_OPE, datatypes.PLUS_OPE):
            res.register(self.advance())
            factor = res.register(self.factor())
            if res.error:
                return res
            ret = UnaryOpNode(token, factor)
            if list_of_attribute != None:
                list_of_attribute.append(ret)
            return res.success(ret)
        elif token.type in (datatypes.POINTER, ):
            return self.data(list_of_attribute)
        elif token.type == datatypes.SQRT_OPE:
            return self.sqrt(list_of_attribute)

        return self.power(list_of_attribute)

    def data(self, list_of_attribute=None):
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
                if list_of_attribute != None:
                    list_of_attribute.append(ret)
                return res.success(left)
            else:
                if res.error:
                    return res
                if atom.token.type == datatypes.INT_TYPE and atom.token.value == 2:
                    ret = PointerNode(token, atom)
                    if list_of_attribute != None:
                        list_of_attribute.append(ret)
                    return res.success(ret)
                elif atom.token.type == datatypes.IDENTIFIER:
                    ret = ConstantPointerNode(token, atom)
                    if list_of_attribute != None:
                        list_of_attribute.append(ret)
                    return res.success(ret)
                else:
                    return res.failure(error.InvalidObject(
                        token.pos_start, atom.pos_end, "Pointer type must be 2 or identifier"
                    ))

    def sqrt(self, list_of_attribute=None):
        res = ParserResult()
        token = self.curr_token

        if token.type in (datatypes.SQRT_OPE,):
            res.register(self.advance())
            factor = res.register(self.factor())
            if res.error:
                return res
            ret = UnaryOpNode(token, factor)
            if list_of_attribute != None:
                list_of_attribute.append(ret)
            return res.success(ret)

    def term(self, list_of_attribute=None):
        term = self.bin_op(self.factor, ((datatypes.MULT_OPE, None),
                           (datatypes.DIV_OPE, None), (datatypes.SLASH, None), ))
        if list_of_attribute != None:
            list_of_attribute.append(term)
        return term

    def expr(self, list_of_attribute=None):
        ret = self.bin_op(self.comp_expr, ((
            datatypes.KEYWORD, datatypes.KEYWORDS["and"]), (datatypes.KEYWORD, datatypes.KEYWORDS["or"])))
        if list_of_attribute != None:
            list_of_attribute.append(ret)

        return ret

    def comp_expr(self, list_of_attribute=None):
        res = ParserResult()
        token = self.curr_token
        if token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["not"]):
            res.register(self.advance())
            expr = res.register(self.arith_expr())
            if res.error:
                return res
            ret = UnaryOpNode(token, expr)
            if list_of_attribute != None:
                list_of_attribute.append(ret)
            return res.success(UnaryOpNode(token, expr))
        ans = self.bin_op(self.arith_expr, (
            (datatypes.KEYWORD, datatypes.KEYWORDS["="]),
            (datatypes.KEYWORD, datatypes.KEYWORDS["<"]),
            (datatypes.KEYWORD, datatypes.KEYWORDS[">"]),
            (datatypes.KEYWORD, datatypes.KEYWORDS["<="]),
            (datatypes.KEYWORD, datatypes.KEYWORDS[">="]),
            (datatypes.KEYWORD, datatypes.KEYWORDS["!="]),
        ))
        if list_of_attribute != None:
            list_of_attribute.append(ans)
        return ans

    def arith_expr(self, list_of_attribute=None):
        ret = self.bin_op(self.term, ((datatypes.PLUS_OPE, None), (
            datatypes.MINUS_OPE, None), (datatypes.KEYWORD, datatypes.KEYWORDS["<-"]), (datatypes.KEYWORD, datatypes.KEYWORDS["->"])))
        if list_of_attribute != None:
            list_of_attribute.append(ret)
        return ret

    def check(self, list_of_attribute=None):
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
            while self.curr_token.type == datatypes.NEWLINE:
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
            while self.curr_token.type == datatypes.NEWLINE:
                res.register(self.advance())
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
            while self.curr_token.type == datatypes.NEWLINE:
                res.register(self.advance())
            if res.error:
                return res
            if not self.curr_token.type == datatypes.RIGHT_PAREN:
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected ')'"
                ))
            res.register(self.advance())
            cond = CondNode(condition, cond_true, cond_false)
            if list_of_attribute != None:
                list_of_attribute.append(cond)
            return res.success(cond)

    def while_loop(self, list_of_attribute=None):
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
            while self.curr_token.type == datatypes.NEWLINE:
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
            while self.curr_token.type == datatypes.NEWLINE:
                res.register(self.advance())
            if not self.curr_token.type == datatypes.RIGHT_PAREN:
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected ')'"
                ))
            res.register(self.advance())
            while_ = WhileNode(condition, what_to_do)
            if list_of_attribute != None:
                list_of_attribute.append(while_)
            return res.success(while_)

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
            do_ = DoNode(cond, do)
            if list_of_attribute != None:
                list_of_attribute.append(do_)
            return res.success(do_)

    def ins(self, list_of_attribute=None):
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
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected ':'"
                ))
            res.register(self.advance())
            body_node = res.register(self.atom())
            ins = InsNode(name, arg_list, body_node)
            if list_of_attribute != None:
                list_of_attribute.append(ins)
            return res.success(ins)

    def string(self, list_of_attribute=None):
        res = ParserResult()
        token = self.curr_token
        if token.type in (datatypes.STRING, datatypes.IDENTIFIER):
            res.register(self.advance())
            string_here = token.value
            if self.curr_token.type == datatypes.L_SQUARE:

                res.register(self.advance())
                index = res.register(self.factor())
                if self.curr_token.type != datatypes.R_SQUARE:
                    return res.failure(error.InvalidIndexOfMemory(
                        self.curr_token.pos_start, self.curr_token.pos_end,
                        "Expected ']'"
                    ))
                res.register(self.advance())
                string_ = StringNode(token, string_here, index)
                if list_of_attribute != None:
                    list_of_attribute.append(string_)
                return res.success(string_)
            string_ = StringNode(token, string_here)
            if list_of_attribute != None:
                list_of_attribute.append(string_)
            return res.success(string_)

    def list_(self, list_of_attribute=None):
        res = ParserResult()
        token = self.curr_token
        list_gene = []
        if token.type == datatypes.L_CURLY:
            res.register(self.advance())
            if self.curr_token.type == datatypes.R_CURLY:
                res.register(self.advance())
                list_ = ListNode(token, list_gene)
                if list_of_attribute != None:
                    list_of_attribute.append(list_)
                return res.success(list_)
            else:
                ele_1 = res.register(self.atom())
                if res.error:
                    return res
                list_gene.append(ele_1)

                while self.curr_token.type == datatypes.COMMA:
                    res.register(self.advance())
                    para2 = res.register(self.atom())
                    if res.error:
                        return res
                    list_gene.append(para2)

                if not self.curr_token.type == datatypes.R_CURLY:
                    return res.failure(error.SyntaxError(
                        self.curr_token.pos_start, self.curr_token.pos_end,
                        "Expected '}'"
                    ))
                res.register(self.advance())
                if self.curr_token.type == datatypes.L_SQUARE:

                    res.register(self.advance())
                    index = res.register(self.factor())
                    if self.curr_token.type != datatypes.R_SQUARE:
                        return res.failure(error.InvalidIndexOfMemory(
                            self.curr_token.pos_start, self.curr_token.pos_end,
                            "Expected ']'"
                        ))
                    res.register(self.advance())
                    list_ = ListNode(token, list_gene, index)
                    if self.curr_token.type != datatypes.L_SQUARE:
                        if list_of_attribute != None:
                            list_of_attribute.append(list_)
                        return res.success(list_)
                    list_ = ListNode(token, list_gene, index)
                    while self.curr_token.type == datatypes.L_SQUARE:
                        res.register(self.advance())
                        index = res.register(self.factor())
                        if not self.curr_token.type == datatypes.R_SQUARE:
                            return res.failure(error.SyntaxError(
                                self.curr_token.pos_start, self.curr_token.pos_end,
                                "Expected ']'"
                            ))

                        res.register(self.advance())
                        list_ = MultiListNode(token, list_, index)

                    if list_of_attribute != None:
                        list_of_attribute.append(list_)
                    return res.success(list_)
                list_ = ListNode(token, list_gene)
                if list_of_attribute != None:
                    list_of_attribute.append(list_)
                return res.success(list_)

    def class_(self, list_of_attribute=None):
        res = ParserResult()
        token = self.curr_token
        list_of_attribut = []
        list_of_method = []
        parameter = []
        super_class = None
        if token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["class"]):
            res.register(self.advance())
            if self.curr_token.type == datatypes.LEFT_PAREN:
                super_class = res.register(self.atom(list_of_attribute))
            name = res.register(self.atom(list_of_attribute))
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
            while self.curr_token.type == datatypes.NEWLINE:
                res.register(self.advance())
            if not self.curr_token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["cons"]):
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected 'cons'"
                ))
            res.register(self.advance())
            if self.curr_token.type == datatypes.LEFT_PAREN:
                res.register(self.advance())
                para1 = res.register(self.atom())
                if res.error:
                    return res
                parameter.append(para1)

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
                        parameter.append(para)

                if not self.curr_token.type == datatypes.RIGHT_PAREN:
                    return res.failure(error.SyntaxError(
                        self.curr_token.pos_start, self.curr_token.pos_end,
                        "Expected ')'"
                    ))
                res.register(self.advance())

            if not self.curr_token.type == datatypes.THEN:
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected ':'"
                ))
            res.register(self.advance())
            res.register(self.atom(list_of_attribut))
            while self.curr_token.type == datatypes.NEWLINE:

                res.register(self.advance())
            if self.curr_token.matches(datatypes.KEYWORD, datatypes.KEYWORDS["method"]):
                res.register(self.advance())
                if not self.curr_token.type == datatypes.THEN:
                    return res.failure(error.SyntaxError(
                        self.curr_token.pos_start, self.curr_token.pos_end,
                        "Expected ':'"
                    ))
                res.register(self.advance())
                res.register(self.atom(list_of_method))
                res.register(self.advance())
                while self.curr_token.type == datatypes.NEWLINE:
                    res.register(self.advance())

            else:
                pass
            if not self.curr_token.type == datatypes.RIGHT_PAREN:
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected ')'"
                ))
            res.register(self.advance())

            list_of_attribut = list(
                map(lambda x: x.node, list_of_attribut))
            list_of_method = list(
                map(lambda x: x.node, list_of_method))
            class_ = ClassNode(name, list_of_attribut,
                               list_of_method, parameter, super_class)
            if list_of_attribute != None:
                list_of_attribute.append(class_)

            return res.success(class_)

    def identifier(self, list_of_attribute=None):
        res = ParserResult()
        token = self.curr_token
        res.register(self.advance())
        identifier = IdentifierNode(token)
        if self.curr_token.type == datatypes.L_SQUARE:
            res.register(self.advance())
            index = res.register(self.atom())
            if not self.curr_token.type == datatypes.R_SQUARE:
                return res.failure(error.SyntaxError(
                    self.curr_token.pos_start, self.curr_token.pos_end,
                    "Expected ']'"
                ))
            res.register(self.advance())
            identifier = IdentifierNode(token, index)
            if self.curr_token.type != datatypes.L_SQUARE:
                if list_of_attribute != None:
                    list_of_attribute.append(identifier)
                return res.success(identifier)
            identifier = IdentifierNode(token, index)
            while self.curr_token.type == datatypes.L_SQUARE:
                res.register(self.advance())
                index = res.register(self.atom())
                if not self.curr_token.type == datatypes.R_SQUARE:
                    return res.failure(error.SyntaxError(
                        self.curr_token.pos_start, self.curr_token.pos_end,
                        "Expected ']'"
                    ))
                res.register(self.advance())
                identifier = MultiIdentifierNode(identifier, index)
            if list_of_attribute != None:
                list_of_attribute.append(identifier)
        return res.success(identifier)

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
