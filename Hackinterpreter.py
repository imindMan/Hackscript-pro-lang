from error import error
from hacktypes import datatypes
import ins_def
from hacktypes.impor_type import *


class Interpreter:
    def __init__(self, memory, symbol_table):
        self.list_of_memory = memory
        self.symbol_table = symbol_table

    def visit(self, node, context, value=True):
        method_name = f'visit_{type(node).__name__}'

        method = getattr(self, method_name, self.no_visit)
        return method(node, context, value)

    def no_visit(self, node, context, value=True):

        return RuntimeResult().failure(error.NoVisitError(
            node.pos_start, node.pos_end,
            "No visit_{} method defined".format(
                type(node).__name__)
        ))

    def visit_NumberNode(self, node, context, value=True):
        return RuntimeResult().success(Number(node.token.value).set_pos(node.pos_start, node.pos_end).set_context(context))

    def visit_StringNode(self, node, context, value=True):
        return RuntimeResult().success(ClassString(node.value).set_pos(node.pos_start, node.pos_end).set_context(context))

    def visit_IdentifierNode(self, node, context, value=True):
        res = RuntimeResult()
        if value:

            if self.symbol_table.get(node.token.value) is None:
                return res.failure(error.InvalidObject(
                    node.pos_start, node.pos_end,
                    "Undefined indentifier"
                ))
            elif self.symbol_table.get(node.token.value) == "cons-pointer":
                return res.success(self.list_of_memory.access_constant(node.token.value).set_pos(node.pos_start, node.pos_end))
            else:
                result = self.symbol_table.get(node.token.value)
                return res.success(result.set_pos(node.pos_start, node.pos_end))
        else:
            return res.success(Identifier(node.token.value).set_pos(node.pos_start, node.pos_end))

    def visit_BinOpNode(self, node, context, value=True):
        res = RuntimeResult()
        left = res.register(self.visit(node.left, context))
        if res.error:
            return res
        right = res.register(self.visit(node.right, context))
        if res.error:
            return res

        if node.op.type == datatypes.PLUS_OPE:

            result, error = left.added_to(right)
        elif node.op.type == datatypes.MINUS_OPE:

            result, error = left.subtracted_to(right)
        elif node.op.type == datatypes.MULT_OPE:
            result, error = left.multiplied_to(
                right)
        elif node.op.type == datatypes.DIV_OPE:
            result, error = left.divided_to(right)
        elif node.op.type == datatypes.POW_OPE:
            result, error = left.powed_to(right)
        elif node.op.type == datatypes.SLASH:
            result, error = left.fraced_to(right)
        elif (node.op.type, node.op.value) == (datatypes.KEYWORD, datatypes.KEYWORDS["<-"]):
            result, error = left.assign_from(right)
        elif (node.op.type, node.op.value) == (datatypes.KEYWORD, datatypes.KEYWORDS["="]):
            result, error = left.equ_to(right)
        elif (node.op.type, node.op.value) == (datatypes.KEYWORD, datatypes.KEYWORDS["!="]):
            result, error = left.not_equ_to(right)
        elif (node.op.type, node.op.value) == (datatypes.KEYWORD, datatypes.KEYWORDS["<="]):
            result, error = left.les_equ_to(right)
        elif (node.op.type, node.op.value) == (datatypes.KEYWORD, datatypes.KEYWORDS[">="]):
            result, error = left.gre_equ_to(right)
        elif (node.op.type, node.op.value) == (datatypes.KEYWORD, datatypes.KEYWORDS["<"]):
            result, error = left.les_to(right)
        elif (node.op.type, node.op.value) == (datatypes.KEYWORD, datatypes.KEYWORDS[">"]):
            result, error = left.gre_to(right)
        elif (node.op.type, node.op.value) == (datatypes.KEYWORD, datatypes.KEYWORDS["and"]):
            result, error = left.and_to(right)
        elif (node.op.type, node.op.value) == (datatypes.KEYWORD, datatypes.KEYWORDS["or"]):
            result, error = left.or_to(right)

        if error:
            return res.failure(error)
        else:
            return res.success(result.set_pos(node.pos_start, node.pos_end))

    def visit_UnaryOpNode(self, node, context, value=True):
        res = RuntimeResult()
        number = res.register(self.visit(node.node, context))
        if res.error:
            return res
        error = None
        if node.op_tok.type == datatypes.MINUS_OPE:
            number, error = number.multiplied_to(Number(-1))
        elif node.op_tok.type == datatypes.SQRT_OPE:
            number, error = number.sqrt()
        elif (node.op_tok.type, node.op_tok.value) == (datatypes.KEYWORD, datatypes.KEYWORDS["not"]):
            number, error = number.not_to()
        if error:
            return res.failure(error)
        else:
            return res.success(number.set_pos(node.pos_start, node.pos_end))

    def visit_PointerNode(self, node, context, value=True):
        res = RuntimeResult()
        type_pointer = res.register(self.visit(node.pointer_value, context))
        if res.error:
            return res
        self.list_of_memory.set_pos(node.pos_start, node.pos_end)
        pointer = Pointer(type_pointer, self.list_of_memory)
        return res.success(pointer.set_pos(node.pos_start, node.pos_end))

    def visit_ConstantPointerNode(self, node, context, value=True):
        res = RuntimeResult()
        name = res.register(self.visit(node.pointer_value, context, False))
        if res.error:
            return res

        self.list_of_memory.set_pos(node.pos_start, node.pos_end)
        pointer = ConstantPointer(name, self.list_of_memory)
        return res.success(pointer.set_pos(node.pos_start, node.pos_end))

    def visit_CondNode(self, node, context, value=True):
        res = RuntimeResult()
        condition = res.register(self.visit(node.condition, context))
        if res.error:
            return res
        if condition.value == 1:
            true = res.register(self.visit(node.value_if_true, context))
            return res.success(true.set_pos(node.pos_start, node.pos_end))
        elif condition.value == 0:
            false = res.register(self.visit(node.value_if_false, context))
            return res.success(false.set_pos(node.pos_start, node.pos_end))

    def visit_WhileNode(self, node, context, value=True):
        res = RuntimeResult()

        while True:
            condition = res.register(self.visit(node.condition, context))
            if res.error:
                return res

            if condition.value != 1:
                break

            res.register(self.visit(node.do, context))
            if res.error:
                return res
        return res.success(Number.null)

    def visit_DoNode(self, node, context, value=True):
        res = RuntimeResult()
        while True:
            res.register(self.visit(node.do, context))
            if res.error:
                return res
            condition = res.register(self.visit(node.condition, context))
            if res.error:
                return res
            if condition.value != 1:
                break

        return res.success(Number.null)

    def visit_InsNode(self, node, context, value=True):
        res = RuntimeResult()
        ins_name = res.register(self.visit(
            node.name, context, False))

        if res.error:
            return res
        args_list = [arg_name.token for arg_name in node.args]
        body = node.body

        ins_value = ins_def.Instruction(
            ins_name.__repr__(), args_list, body, self.list_of_memory).set_pos(node.pos_start, node.pos_end).set_context(context)
        self.symbol_table.set(ins_name.__repr__(), ins_value)
        return res.success(ins_value)

    def visit_CallNode(self, node, context, value=True):
        res = RuntimeResult()
        args = []

        value_to_call = res.register(self.visit(node.name, context))

        if res.error:
            return res

        value_to_call = value_to_call.copy().set_pos(node.pos_start, node.pos_end)

        for arg_node in node.args:
            args.append(res.register(self.visit(arg_node, context)))
            if res.error:
                return res

        return_value = res.register(value_to_call.execute(args))
        if res.error:
            return res
        return_value = return_value.copy().set_pos(
            node.pos_start, node.pos_end).set_context(context)
        return res.success(return_value)
