# Project architecture laid out:

cli -> handles command line processing
diagnostics -> add custom error types handling, pretty print errors
lexer -> convert source code into tokens
parser -> convert tokens into an ast
ast-> ast node definitions
codegen -> handle code generation

later things:
semantic -> type
