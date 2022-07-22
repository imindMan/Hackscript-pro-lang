import Hacklexer
import Hackparser
import Hackinterpreter
from hacktypes.impor_type import *
from ins_def import *
from error.error import *


symbol_table = SymbolTable()
launch_table = {}
memory = ListofMemory(symbol_table, launch_table)


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


def run(text, fn):
    try:
        global memory, symbol_table
        lexer = Hacklexer.Lexer(fn, text)
        tokens, err = lexer.make_tokens()
        print(tokens)

        # return tokens, error
        if err:
            return None, err

        parser = Hackparser.Parser(tokens)
        ast = parser.parse()
        print(ast.node)
        if ast.error:
            return None, ast.error

        intepreter = Hackinterpreter.Interpreter(memory, symbol_table)
        context = Context("<program>")

        res = intepreter.visit(ast.node, context)

        return res.value, res.error
    except RecursionError:
        return None, RuntimeError(
            ast.node.pos_start, ast.node.pos_end,
            "Maximum recursion depth exceeded", context
        )
