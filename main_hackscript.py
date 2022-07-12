import Hacklexer
import Hackparser
import Hackinterpreter
from hacktypes.impor_type import *
from ins_def import *


symbol_table = SymbolTable()
memory = ListofMemory(symbol_table)

Method.change_status = Method("change_status", memory)
Method.exit = Method("exit", memory)
Method.clear = Method("clear", memory)
Method.set_constant = Method("set_constant", memory)

symbol_table.set("null", Number.null)
symbol_table.set("false", Number.false)
symbol_table.set("true", Number.true)
symbol_table.set("!", Method.change_status)
symbol_table.set("exit", Method.exit)
symbol_table.set("clear", Method.clear)
symbol_table.set("s", Method.set_constant)


def run(text, fn):
    global memory, symbol_table
    lexer = Hacklexer.Lexer(fn, text)
    tokens, error = lexer.make_tokens()
    print(tokens)

    # return tokens, error
    if error:
        return None, error

    parser = Hackparser.Parser(tokens)
    ast = parser.parse()
    print(ast.node)
    if ast.error:
        return None, ast.error

    intepreter = Hackinterpreter.Interpreter(memory, symbol_table)
    context = Context("<program>")

    res = intepreter.visit(ast.node, context)

    return res.value, res.error
