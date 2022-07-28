from error import error
from hacktypes import datatypes
import ins_def
from hacktypes.impor_type import *


class Interpreter:
    def __init__(self, memory, symbol_table):
        self.list_of_memory = memory
        self.symbol_table = symbol_table
        self.error_right_now = None

    def visit(self, node, context, value=True, attributes=None):
        method_name = f'visit_{type(node).__name__}'

        method = getattr(self, method_name, self.no_visit)
        return method(node, context, value, attributes)

    def no_visit(self, node, context, value=True, attributes=None):

        return RuntimeResult().failure(error.NoVisitError(
            node.pos_start, node.pos_end,
            "No visit_{} method defined".format(
                type(node).__name__)
        ))

    def visit_NumberNode(self, node, context, value=True, attributes=None):
        return RuntimeResult().success(Number(node.token.value).set_pos(node.pos_start, node.pos_end).set_context(context))

    def visit_StringNode(self, node, context, value=True, attributes=None):
        res = RuntimeResult()
        if not node.index:
            return res.success(ClassString(node.value).set_pos(node.pos_start, node.pos_end).set_context(context))
        else:
            index = res.register(self.visit(node.index, context))
            if index.value >= len(node.value) or index.value < 0:
                return res.failure(error.InvalidIndexOfMemory(
                    node.pos_start, node.pos_end,
                    "Invalid index of the string"
                ))
            node.value = node.value[index.value]
            return res.success(ClassString(node.value).set_pos(node.pos_start, node.pos_end).set_context(context))

    def visit_IdentifierNode(self, node, context, value=True, attributes=None):
        res = RuntimeResult()
        if value:

            if self.symbol_table.get(node.token.value) is None:
                return res.failure(error.InvalidObject(
                    node.pos_start, node.pos_end,
                    "Undefined indentifier"
                ))
            elif self.symbol_table.get(node.token.value) == "cons-pointer":
                return res.success(self.list_of_memory.access_constant(node.token.value).set_pos(node.pos_start, node.pos_end).set_context(context))
            elif isinstance(self.symbol_table.get(node.token.value), ClassString) and node.index:
                string_spe = self.symbol_table.get(node.token.value)
                index = res.register(self.visit(node.index, context))
                if index.value >= len(string_spe.value) or index.value < 0:
                    return res.failure(error.InvalidIndexOfMemory(
                        node.pos_start, node.pos_end,
                        "Invalid index of the string"
                    ))
                string_spe = ClassString(string_spe.value[index.value]).set_pos(
                    string_spe.pos_start, string_spe.pos_end).set_context(string_spe.context)
                return res.success(string_spe.set_pos(node.pos_start, node.pos_end).set_context(context))
            elif isinstance(self.symbol_table.get(node.token.value), List) and node.index:
                list_spe = self.symbol_table.get(node.token.value)
                index = res.register(self.visit(node.index, context))
                if index.value >= len(list_spe.value) or index.value < 0:
                    return res.failure(error.InvalidIndexOfMemory(
                        node.pos_start, node.pos_end,
                        "Invalid index of the list"
                    ))
                list_spe = List(list_spe.value[index.value]).set_context(
                    list_spe.context)
                return res.success(list_spe.set_pos(node.pos_start, node.pos_end).set_context(context))

            else:
                result = self.symbol_table.get(node.token.value)
                return res.success(result.set_pos(node.pos_start, node.pos_end).set_context(context))
        else:
            if not node.index:
                return res.success(Identifier(node.token.value).set_pos(node.pos_start, node.pos_end).set_context(context))
            else:
                index = res.register(self.visit(node.index, context))
                return res.success(Identifier(node.token.value, index).set_pos(node.pos_start, node.pos_end).set_context(context))

    def visit_BinOpNode(self, node, context, value=True, attributes=None):
        res = RuntimeResult()
        if (node.op.type, node.op.value) == (datatypes.KEYWORD, datatypes.KEYWORDS["->"]):
            left = res.register(self.visit(node.left, context))
            if res.error:
                return res
            right = res.register(self.visit(node.right, context, False))

            if res.error:
                return res
        else:
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
        elif (node.op.type, node.op.value) == (datatypes.KEYWORD, datatypes.KEYWORDS["->"]):
            try:
                result, error = left.attribute(right, right.args)
            except:
                result, error = left.attribute(right)
        if error:
            return res.failure(error)
        elif (error, result) == (None, None):
            return res.failure(self.error_right_now)
        else:
            return res.success(result.set_pos(node.pos_start, node.pos_end))

    def visit_ListNode(self, node, context, value=True, attributes=None):
        res = RuntimeResult()
        if not node.index:
            list_in = [res.register(self.visit(i, context))
                       for i in node.value]
            return res.success(List(list_in).set_pos(node.pos_start, node.pos_end).set_context(context))
        else:
            list_ = [res.register(self.visit(i, context)) for i in node.value]
            index = res.register(self.visit(node.index, context))
            if index.value >= len(node.value) or index.value < 0:
                return res.failure(error.InvalidIndexOfMemory(
                    node.pos_start, node.pos_end,
                    "Invalid index of the list"
                ))
            list_ = list_[index.value]
            return res.success(list_.set_pos(node.pos_start, node.pos_end).set_context(context))

    def visit_UnaryOpNode(self, node, context, value=True, attributes=None):
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

    def visit_PointerNode(self, node, context, value=True, attributes=None):
        res = RuntimeResult()
        type_pointer = res.register(self.visit(node.pointer_value, context))
        if res.error:
            return res
        self.list_of_memory.set_pos(node.pos_start, node.pos_end)
        pointer = Pointer(type_pointer, self.list_of_memory)
        return res.success(pointer.set_pos(node.pos_start, node.pos_end).set_context(context))

    def visit_ConstantPointerNode(self, node, context, value=True, attributes=None):
        res = RuntimeResult()
        name = res.register(self.visit(node.pointer_value, context, False))
        if res.error:
            return res

        self.list_of_memory.set_pos(node.pos_start, node.pos_end)
        pointer = ConstantPointer(name, self.list_of_memory)
        return res.success(pointer.set_pos(node.pos_start, node.pos_end).set_context(context))

    def visit_CondNode(self, node, context, value=True, attributes=None):
        res = RuntimeResult()
        condition = res.register(self.visit(node.condition, context))
        if res.error:
            return res
        if condition.value == 1:
            true = res.register(self.visit(node.value_if_true, context))
            return res.success(true.set_pos(node.pos_start, node.pos_end).set_context(context))
        elif condition.value == 0:
            false = res.register(self.visit(node.value_if_false, context))
            return res.success(false.set_pos(node.pos_start, node.pos_end).set_context(context))

    def visit_WhileNode(self, node, context, value=True, attributes=None):
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

    def visit_DoNode(self, node, context, value=True, attributes=None):
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

    def visit_InsNode(self, node, context, value=True, attributes=None):
        res = RuntimeResult()
        ins_name = res.register(self.visit(
            node.name, context, False))

        if res.error:
            return res
        args_list = [arg_name.token for arg_name in node.args]
        body = node.body

        ins_value = ins_def.Instruction(
            ins_name.__repr__(), args_list, body, self.list_of_memory).set_pos(node.pos_start, node.pos_end).set_context(context)
        if value == True:
            self.symbol_table.set(ins_name.__repr__(), ins_value)

        return res.success(ins_value)

    def visit_CallNode(self, node, context, value=True, attributes=None):
        if value == True:
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
            try:
                if type(value_to_call) == type(ins_def.Class(value_to_call.name, value_to_call.methods, value_to_call.parameters, value_to_call.run, value_to_call.memory, value_to_call.super_class)):
                    return_value = res.register(value_to_call.execute(args))
                    if res.error:
                        return res
                    return_value = return_value.copy().set_pos(
                        node.pos_start, node.pos_end).set_context(context)
                    return res.success(value_to_call)
            except AttributeError:

                return_value = res.register(value_to_call.execute(args))
                if res.error:
                    return res
                return_value = return_value.copy().set_pos(
                    node.pos_start, node.pos_end).set_context(context)
                return res.success(return_value)
        else:
            res = RuntimeResult()
            value_to_call = res.register(self.visit(node.name, context, False))
            args = []
            for arg_node in node.args:
                args.append(res.register(self.visit(arg_node, context)))
                if res.error:
                    return res
            value_to_call.args = args
            return res.success(value_to_call)

    def visit_ClassNode(self, node, context, value=True, attributes=None):
        res = RuntimeResult()
        name = res.register(self.visit(node.name, context, False))
        arg_list = [arg_name.token for arg_name in node.parameter]
        body = StatementNode(node.name, node.run)
        methods = [res.register(self.visit(i, context, False))
                   for i in node.methods]

        if node.superclass:
            super_class = res.register(self.visit(node.superclass, context))
            class_ = ins_def.Class(
                name.__repr__(), methods, arg_list, body, self.list_of_memory, super_class)
            self.symbol_table.set(name.__repr__(), class_)
            return res.success(class_)
        else:
            class_ = ins_def.Class(
                name.__repr__(), methods, arg_list, body, self.list_of_memory)
            self.symbol_table.set(name.__repr__(), class_)
            return res.success(class_)

    def visit_AttributeNode(self, node, context, value=True, attributes=None):
        res = RuntimeResult()
        if attributes is None:
            return res.failure(error.RuntimeError(
                node.pos_start, node.pos_end,
                "Expected a class", context
            ))
        else:
            if node.value:
                val = res.register(self.visit(node.value, context))
                name = res.register(self.visit(node.name, context, False))
                attributes[name.__repr__()] = val
                self.symbol_table.set(name.__repr__(), val)

            else:
                name = res.register(self.visit(node.name, context))

                if attributes.get(name.__repr__(), None) is None:

                    return res.failure(error.RuntimeError(
                        node.pos_start, node.pos_end,
                        f"Undefined attribute detected",
                        context
                    ))

                else:
                    return res.success(attributes.get(name.__repr__(), None).set_pos(node.pos_start, node.pos_end).set_context(context))
        return res.success(Number.null.set_pos(node.pos_start, node.pos_end).set_context(context))

    def visit_PlaceHolderNode(self, node, context, value=True, attributes=None):
        res = RuntimeResult()
        return res.success(PlaceHolder().set_pos(node.pos_start, node.pos_end).set_context(context))

    def visit_StatementNode(self, node, context, value=True, attributes=None):
        res = RuntimeResult()

        list1 = []
        temp = None
        for i in node.value:
            res = self.visit(i, context, value, attributes=attributes)
            temp = res
            if res.error:
                break
            list1.append(res)
        if temp.error:
            self.error = temp.value
            return temp
        elif len(list1) == 1:
            return res.success(list1[0].value.set_pos(node.pos_start, node.pos_end).set_context(context))

        return res.success(Number.null.set_pos(node.pos_start, node.pos_end).set_context(context))
