
import error.error_message as error_message


class Error:
    def __init__(self, pos_start, pos_end, error_name, details):
        self.pos_start = pos_start
        self.pos_end = pos_end
        self.error_name = error_name
        self.details = details

    def as_string(self):
        str_to_ret = "HackScript Received: Error"
        str_to_ret += "\nHere: \n" + \
            error_message.string_highlight(
                self.pos_start.fcontent, self.pos_start, self.pos_end)
        str_to_ret += f"\n(Index: {self.pos_start.line+1}-{self.pos_start.column+1}, file: {self.pos_start.fname})"
        str_to_ret += f"\n{self.error_name}: {self.details}"

        return str_to_ret


class IllegalCharacter(Error):
    def __init__(self, pos_start, pos_end, details):
        super().__init__(pos_start, pos_end, "IllegalCharacter", details)


class SyntaxError(Error):
    def __init__(self, pos_start, pos_end, details):
        super().__init__(pos_start, pos_end, "SyntaxError", details)


class NoVisitError(Error):
    def __init__(self, pos_start, pos_end, details):
        super().__init__(pos_start, pos_end, "NoVisitError", details)


class InvalidIndexOfMemory(Error):
    def __init__(self, pos_start, pos_end, details):
        super().__init__(pos_start, pos_end, "InvalidIndexOfMemory", details)


class OperatorNotSupported(Error):
    def __init__(self, pos_start, pos_end, details):
        super().__init__(pos_start, pos_end, "OperatorNotSupported", details)


class UndefinedObject(Error):
    def __init__(self, pos_start, pos_end, details):
        super().__init__(pos_start, pos_end, "UndefinedObject", details)


class RuntimeError(Error):
    def __init__(self, pos_start, pos_end, details, context):
        super().__init__(pos_start, pos_end, "RuntimeError", details)
        self.context = context

    def as_string(self):
        str_to_ret = "HackScript Received: Error"
        str_to_ret += "\nHere: \n"

        str_req = self.generate_traceback()
        if str_req == '':
            str_to_ret += f"\n{error_message.string_highlight(self.pos_start.fcontent, self.pos_start, self.pos_end)}"
            str_to_ret += f"\n(Index: {self.pos_start.line+1}-{self.pos_start.column+1}, file: {self.pos_start.fname})"
        else:
            str_to_ret += str_req
        str_to_ret += f"\n{self.error_name}: {self.details}"
        return str_to_ret

    def generate_traceback(self):
        result = ''
        pos = self.pos_start
        ctx = self.context

        while ctx:
            result += \
                f"\n{error_message.string_highlight(pos.fcontent, pos, pos.copy().advance())}"
            result += f'\n(Index: {str(pos.line + 1)}-{str(pos.column+1)}, file: {pos.fname}, in {ctx.display_name})\n'
            pos = ctx.parent_entry_pos
            ctx = ctx.parent

        return result
