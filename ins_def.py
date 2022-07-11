import Hackinterpreter
from hacktypes.impor_type import *
from error.error import *


class Instruction(Value):
    def __init__(self, name, args, body, parent_symbol_table):
        super().__init__(name)
        self.name = name
        self.args = args
        self.body = body
        self.parent_symbol_table = parent_symbol_table

    def execute(self, args):
        res = RuntimeResult()
        context = Context(self.name, self.context, self.pos_start)
        symbol_table = SymbolTable(self.parent_symbol_table)
        memory = ListofMemory(symbol_table)

        if len(args) > len(self.args):
            return res.failure(RuntimeError(
                self.pos_start, self.pos_end,
                f"{len(args) - len(self.args)} too many args passed into '{self.name}'",
                self.context
            ))

        if len(args) < len(self.args):
            return res.failure(RuntimeError(
                self.pos_start, self.pos_end,
                f"{len(self.args) - len(args)} too few args passed into '{self.name}'",
                self.context
            ))
        for i in range(len(args)):
            arg_name = self.args[i]
            arg_value = args[i]
            arg_value.set_context(context)
            symbol_table.set(arg_name.value, arg_value)
        interpreter = Hackinterpreter.Interpreter(memory, symbol_table)
        value = res.register(interpreter.visit(self.body, context))
        if res.error:
            return res
        return res.success(value)

    def copy(self):
        res = Instruction(self.name, self.args, self.body,
                          self.parent_symbol_table)
        res.set_pos(self.pos_start, self.pos_end)
        res.set_context(self.context)

        return res

    def __repr__(self):
        return f"<instruction: {self.name}>"
