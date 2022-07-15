import random
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

    def set_up_symbol_table(self, symbol_table, memory):
        Method.change_status = Method("change_status", memory)
        Method.exit = Method("exit", memory)
        Method.clear = Method("clear", memory)
        Method.set_constant = Method("set_constant", memory)
        Method.launch = Method("launch", memory)
        Method.end_launch = Method("end_launch", memory)
        Method.push = Method("push", memory)
        Method.random = Method("random", memory)

        symbol_table.set("null", NULL)
        symbol_table.set("true", TRUE)
        symbol_table.set("false", FALSE)
        symbol_table.set("!", Method.change_status)
        symbol_table.set("exit", Method.exit)
        symbol_table.set("clear", Method.clear)
        symbol_table.set("s", Method.set_constant)
        symbol_table.set("$", Method.launch)
        symbol_table.set(".", Method.end_launch)
        symbol_table.set("pu", Method.push)
        symbol_table.set("in", Identifier("in"))
        symbol_table.set("out", Identifier("out"))
        symbol_table.set("con", Identifier("con"))
        symbol_table.set("?", Method.random)
        symbol_table.set("pp", Identifier("pp"))

    def generate_basic_thing(self):
        context = Context(self.name, self.context, self.pos_start)
        symbol_table = SymbolTable(self.parent_symbol_table)
        launch_table = {}
        memory = ListofMemory(symbol_table, launch_table, self.memory)

        self.set_up_symbol_table(symbol_table, memory)
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
        self.launch_table = memory.launch_table

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
            status = memory.symbols_table.get("status")
            status.value = str(status.value)
            err = parent_memory.curr_char.set_pos(self.pos_start, self.pos_end).set_context(context).change_status(
                status)
            if err:
                return res
            return res.success(NULL)
        else:
            return res.failure(error.InvalidStatus(
                self.pos_start, self.pos_end,
                f"Invalid status '{memory.symbols_table.get('value')}'"
            ))
    execute_change_status.arg = [Identifier("status")]

    def execute_clear(self, context, memory):
        os.system("cls") if os.name == "nt" else os.system("clear")
        return RuntimeResult().success(NULL)
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
            return res.success(NULL)
        else:
            return res.failure(error.InvalidObject(
                self.pos_start, self.pos_end,
                "Invalid parameter value"
            ))
    execute_set_constant.arg = [Identifier("value")]

    def execute_launch(self, context, memory: ListofMemory):
        res = RuntimeResult()
        parent_memory = memory.parent_list_of_memory

        if len(self.launch_table) == 1:
            self.launch_table = {}
        if isinstance(memory.symbols_table.get("value"), Pointer):
            self.launch_table["pointer_on_launch"] = memory.symbols_table.get(
                "value")

            return res.success(NULL)
        elif isinstance(memory.symbols_table.get("value"), ConstantPointer):
            if parent_memory.access_constant(memory.symbols_table.get("value").type.value) == "Error catching while defined constant pointer":
                return res.failure(error.InvalidObject(
                    self.pos_start, self.pos_end,
                    "Undefined constant pointer"
                ))
            self.launch_table["pointer_constant_on_launch"] = memory.symbols_table.get(
                "value")

            return res.success(NULL)
        else:

            return res.failure(error.InvalidObject(
                self.pos_start, self.pos_end,
                "Launch method can only accept a Pointer or a Constant Pointer"
            ))

    execute_launch.arg = [Identifier("value")]

    def execute_end_launch(self, context, memory: ListofMemory):
        res = RuntimeResult()
        parent_memory = memory.parent_list_of_memory
        if isinstance(memory.symbols_table.get("value"), Pointer):
            self.launch_table = {}
            return res.success(NULL)

        elif isinstance(memory.symbols_table.get("value"), ConstantPointer):
            if parent_memory.access_constant(memory.symbols_table.get("value").type.value) == "Error catching while defined constant pointer":
                return res.failure(error.InvalidObject(
                    self.pos_start, self.pos_end,
                    "Undefined constant pointer"
                ))
            self.launch_table = {}
            return res.success(NULL)
        else:
            return res.failure(error.InvalidObject(
                self.pos_start, self.pos_end,
                "End launch method can only accept a Pointer, Constant Pointer"
            ))
    execute_end_launch.arg = [Identifier("value")]

    def execute_push(self, context, memory: ListofMemory):
        res = RuntimeResult()
        parent_memory = memory.parent_list_of_memory

        if len(self.launch_table) > 0:

            pointer_to_push = list(self.launch_table.values())[0]

        else:
            return res.failure(error.RuntimeError(
                self.pos_start, self.pos_end,
                "Current there's no pointer on the launch table, maybe you miss to launch it?",
                context
            ))
        if memory.symbols_table.get("how_to_push") == parent_memory.symbols_table.get("in"):

            if isinstance(memory.symbols_table.get("value"), Pointer) or isinstance(memory.symbols_table.get("value"), ConstantPointer):
                return res.failure(error.InvalidObject(
                    self.pos_start, self.pos_end,
                    "Cannot push a pointer or a constant pointer into a memory stack"
                ))
            if isinstance(pointer_to_push, Pointer):
                parent_memory.curr_char.push(memory.symbols_table.get("value"))
            if isinstance(pointer_to_push, ConstantPointer):
                constant_pointer = parent_memory.access_constant(
                    pointer_to_push.type.value)
                constant_pointer.push(memory.symbols_table.get("value"))

        elif memory.symbols_table.get("how_to_push") == parent_memory.symbols_table.get("out"):

            if memory.symbols_table.get("value") == parent_memory.symbols_table.get("con"):

                if isinstance(pointer_to_push, Pointer):
                    if len(parent_memory.curr_char.data) == 1:
                        print(parent_memory.curr_char.data[0])
                    elif len(parent_memory.curr_char.data) == 0:
                        print(r"{}")
                    elif len(parent_memory.curr_char.data) > 1:
                        print(List(parent_memory.curr_char.data))

                    parent_memory.curr_char.delete()
                elif isinstance(memory.symbols_table.get("value"), ConstantPointer):
                    constant_pointer = parent_memory.access_constant(
                        pointer_to_push.type.value)
                    if len(constant_pointer.data) == 1:
                        print(constant_pointer.data[0])
                    elif len(constant_pointer.data) == 0:
                        print(r"{}")
                    elif len(constant_pointer.data) > 1:
                        print(List(constant_pointer.data))
                    parent_memory.access_constant(
                        pointer_to_push.type.value).delete()

        return res.success(NULL)

    execute_push.arg = [Identifier("how_to_push"), Identifier("value")]

    def execute_random(self, context, memory: ListofMemory):
        res = RuntimeResult()
        if isinstance(memory.symbols_table.get("value"), List):
            choice = random.choice(memory.symbols_table.get("value").value)
            return res.success(choice)
        elif memory.symbols_table.get("value") == memory.parent_list_of_memory.symbols_table.get("pp"):
            result = input()
            return res.success(ClassString(result))

    execute_random.arg = [Identifier("value")]
