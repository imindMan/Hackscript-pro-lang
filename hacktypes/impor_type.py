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

    def copy(self):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot copy this object"
        )

    def assign_from(self, other):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def push(self, other):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def delete(self, other):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def change_status(self, other):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def attribute(self, other):
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Undefined attribute"
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

    def attribute(self, other):
        if isinstance(other, Identifier):
            if other.value == "value":
                return Number(self.value).set_context(self.context), None

        return None, error.InvalidObject(
            self.pos_start, self.pos_end,
            "Invalid parameter"
        )

    def assign_from(self, other):
        if isinstance(other, Number):
            self.value = other.value

            return self.set_context(self.context), None
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def __repr__(self):

        return f"{self.value}"


class Memory(Value):
    def __init__(self, name):
        super().__init__(name)
        self.name = ClassString(name)
        self.value = []
        self.status = ClassString("200")
        self.attributes = {
            "value": List(self.value),
            "status": self.status,
            "name": self.name
        }

    def push(self, data):
        if self.status.value == "200":
            self.status.value = "210"
            self.value.append(data)

            self.status.value = "220"

    def delete(self, index=None):
        if self.status.value == "200":
            self.status.value = "210"
            try:
                if index == None:
                    self.value = []
                self.value.remove(self.value.value[index])
            except:
                return error.InvalidIndexOfMemory(
                    self.pos_start, self.pos_end,
                    "Invalid index because data doesn't have that much memory"
                )

            finally:

                self.status.value = "230"

    def change_status(self, status):
        if int(status.value) > 230:
            return error.InvalidStatus(
                self.pos_start, self.pos_end,
                f"Invalid status {status}"
            )
        self.status.value = str(status.value)
        return None

    def copy(self):
        memory = Memory(self.name)
        memory.value = self.value
        memory.status = self.status
        memory.set_pos(self.pos_start, self.pos_end)
        memory.set_context(self.context)
        return memory

    def attribute(self, other):
        if isinstance(other, Identifier):
            if self.attributes.get(other.value, None) is None:
                return None, error.InvalidObject(
                    self.pos_start, self.pos_end,
                    "Undefined attribute"
                )

            elif other.index != None:

                return_value = self.attributes[other.value]
                if len(return_value.value) == 1:
                    return ClassString(return_value.value[0].value[other.index.value]), None
                elif other.index.value < 0 or other.index.value > len(return_value.value):
                    return None, error.InvalidObject(

                        self.pos_start, self.pos_end,
                        "Invalid index specified"
                    )
                try:
                    return return_value.value[other.index.value], None
                except:
                    return None, error.InvalidObject(
                        self.pos_start, self.pos_end,
                        "Invalid index specified"
                    )
            else:
                if len(self.attributes[other.value].value) == 1:
                    return self.attributes[other.value].value[0], None
                else:
                    return self.attributes[other.value], None

    def __repr__(self):
        return f"data:{self.name} {self.value}({self.status})"


class ListofMemory:
    def __init__(self, symbols_table, launch_table, parent_list_of_memory=None):

        self.value = [Memory(str(i)) for i in range(10)]
        self.index = 0
        self.curr_char = self.value[self.index] if 0 <= self.index < len(
            self.value) else None
        self.symbols_table = symbols_table
        self.parent_list_of_memory = parent_list_of_memory
        self.launch_table = launch_table
        self.set_pos()
        self.set_context()

    def change_status(self, status):
        self.curr_char.change_status(status)

    def set_pos(self, pos_start=None, pos_end=None):
        self.pos_start = pos_start
        self.pos_end = pos_end
        return self

    def set_context(self, context=None):
        self.context = context
        return self

    def set_constant(self, name):
        self.curr_char.name = name
        index = self.index
        if index + 1 >= len(self.value):
            self.value.append(Memory(str(f"{index + 1}")))
            self.index += 1
            temp_data = self.curr_char.value
            self.curr_char.value = []
            self.curr_char = self.value[self.index] if 0 <= self.index < len(
                self.value) else None
            self.curr_char.value = temp_data
        else:
            self.index += 1
            temp_data = self.curr_char.value
            self.curr_char.value = []
            self.curr_char = self.value[self.index] if 0 <= self.index < len(
                self.value) else None
            self.curr_char.value = temp_data
        self.symbols_table.set(name, "cons-pointer")

    def access_constant(self, name):
        for i in self.value:
            if i.name == name:
                return i
        return ClassString("Error catching while defined constant pointer")

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
        if index < 0 or index >= len(self.value):

            index = old_index
            return None, error.InvalidIndexOfMemory(
                self.pos_start, self.pos_end,
                "Invalid index because data doesn't have that much memory"
            )

        curr_char = self.value[index]

        return (index, curr_char), None

    def copy(self):
        list_of_memory = ListofMemory(self.symbols_table, self.launch_table)
        list_of_memory.value = self.value
        list_of_memory.index = self.index
        list_of_memory.curr_char = list_of_memory.value[list_of_memory.index] if list_of_memory.index < len(
            list_of_memory.value) else None
        list_of_memory.set_pos(self.pos_start, self.pos_end)
        list_of_memory.set_context(self.context)
        return list_of_memory

    def __repr__(self):
        return str(self.value)


class Pointer(Value):
    def __init__(self, type, list_of_memory):
        super().__init__(type)
        self.type = type
        self.list_of_memory = list_of_memory
        self.attributes = {
            "value": self.list_of_memory.curr_char,
            "list_memo": self.list_of_memory
        }

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

    def attribute(self, other):
        if isinstance(other, Identifier):
            if self.attributes.get(other.value, None) is None:
                return None, error.InvalidObject(
                    self.pos_start, self.pos_end,
                    "Undefined attribute"
                )

            elif other.index != None:

                return_value = self.attributes[other.value]
                if len(return_value.value) == 1:
                    return ClassString(return_value.value[0].value[other.index.value]), None
                elif other.index.value < 0 or other.index.value > len(return_value.value):
                    return None, error.InvalidObject(

                        self.pos_start, self.pos_end,
                        "Invalid index specified"
                    )
                return return_value.value[other.index.value], None
            else:
                if len(self.attributes[other.value].value) == 1:
                    return self.attributes[other.value].value[0], None
                else:
                    return self.attributes[other.value], None

    def __repr__(self):

        return f"pointer:{self.type}({self.list_of_memory.index}, {self.list_of_memory.curr_char})"


class ConstantPointer(Value):
    def __init__(self, type, list_of_memory):
        super().__init__(type)
        self.type = type
        self.list_of_memory = list_of_memory
        self.attributes = {
            "value": self.list_of_memory.access_constant(self.type.value)
        }

    def copy(self):
        return Pointer(self.type, self.list_of_memory)

    def attribute(self, other):
        if isinstance(other, Identifier):
            if self.attributes.get(other.value, None) is None:
                return None, error.InvalidObject(
                    self.pos_start, self.pos_end,
                    "Undefined attribute"
                )

            elif other.index != None:

                return_value = self.attributes[other.value]
                if len(return_value.value) == 1:
                    return ClassString(return_value.value[0].value[other.index.value]), None
                elif other.index.value < 0 or other.index.value > len(return_value.value):
                    return None, error.InvalidObject(

                        self.pos_start, self.pos_end,
                        "Invalid index specified"
                    )
                return return_value.value[other.index.value], None
            else:
                if len(self.attributes[other.value].value) == 1:
                    return self.attributes[other.value].value[0], None
                else:
                    return self.attributes[other.value], None

    def __repr__(self):
        return self.list_of_memory.access_constant(self.type.value).__repr__()


class Identifier(Value):
    def __init__(self, value, index=None):
        super().__init__(value)
        self.value = value
        self.index = index

    def copy(self):
        identifier = Identifier(self.value).set_pos(
            self.pos_start, self.pos_end).set_context(self.context)
        return identifier


class ClassString(Value):
    def __init__(self, value):
        super().__init__(value)
        self.value = value
        self.attributes = {
            "value": self.value
        }

    def added_to(self, other):
        if isinstance(other, ClassString):
            res = self.value + other.value
            return ClassString(res).set_context(self.context), None
        else:
            return None, error.OperatorNotSupported(
                self.pos_start, self.pos_end,
                "Cannot using this operator in this expression"
            )

    def multiplied_to(self, other):
        if isinstance(other, Number):
            res = self.value * other.value
            return ClassString(res).set_context(self.context), None
        else:
            return None, error.OperatorNotSupported(
                self.pos_start, self.pos_end,
                "Cannot using this operator in this expression"
            )

    def equ_to(self, other):
        return Number(int(self.value == other.value)).set_context(self.context), None

    def not_equ_to(self, other):
        return Number(int(self.value != other.value)).set_context(self.context), None

    def gre_to(self, other):
        return Number(int(len(self.value) > len(other.value))).set_context(self.context), None

    def les_to(self, other):
        return Number(int(len(self.value) < len(other.value))).set_context(self.context), None

    def gre_equ_to(self, other):
        return Number(int(len(self.value) >= len(other.value))).set_context(self.context), None

    def les_equ_to(self, other):
        return Number(int(len(self.value) <= len(other.value))).set_context(self.context), None

    def and_to(self, other):
        return Number(int(self.value != "" and other.value != "")).set_context(self.context), None

    def or_to(self, other):

        return Number(int(self.value != "" or other.value != "")).set_context(self.context), None

    def not_to(self):
        return Number(int(self.value != "")).set_context(self.context), None

    def assign_from(self, other):
        if isinstance(other, ClassString):
            self.value = other.value

            return self.set_context(self.context), None
        return None, error.OperatorNotSupported(
            self.pos_start, self.pos_end,
            "Cannot using this operator in this expression"
        )

    def attribute(self, other):
        if isinstance(other, Identifier):
            if self.attributes.get(other.value, None) is None:
                return None, error.InvalidObject(
                    self.pos_start, self.pos_end,
                    "Undefined attribute"
                )

            elif other.index != None:

                return_value = self.attributes[other.value]
                if len(return_value.value) == 1:
                    return ClassString(return_value.value[0].value[other.index.value]), None
                elif other.index.value < 0 or other.index.value > len(return_value.value):
                    return None, error.InvalidObject(

                        self.pos_start, self.pos_end,
                        "Invalid index specified"
                    )
                return return_value.value[other.index.value], None
            else:
                if len(self.attributes[other.value].value) == 1:
                    return self.attributes[other.value].value[0], None
                else:
                    return self.attributes[other.value], None

    def copy(self):
        string_ = ClassString(self.value).set_pos(
            self.pos_start, self.pos_end).set_context(self.context)
        return string_

    def __str__(self):
        return f"{self.value}"


class List(Value):
    def __init__(self, value):
        super().__init__(value)
        self.attributes = {
            "value": self.value
        }

    def copy(self):
        list_ = List(self.value)
        list_.set_pos(self.pos_start, self.pos_end)
        list_.set_context(self.context)
        return list_

    def attribute(self, other):
        if isinstance(other, Identifier):
            if self.attributes.get(other.value, None) is None:
                return None, error.InvalidObject(
                    self.pos_start, self.pos_end,
                    "Undefined attribute"
                )

            elif other.index != None:

                return_value = self.attributes[other.value]
                if len(return_value.value) == 1:
                    return ClassString(return_value.value[0].value[other.index.value]), None
                elif other.index.value < 0 or other.index.value > len(return_value.value):
                    return None, error.InvalidObject(

                        self.pos_start, self.pos_end,
                        "Invalid index specified"
                    )
                return return_value.value[other.index.value], None
            else:
                if len(self.attributes[other.value].value) == 1:
                    return self.attributes[other.value].value[0], None
                else:
                    return self.attributes[other.value], None

    def __repr__(self):
        str_to_return = "{"
        for i in range(len(self.value)):
            if i == len(self.value) - 1:
                str_to_return += self.value[i].__repr__()
            else:
                str_to_return += self.value[i].__repr__() + ", "
        str_to_return += "}"
        return str_to_return


Number.null = Number(0)
Number.false = Number(0)
Number.true = Number(1)


class PlaceHolder(Value):
    def __init__(self):
        super().__init__(Number.null)
        self.attributes = {
            "value": self.value
        }

    def assign_from(self, other):
        self.value = other.copy()
        self.attributes = {
            "value": self.value
        }

        return self, None

    def attribute(self, other):
        if isinstance(other, Identifier):
            if other.value == "value":
                return self.attributes.get(other.value), None
            else:
                return None, error.InvalidObject(
                    self.pos_start, self.pos_end,
                    "Undefined attribute"
                )

    def copy(self):
        placeholder = PlaceHolder()
        placeholder.attributes = self.attributes
        placeholder.set_pos(self.pos_start, self.pos_end)
        placeholder.set_context(self.context)
        return placeholder

    def __repr__(self):
        place_string = self.value if self.value != Number.null else "null"
        return f"<placeholder: {place_string}>"


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
    def __init__(self, token, index=None):
        self.token = token
        self.index = index
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


class StringNode:
    def __init__(self, token, value, index=None):
        self.value = value
        self.index = index
        self.pos_start = token.pos_start
        self.pos_end = token.pos_end

    def __repr__(self):
        return f"{repr(self.value)}"


class ListNode:
    def __init__(self, token, value, index=None):
        self.value = value
        self.index = index
        self.pos_start = token.pos_start
        self.pos_end = token.pos_end

    def __repr__(self):
        str_to_return = "{"
        for i in range(len(self.value)):
            if i == len(self.value) - 1:
                str_to_return += self.value[i].__repr__()
            else:
                str_to_return += self.value[i].__repr__() + ", "
        str_to_return += "}"
        return str_to_return


class StatementNode:
    def __init__(self, token, value):
        self.value = value
        self.pos_start = token.pos_start
        self.pos_end = token.pos_end

    def __repr__(self):
        str_to_return = "{"
        for i in range(len(self.value)):
            if i == len(self.value) - 1:
                str_to_return += self.value[i].__repr__()
            else:
                str_to_return += self.value[i].__repr__() + ", "
        str_to_return += "}"
        return str_to_return


class ClassNode:
    def __init__(self, name, attributes, methods, parameter, superclass=None):
        self.name = name
        self.run = attributes
        self.methods = methods
        self.parameter = parameter
        self.superclass = superclass

        self.pos_start = self.name.pos_start
        self.pos_end = self.name.pos_end

    def __repr__(self):
        return f"<class: {self.name}>"


class AttributeNode:
    def __init__(self, name, value=None):
        self.name = name
        self.value = value
        self.pos_start = self.name.pos_start
        self.pos_end = self.name.pos_end

    def __repr__(self):
        return f"{self.name}"


class PlaceHolderNode:
    def __init__(self, name):
        self.name = name
        self.pos_start = self.name.pos_start
        self.pos_end = self.name.pos_end

    def __repr__(self):
        return f"<phd: {self.name}>"
