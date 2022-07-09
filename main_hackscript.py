import Hacklexer
import Hackparser
import Hackinterpreter
from hacktypes.impor_type import *


Number.null = Number(0)
Number.true = Number(1)
Number.false = Number(0)
symbol_table = SymbolTable()
memory = ListofMemory(symbol_table)
symbol_table.set("null", Number.null)
symbol_table.set("false", Number.false)
symbol_table.set("true", Number.true)


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
