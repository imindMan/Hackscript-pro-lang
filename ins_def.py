import random
import Hackinterpreter
import Hackparser
import Hacklexer
from hacktypes.datatypes import *
from hacktypes.impor_type import *
from error.error import *
import os
import importlib
import types
import inspect


def checkempty(string):
    for i in string:
        if i in FULL_SYMBOLS + DIGITS + LETTERS:
            return False
    return True


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
        Method.type = Method("type", memory)
        Method.range = Method("range", memory)
        Method.remove = Method("remove", memory)

        symbol_table.set("null", null)
        symbol_table.set("true", true)
        symbol_table.set("false", false)
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
        symbol_table.set("trash", Identifier("trash"))
        symbol_table.set("?", Method.random)
        symbol_table.set("pp", Identifier("pp"))
        symbol_table.set("len", Method.len)
        symbol_table.set("%", Method.type)
        symbol_table.set("rl", Method.range)
        symbol_table.set("dd", Method.remove)

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


class Class(Value):
    def __init__(self, name, methods, parameters, run, memory, super_class=None):
        super().__init__(name)
        self.name = name
        self.methods = methods
        if len(self.methods) == 1 and isinstance(self.methods[0], Identifier):
            self.attributes = {}
        else:
            self.attributes = dict(map(lambda x: (x.name, x), self.methods))
        self.parameters = parameters
        self.run = run
        self.super_class = super_class
        self.memory = memory
        self.parent_symbol_table = memory.symbols_table

    def set_up_symbol_table(self, symbol_table, memory):
        res = RuntimeResult()
        Method.change_status = Method("change_status", memory)
        Method.exit = Method("exit", memory)
        Method.clear = Method("clear", memory)
        Method.set_constant = Method("set_constant", memory)
        Method.launch = Method("launch", memory)
        Method.end_launch = Method("end_launch", memory)
        Method.push = Method("push", memory)
        Method.random = Method("random", memory)
        Method.type = Method("type", memory)
        Method.range = Method("range", memory)
        Method.remove = Method("remove", memory)

        symbol_table.set("null", null)
        symbol_table.set("true", true)
        symbol_table.set("false", false)
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
        symbol_table.set("trash", Identifier("trash"))
        symbol_table.set("?", Method.random)
        symbol_table.set("pp", Identifier("pp"))
        symbol_table.set("len", Method.len)
        symbol_table.set("%", Method.type)
        symbol_table.set("rl", Method.range)
        symbol_table.set("dd", Method.remove)
        for i in self.methods:
            try:
                symbol_table.set(i.name, i)
            except:
                return res.failure(error.InvalidObject(
                    self.pos_start, self.pos_end,
                    "Method part can only accept instructions, classes, and some special expressions only, no more kind of expressions. If you want to do it, you have to put in the constructor"
                ))
        if isinstance(self.super_class, Class):
            for i in self.super_class.methods:
                try:
                    symbol_table.set(i.name, i)
                    self.attributes[i.name] = i
                except:
                    return res.failure(error.InvalidObject(
                        self.pos_start, self.pos_end,
                        "Method part can only accept instructions, classes, and some special expressions only, no more kind of expressions. If you want to do it, you have to put in the constructor"
                    ))

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

    def execute(self, args):
        res = RuntimeResult()
        context, symbol_table, memory = self.generate_basic_thing()

        res.register(self.check_and_populate_args(
            self.parameters, args, context, memory))
        if res.error:
            return res
        interpreter = Hackinterpreter.Interpreter(memory, symbol_table)
        value = res.register(interpreter.visit(
            self.run, context, attributes=self.attributes, superclass=self.super_class))
        if res.error:
            return res

        return res.success(value)

    def attribute(self, other, args=None):
        if isinstance(other, Identifier):
            if self.attributes.get(other.value, None) is None:
                return None, error.RuntimeError(
                    self.pos_start, self.pos_end,
                    "Attribute cannot be detected", self.context
                )
            elif isinstance(self.attributes.get(other.value, None), Instruction):
                return_value = None
                if args is not None:
                    return_value = self.attributes.get(
                        other.value).execute(args)
                else:
                    return_value = self.attributes.get(
                        other.value).execute([])
                return return_value.value, None
            return self.attributes[other.value], None

    def assign_from(self, other=None):
        if isinstance(other, Class):
            self.name = other.name
            self.methods = other.methods
            self.attributes = other.attributes
            self.parameters = other.parameters
            self.run = other.run
            self.super_class = other.super_class
            self.memory = other.memory
            self.parent_symbol_table = other.parent_symbol_table
            return self, None
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def equ_to(self, other):
        if not isinstance(other, Class):
            return Boolean(0).set_context(self.context), None
        if (self.name, self.methods, self.parameters, self.run, self.memory, self.super_class, self.attributes) == \
                (other.name, other.methods, other.parameters, other.run, other.memory, other.super_class, other.attributes):
            return Boolean(1).set_context(self.context), None
        return Boolean(0).set_context(self.context), None

    def not_equ_to(self, other):
        if not isinstance(other, Class):
            return Boolean(0).set_context(self.context), None
        if (self.name, self.methods, self.parameters, self.run, self.memory, self.super_class, self.attributes) != \
                (other.name, other.methods, other.parameters, other.run, other.memory, other.super_class, other.attributes):
            return Boolean(1).set_context(self.context), None
        return Boolean(0).set_context(self.context), None

    def copy(self):
        class_ = Class(self.name, self.methods, self.parameters,
                       self.run, self.memory, self.super_class)
        class_.attributes = self.attributes
        class_.set_pos(self.pos_start, self.pos_end)
        class_.set_context(self.context)
        return class_

    def __repr__(self):
        return f"<class: {self.name}>"


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
            return res.success(null)
        else:
            return res.failure(error.InvalidStatus(
                self.pos_start, self.pos_end,
                f"Invalid status '{memory.symbols_table.get('value')}'"
            ))
    execute_change_status.arg = [Identifier("status")]

    def execute_clear(self, context, memory):
        os.system("cls") if os.name == "nt" else os.system("clear")
        return RuntimeResult().success(null)
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
            return res.success(null)
        else:
            return res.failure(error.InvalidObject(
                self.pos_start, self.pos_end,
                "Invalid parameter value"
            ))
    execute_set_constant.arg = [Identifier("value")]

    def execute_launch(self, context, memory: ListofMemory):
        res = RuntimeResult()
        parent_memory = memory.parent_list_of_memory

        if isinstance(memory.symbols_table.get("value"), Pointer):
            self.launch_table["pointer_on_launch"] = memory.symbols_table.get(
                "value")

            return res.success(null)
        elif isinstance(memory.symbols_table.get("value"), ConstantPointer):
            if parent_memory.access_constant(memory.symbols_table.get("value").type.value) == "Error catching while defined constant pointer":
                return res.failure(error.InvalidObject(
                    self.pos_start, self.pos_end,
                    "Undefined constant pointer"
                ))
            self.launch_table["pointer_constant_on_launch"] = memory.symbols_table.get(
                "value")

            return res.success(null)
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
            value = self.launch_table.get("pointer_on_launch", None)
            if value == None:
                return res.failure(error.InvalidObject(
                    self.pos_start, self.pos_end,
                    "No pointer detected on launch"
                ))
            del self.launch_table["pointer_on_launch"]

            return res.success(null)

        elif isinstance(memory.symbols_table.get("value"), ConstantPointer):
            if parent_memory.access_constant(memory.symbols_table.get("value").type.value) == "Error catching while defined constant pointer":
                return res.failure(error.InvalidObject(
                    self.pos_start, self.pos_end,
                    "Undefined constant pointer"
                ))
            if self.launch_table.get("pointer_constant_on_launch", None) == None:
                return res.failure(error.InvalidObject(
                    self.pos_start, self.pos_end,
                    "No constant pointer detected on launch"
                ))
            del self.launch_table["pointer_constant_on_launch"]
            return res.success(null)
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
                    if len(parent_memory.curr_char.value) == 1:
                        print(parent_memory.curr_char.value[0])
                    elif len(parent_memory.curr_char.value) == 0:
                        print(r"{}")
                    elif len(parent_memory.curr_char.value) > 1:
                        print(parent_memory.curr_char.value)

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
            elif memory.symbols_table.get("value") == parent_memory.symbols_table.get("trash"):

                if isinstance(pointer_to_push, Pointer):

                    parent_memory.curr_char.delete()
                elif isinstance(memory.symbols_table.get("value"), ConstantPointer):
                    constant_pointer = parent_memory.access_constant(
                        pointer_to_push.type.value)
                    parent_memory.access_constant(
                        pointer_to_push.type.value).delete()

        return res.success(null)

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

    def execute_len(self, context, memory):
        res = RuntimeResult()
        return res.success(Number(len(memory.symbols_table.get("value").value)))
    execute_len.arg = [Identifier("value")]

    def execute_type(self, context, memory):
        res = RuntimeResult()
        if memory.symbols_table.get("type") == null:
            if isinstance(memory.symbols_table.get("value"), ClassString):
                return res.success(ClassString("<string>"))
            elif isinstance(memory.symbols_table.get("value"), List):
                return res.success(ClassString("<list>"))
            elif isinstance(memory.symbols_table.get("value"), ConstantPointer):
                return res.success(ClassString("<constant-pointer>"))
            elif isinstance(memory.symbols_table.get("value"), Pointer):
                return res.success(ClassString("<pointer>"))
            elif isinstance(memory.symbols_table.get("value"), Number):
                return res.success(ClassString("<number>"))
            elif isinstance(memory.symbols_table.get("value"), ListofMemory):
                return res.success(ClassString("<listmemo>"))
            elif isinstance(memory.symbols_table.get("value"), Memory):
                return res.success(ClassString("<memory>"))
            elif isinstance(memory.symbols_table.get("value"), PlaceHolder):
                return res.success(ClassString("<holder>"))
            elif isinstance(memory.symbols_table.get("value"), Class):
                return_string = memory.symbols_table.get("value").__repr__()
                return res.success(ClassString(return_string))
            elif isinstance(memory.symbols_table.get("value"), Boolean):
                return res.success(ClassString("<boolean>"))
            elif isinstance(memory.symbols_table.get("value"), Null):
                return res.success(ClassString("<null>"))

        elif isinstance(memory.symbols_table.get("type"), ClassString):
            if memory.symbols_table.get("type").value == "num":
                if isinstance(memory.symbols_table.get("value"), ClassString):
                    val = memory.symbols_table.get("value").value
                    try:
                        if "." in val:
                            val = float(val)
                        else:
                            val = int(val)
                    except:
                        return res.failure(error.IllegalChangeType(
                            self.pos_start, self.pos_end,
                            "Invalid value to change type"
                        ))
                    return res.success(Number(val))
                elif isinstance(memory.symbols_table.get("value"), Number):
                    return res.success(memory.symbols_table.get("value"))

            elif memory.symbols_table.get("type").value == "list":
                if isinstance(memory.symbols_table.get("value"), ClassString):
                    val = memory.symbols_table.get("value").value
                    val = [char for char in val]
                    return res.success(List(val))
                elif isinstance(memory.symbols_table.get("value"), List):
                    return res.success(memory.symbols_table.get("value"))

            elif memory.symbols_table.get("type").value == "string":
                if isinstance(memory.symbols_table.get("value"), ClassString):
                    return res.success(memory.symbols_table.get("value"))
                elif isinstance(memory.symbols_table.get("value"), Number):
                    val = memory.symbols_table.get("value").value
                    val = str(val)
                    return res.success(ClassString(val))
                elif isinstance(memory.symbols_table.get("value"), List):
                    return res.success(ClassString(str(memory.symbols_table.get("value").__repr__())))

        return res.failure(error.InvalidObject(
            self.pos_start, self.pos_end,
            "Invalid parameters"
        ))

    execute_type.arg = [Identifier("type"), Identifier("value")]

    def execute_range(self, context, memory):
        res = RuntimeResult()
        if isinstance(memory.symbols_table.get("start"), Number) and isinstance(memory.symbols_table.get("end"), Number):
            list_ = [Number(i) for i in range(memory.symbols_table.get(
                "start").value, memory.symbols_table.get("end").value + 1)]
            return res.success(List(list_))
        return res.failure(error.InvalidObject(
            self.pos_start, self.pos_end,
            "Invalid parameters"
        ))
    execute_range.arg = [Identifier("start"), Identifier("end")]

    def execute_import_a_library(self, context, memory):
        result = RuntimeResult()
        name = memory.symbols_table.get("name_of_library").value
        parent_symbol_table = memory.parent_list_of_memory.symbols_table
        if os.path.exists(name):
            f = open(name, "r")
            file_inc = f.read()
            f.close()
            if checkempty(file_inc):
                return result.success(null)
            try:
                lexer = Hacklexer.Lexer(os.path.abspath(name), file_inc)
                tokens, err = lexer.make_tokens()

                # return tokens, error
                if err:
                    return result.failure(error)

                parser = Hackparser.Parser(tokens)
                ast = parser.parse()
                if ast.error:
                    return result.failure(ast.error)

                intepreter = Hackinterpreter.Interpreter(
                    memory.parent_list_of_memory, parent_symbol_table)
                res = intepreter.visit(ast.node, context)
                if res.error:
                    return result.failure(res.error)
                return result.success(res.value)

            except RecursionError:
                return result.failure(RuntimeError(
                    ast.node.pos_start, ast.node.pos_end,
                    "Non-stop infinity run (mostly due to recursion)",
                    context
                ))
            except KeyboardInterrupt:
                print(
                    "status: [stopped] -> info (?still_run, ?interrupt, ?end_run)")
                return None, None

        else:
            try:
                library = importlib.import_module(
                    f"library.{name}.main")
                list_sample = []

                for i in dir(library):
                    if i[:2] != "__":
                        list_sample.append(i)
                for i in list_sample:
                    if inspect.isclass(getattr(library, i)):
                        parent_symbol_table.set(
                            i, CustomClass(getattr(library, i)))
                    elif inspect.isfunction(getattr(library, i)):
                        parent_symbol_table.set(
                            i, CustomFunction(getattr(library, i)))

                return result.success(null)
            except:
                return result.failure(error.InvalidObject(
                    self.pos_start, self.pos_end,
                    "Undefined library"
                ))

    execute_import_a_library.arg = [Identifier("name_of_library")]

    def execute_remove(self, context, memory):
        res = RuntimeResult()
        if isinstance(memory.symbols_table.get("value"), ConstantPointer):
            if memory.symbols_table.get("value").__repr__() == "Error catching while defined constant pointer":
                return res.failure(error.InvalidObject(
                    self.pos_start, self.pos_end,
                    "The constant pointer cannot be defined"
                ))
            parent_memory = memory.parent_list_of_memory
            parent_memory.delete_constant(
                memory.symbols_table.get("value"))
            return res.success(null)
        return res.failure(error.InvalidObject(
            self.pos_start, self.pos_end,
            "Invalid type"
        ))
    execute_remove.arg = [Identifier("value")]
