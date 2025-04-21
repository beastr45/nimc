// lexer pseudo code

// Token data type
~ Identifiers are ascii letter or underscore followed by letters underscores or digits
~ Constants are one or more digits
Enumeration token type->
Identifier(word): [a-zA-Z_]\w*\b
Constant(number): [0-9]+\b
int Keyword: int\b
void Keyword: void\b
return Keyword: return\b
open paren: \(
close paren: \)
open brace: {
close brace: }
Semicolon: ;
<- Close Enumeration

// parsing file strings into tokens
~ use bufread for input, later a fast library like memmap may be desired
~ 
while input isn't empty:
	if input starts with whitespace:
		trim whitespace from input
	else: 
		~ refactor regex matching with cursor
		find longest match at start of input
		if no match found raise error
		convert the matching substring into a token
		~ iterate regex patterns until one is found at start of string
		add new token to token array
		remove the matching string from the start of input


// Cursor data type
this data type holds:
	Line: token line
	Col: column at token start
// Cursor wraps file reading
// pop next function will return the next token in string
pop next token-> 
