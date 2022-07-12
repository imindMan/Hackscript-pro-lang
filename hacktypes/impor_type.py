import error.error as error
import math
import fractions


class RuntimeResult:
    def __init__(self):
        self.error = None
        self.value = None

    def register(self, res):
        if res.error:
            self.error = res.error
        return res.value

    def success(self, value):
        self.value = value
        return self

    def failure(self, error):
        self.error = error
        return self


class SymbolTable:
    def __init__(self, parent=None):
        self.symbols = {}
        self.parent = parent

    def get(self, name):
        value = self.symbols.get(name, None)
        if value == None and self.parent:
            return self.parent.get(name)
        return value

    def set(self, name, value):
        self.symbols[name] = value

    def remove(self, name):
        del self.symbols[name]

    def __repr__(self):
        return f"{self.symbols}"


class Position:
    def __init__(self, index, line, column, fname, fcontent):
        self.index = index
        self.line = line
        self.column = column
        self.fname = fname
        self.fcontent = fcontent

    def advance(self, curr_char=None):
        self.index += 1
        self.column += 1

        if curr_char == "\n":
            self.line += 1
            self.column = 0
        return self

    def copy(self):
        return Position(self.index, self.line, self.column, self.fname, self.fcontent)


class Context:
    def __init__(self, display_name, parent=None, parent_entry_pos=None):
        self.display_name = display_name
        self.parent = parent
        self.parent_entry_pos = parent_entry_pos

###############################################
# Interpreter Value
###############################################


class Value:
    def __init__(self, value=None):
        self.value = value

        self.set_pos()
        self.set_context()

    def set_pos(self, pos_start=None, pos_end=None):
        self.pos_start = pos_start
        self.pos_end = pos_end
        return self

    def set_context(self, context=None):
        self.context = context
        return self

    def added_to(self, other):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def subtracted_to(self, other):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def multiplied_to(self, other):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def divided_to(self, other):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def powed_to(self, other):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def fraced_to(self, other):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def sqrt(self):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def assign_from(self, other):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def equ_to(self, other):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def not_equ_to(self, other):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def gre_to(self, other):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def les_to(self, other):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def gre_equ_to(self, other):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def les_equ_to(self, other):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def and_to(self, other):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def or_to(self, other):

        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def not_to(self):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def assign_from(self, other):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def __repr__(self):

        return f"{self.value}"


class Number(Value):
    def __init__(self, value):
        self.value = value
        self.set_pos()
        self.set_context()

    def set_pos(self, pos_start=None, pos_end=None):
        self.pos_start = pos_start
        self.pos_end = pos_end
        return self

    def set_context(self, context=None):
        self.context = context
        return self

    def added_to(self, other):
        if isinstance(other, Number):
            res = self.value + other.value
            return Number(res).set_context(self.context), None
        else:
            return None, error.OperatorNotSupported(
                self.pos_start, self.pos_end,
                "Invalid type for doing expression (integer or float)"
            )

    def subtracted_to(self, other):
        if isinstance(other, Number):
            res = self.value - other.value
            return Number(res).set_context(self.context), None
        else:
            return None, error.OperatorNotSupported(
                self.pos_start, self.pos_end,
                "Invalid type for doing expression (integer or float)"
            )

    def multiplied_to(self, other):
        if isinstance(other, Number):
            res = self.value * other.value
            return Number(res).set_context(self.context), None
        else:
            return None, error.OperatorNotSupported(
                self.pos_start, self.pos_end,
                "Invalid type for doing expression (integer or float)"
            )

    def divided_to(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                return None, error.RuntimeError(
                    self.pos_start, self.pos_end, "Division by zero", self.context)
            res = self.value / other.value
            return Number(res).set_context(self.context), None
        else:
            return None, error.OperatorNotSupported(
                self.pos_start, self.pos_end,
                "Invalid type for doing expression (integer or float)"
            )

    def powed_to(self, other):
        if isinstance(other, Number):
            res = self.value ** other.value
            return Number(res).set_context(self.context), None
        else:
            return None, error.OperatorNotSupported(
                self.pos_start, self.pos_end,
                "Invalid type for doing expression (integer or float)"
            )

    def fraced_to(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                return None, error.RuntimeError(
                    self.pos_start, self.pos_end, "Division by zero", self.context
                )
            res = fractions.Fraction(self.value, other.value)
            return Number(res).set_context(self.context), None
        else:
            return None, error.OperatorNotSupported(
                self.pos_start, self.pos_end,
                "Invalid type for doing expression (integer or float)"
            )

    def sqrt(self):
        if self.value >= 0:
            res = math.sqrt(self.value)
            return Number(res).set_context(self.context), None
        elif self.value < 0:
            return None, error.RuntimeError(
                self.pos_start, self.pos_end,
                "Factor must be a positive number", self.context
            )
        else:
            return None, error.OperatorNotSupported(
                self.pos_start, self.pos_end,
                "Invalid type for doing expression (integer or float)"
            )

    def equ_to(self, other):
        return Number(int(self.value == other.value)).set_context(self.context), None

    def not_equ_to(self, other):
        return Number(int(self.value != other.value)).set_context(self.context), None

    def gre_to(self, other):
        return Number(int(self.value > other.value)).set_context(self.context), None

    def les_to(self, other):
        return Number(int(self.value < other.value)).set_context(self.context), None

    def gre_equ_to(self, other):
        return Number(int(self.value >= other.value)).set_context(self.context), None

    def les_equ_to(self, other):
        return Number(int(self.value <= other.value)).set_context(self.context), None

    def and_to(self, other):
        return Number(int(self.value and other.value)).set_context(self.context), None

    def or_to(self, other):

        return Number(int(self.value or other.value)).set_context(self.context), None

    def copy(self):
        number = Number(self.value)
        number.set_pos(self.pos_start, self.pos_end)
        number.set_context(self.context)
        return number

    def not_to(self):
        return Number(int(not self.value)).set_context(self.context), None

    def __repr__(self):

        return f"{self.value}"


class Memory(Value):
    def __init__(self, name):
        super().__init__(name)
        self.name = name
        self.data = []
        self.status = "200"

    def push(self, data):
        if self.status == "200":
            self.status = "210"
            self.data.append(data)
            self.status = "220"

    def delete(self, index):
        if self.status == "200":
            self.status = "210"
            try:
                self.data.remove(self.data[index])
            except:
                return error.InvalidIndexOfMemory(
                    self.pos_start, self.pos_end,
                    "Invalid index because data doesn't have that much memory"
                )

            self.status = "230"

    def change_status(self, status):
        if status.value > 230:
            return error.InvalidStatus(
                self.pos_start, self.pos_end,
                f"Invalid status {status}"
            )
        self.status = status
        return None

    def __repr__(self):
        return f"data:{self.data}({self.status})"


class ListofMemory:
    def __init__(self, symbols_table, parent_list_of_memory=None):

        self.data = [Memory(str(i)) for i in range(10)]
        self.index = 0
        self.curr_char = self.data[self.index] if 0 <= self.index < len(
            self.data) else None
        self.symbols_table = symbols_table
        self.parent_list_of_memory = parent_list_of_memory
        self.set_pos()
        self.set_context()

    def change_status(self, status):
        self.curr_char.change_status(status)

    def set_pos(self, pos_start=None, pos_end=None):
        self.pos_start = pos_start
        self.pos_end = pos_end

    def set_context(self, context=None):
        self.context = context
        return self

    def set_constant(self, name):
        self.curr_char.name = name
        index = self.index
        if index + 1 >= len(self.data):
            self.data.append(Memory(str(f"{index + 1}")))
            self.index += 1
            self.curr_char = self.data[self.index] if 0 <= self.index < len(
                self.data) else None
        else:
            self.index += 1
            self.curr_char = self.data[self.index] if 0 <= self.index < len(
                self.data) else None
        self.symbols_table.set(name, "cons-pointer")

    def access_constant(self, name):
        for i in self.data:
            if i.name == name:
                return i

    def move(self, number):
        if type(number.value) == type(float()):
            return None, error.OperatorNotSupported(
                self.pos_start, self.pos_end,
                "Invalid type to move pointer (integer only)"
            )
        index = self.index
        curr_char = self.curr_char
        old_index = self.index
        index += number.value
        if index < 0 or index >= len(self.data):

            index = old_index
            return None, error.InvalidIndexOfMemory(
                self.pos_start, self.pos_end,
                "Invalid index because data doesn't have that much memory"
            )

        curr_char = self.data[index]

        return (index, curr_char), None

    def copy(self):
        list_of_memory = ListofMemory(self.symbols_table)
        list_of_memory.data = self.data
        list_of_memory.index = self.index
        list_of_memory.curr_char = list_of_memory.data[list_of_memory.index] if list_of_memory.index < len(
            list_of_memory.data) else None
        list_of_memory.set_pos(self.pos_start, self.pos_end)
        list_of_memory.set_context(self.context)
        return list_of_memory

    def __repr__(self):
        return str(self.data)


class Pointer(Value):
    def __init__(self, type, list_of_memory):
        super().__init__(type)
        self.type = type
        self.list_of_memory = list_of_memory

    def copy(self):
        return Pointer(self.type, self.list_of_memory)

    def added_to(self, number):
        if isinstance(number, Identifier):
            if isinstance(number.value, Number):
                number = number.value
            else:
                return None, error.OperatorNotSupported(
                    self.pos_start, self.pos_end,
                    "Cannot using this operator in this expression"
                )
        result, error = self.list_of_memory.move(number)

        if error:
            return None, error
        else:
            sample_pointer = Pointer(self.type, self.list_of_memory.copy())
            sample_pointer.list_of_memory.index = result[0]
            sample_pointer.list_of_memory.curr_char = result[1]

            return sample_pointer, None

    def subtracted_to(self, number):
        if isinstance(number, Identifier):
            if isinstance(number.value, Number):
                number = number.value
            else:
                return None, error.OperatorNotSupported(
                    self.pos_start, self.pos_end,
                    "Cannot using this operator in this expression"
                )

        result, error = self.list_of_memory.move(
            number.multiplied_to(Number(-1))[0]
        )

        if error:
            return None, error
        else:
            sample_pointer = Pointer(self.type, self.list_of_memory.copy())
            sample_pointer.list_of_memory.index = result[0]
            sample_pointer.list_of_memory.curr_char = result[1]

            return sample_pointer, None

    def assign_from(self, other):
        if isinstance(other, Pointer):
            self.list_of_memory.index = other.list_of_memory.index

            self.list_of_memory.curr_char = other.list_of_memory.curr_char

            return self.set_context(self.context), None
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def __repr__(self):

        return f"pointer:{self.type}({self.list_of_memory.index}, {self.list_of_memory.curr_char})"


class ConstantPointer(Value):
    def __init__(self, type, list_of_memory):
        super().__init__(type)
        self.type = type
        self.list_of_memory = list_of_memory

    def copy(self):
        return Pointer(self.type, self.list_of_memory)


class Identifier(Value):
    def __init__(self, value):
        super().__init__(value)
        self.value = value


Number.null = Number(0)
Number.true = Number(1)
Number.false = Number(0)
###################################
# ALL THE NODES
###################################


class NumberNode:
    def __init__(self, token):
        self.token = token
        self.pos_start = self.token.pos_start

        self.pos_end = self.token.pos_end

    def __repr__(self) -> str:
        return f"{self.token}"


class IdentifierNode:
    def __init__(self, token):
        self.token = token
        self.pos_start = self.token.pos_start
        self.pos_end = self.token.pos_end

    def __repr__(self):
        return f"{self.token}"


class BinOpNode:
    def __init__(self, left, op, right):
        self.left = left
        self.op = op
        self.right = right

        self.pos_start = self.left.pos_start
        self.pos_end = self.right.pos_end

    def __repr__(self) -> str:
        return f"({self.left}, {self.op}, {self.right})"


class UnaryOpNode:
    def __init__(self, op_tok, node):
        self.op_tok = op_tok
        self.node = node

        self.pos_start = self.op_tok.pos_start
        self.pos_end = node.pos_end

    def __repr__(self):
        return f"({self.op_tok}, {self.node})"


class PointerNode:
    def __init__(self, op_tok, pointer_value):

        self.pointer_value = pointer_value

        self.pos_start = op_tok.pos_start
        self.pos_end = self.pointer_value.pos_end

    def __repr__(self):
        return f"pointer:{self.pointer_value}"


class ConstantPointerNode:
    def __init__(self, op_tok, pointer_value):
        self.pointer_value = pointer_value
        self.pos_start = op_tok.pos_start
        self.pos_end = self.pointer_value.pos_end

    def __repr__(self):
        return f"constant-pointer:{self.pointer_value}"


class CondNode:
    def __init__(self, condition, value_if_true, value_if_false):
        self.condition = condition
        self.value_if_true = value_if_true
        self.value_if_false = value_if_false

        self.pos_start = condition.pos_start
        self.pos_end = value_if_false.pos_end

    def __repr__(self):
        return f"check -> ({self.condition}) true: {self.value_if_true} false {self.value_if_false}"


class WhileNode:
    def __init__(self, condition, do):
        self.condition = condition
        self.do = do
        self.pos_start = self.condition.pos_start
        self.pos_end = self.do.pos_end

    def __repr__(self):
        return f"while-loop: ({self.condition}, {self.do})"


class DoNode:
    def __init__(self, condition, do):
        self.condition = condition
        self.do = do
        self.pos_start = self.do.pos_start
        self.pos_end = self.condition.pos_end

    def __repr__(self):
        return f"do-while-loop: ({self.condition}, {self.do})"


class InsNode:
    def __init__(self, name, args, body):
        self.name = name
        self.args = args
        self.body = body

        self.pos_start = self.name.pos_start
        self.pos_end = self.name.pos_end

    def __repr__(self):
        return f"<ins {self.name} get {self.args} ({self.body})>"


class CallNode:
    def __init__(self, name, args):
        self.name = name
        self.args = args
        self.pos_start = self.name.pos_start
        self.pos_end = self.name.pos_end

    def __repr__(self):
        return f"<get {self.name} #{self.args}>"
