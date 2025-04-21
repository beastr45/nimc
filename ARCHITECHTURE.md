# Project architecture laid out:

cli -> handles command line processing
wrapper -> handles running compiler commands
compiler -> main compilation logic, will run the compilation pipeline
lexer -> convert source code into tokens
parser -> convert tokens into an ast
ast-> ast node definitions
codegen -> handle code generation

diagnostics -> add custom error types handling, pretty print errors

later things:
semantic -> type
