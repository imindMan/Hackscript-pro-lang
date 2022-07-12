import Hackinterpreter
from hacktypes.impor_type import *
from error.error import *
import os


class GeneralInstruction(Value):
    def __init__(self, name, memory: ListofMemory):
        super().__init__(name)
        self.name = name
        self.memory = memory
        self.parent_symbol_table = memory.symbols_table

    def generate_basic_thing(self):
        context = Context(self.name, self.context, self.pos_start)
        symbol_table = SymbolTable(self.parent_symbol_table)
        memory = ListofMemory(symbol_table, self.memory)
        return context, symbol_table, memory

    def check_args(self, arg_names, args):
        res = RuntimeResult()
        if len(args) > len(arg_names):
            return res.failure(RuntimeError(
                self.pos_start, self.pos_end,
                f"Too many args passed into '{self.name}'",
                self.context
            ))

        if len(args) < len(arg_names):
            return res.failure(RuntimeError(
                self.pos_start, self.pos_end,
                f"Too few args passed into '{self.name}'",
                self.context
            ))
        return res.success(None)

    def populate_args(self, arg_names, args, context, memory):
        for i in range(len(args)):
            arg_name = arg_names[i]
            arg_value = args[i]
            arg_value.set_context(context)
            memory.symbols_table.set(arg_name.value, arg_value)

    def check_and_populate_args(self, arg_names, args, context, memory):
        res = RuntimeResult()
        res.register(self.check_args(arg_names, args))
        if res.error:
            return res
        self.populate_args(arg_names, args, context, memory)
        return res.success(None)


class Instruction(GeneralInstruction):
    def __init__(self, name, args, body, memory):
        super().__init__(name, memory)
        self.args = args
        self.body = body

    def execute(self, args):
        res = RuntimeResult()
        context, symbol_table, memory = self.generate_basic_thing()

        res.register(self.check_and_populate_args(
            self.args, args, context, memory))
        if res.error:
            return res
        interpreter = Hackinterpreter.Interpreter(memory, symbol_table)
        value = res.register(interpreter.visit(self.body, context))
        if res.error:
            return res
        return res.success(value)

    def copy(self):
        res = Instruction(self.name, self.args, self.body,
                          self.memory)
        res.set_pos(self.pos_start, self.pos_end)
        res.set_context(self.context)

        return res

    def __repr__(self):
        return f"<instruction: {self.name}>"


class Method(GeneralInstruction):

    def __init__(self, name, memory):
        super().__init__(name, memory)

    def execute(self, args):
        res = RuntimeResult()
        context, symbol_table, memory = self.generate_basic_thing()

        method_name = f'execute_{self.name}'

        method = getattr(self, method_name, self.no_visit)
        res.register(self.check_and_populate_args(
            method.arg, args, context, memory))
        if res.error:
            return res
        return_value = res.register(method(context, memory))
        if res.error:
            return res
        return res.success(return_value)

    def no_visit(self, node, context):
        return RuntimeResult().failure(error.NoVisitError(
            node.pos_start, node.pos_end,
            "No visit_{} method defined".format(
                type(node).__name__)
        ))

    def copy(self):
        copy = Method(self.name, self.memory)
        copy.set_pos(self.pos_start, self.pos_end)
        copy.set_context(self.context)

        return copy

    def __repr__(self):
        return f"<method {self.name}>"

    def execute_change_status(self, context, memory):
        res = RuntimeResult()
        parent_memory = memory.parent_list_of_memory
        if isinstance(memory.symbols_table.get("status"), Number):
            err = parent_memory.curr_char.set_pos(self.pos_start, self.pos_end).set_context(context).change_status(
                memory.symbols_table.get("status"))
            if error:
                return res.failure(err)
            return res.success(Number.null)
        else:
            return res.failure(error.InvalidStatus(
                self.pos_start, self.pos_end,
                f"Invalid status '{memory.symbols_table.get('value')}'"
            ))
    execute_change_status.arg = [Identifier("status")]

    def execute_clear(self, context, memory):
        os.system("cls") if os.name == "nt" else os.system("clear")
        return RuntimeResult().success(Number.null)
    execute_clear.arg = []

    def execute_exit(self, context, memory):
        exit()
    execute_exit.arg = []

    def execute_set_constant(self, context, memory: ListofMemory):
        res = RuntimeResult()
        parent_memory = memory.parent_list_of_memory
        if isinstance(memory.symbols_table.get("value"), ConstantPointer):
            parent_memory.set_pos(self.pos_start, self.pos_end).set_context(context).set_constant(
                memory.symbols_table.get("value").type.value)
            return res.success(Number.null)
        else:
            return res.failure(error.InvalidObject(
                self.pos_start, self.pos_end,
                "Invalid parameter value"
            ))
    execute_set_constant.arg = [Identifier("value")]
