# Things to implement

- Create ast data type and ast parser

- Create custom compiler error types
  now implement it 

- [x] Rewrite the Return enum used in the compile command
- Additionally the error printout seems a little shaky, will need to check that and make sure everything is always legible as english

- [x] Implement shell argument parsing
- [x]  Implement a lexer & lexing data type
  - Is a vector holding the enum type of token
  - Lexer enums have token type and token content in the data attached
  - Token info is a struct that hold metadata of each token and the token enum itself line and info for err reporting?
