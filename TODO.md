# Things to implement

- Implement shell argument parsing
- Rewrite the Return enum used in the compile command
- Implement a lexer & lexing data type
  - Is a vector holding the enum type of token
  - Lexer enums have token type and token content in the data attached
  - Token info is a struct that hold metadata of each token and the token enum itself line and info for err reporting?
