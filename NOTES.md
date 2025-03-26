# Some notes for quick refrence from books and online resources

Fours steps to compilation.

1. Lexer: Break the source code into a list of toekns
2. Parser: convert the list of tokens into and AST
3. ASM gen: converts ast into assembly
4. Code Emission: writes the assembly code into a file and pass it into an assembler.

Compiling wrapping steps
1. Preprocesses the source file
gcc -E -P INPUT_FILE -o PROPROCESSED_FILE
2. Compile the preprocesses source file, output an assembly file
Delete the preprocessed file after done
3. Assemble the assembly file,
gcc ASSEMBLY_FILE -o OUTPUT_FILE
Delete assembly file when done

Compiler api
./YOUR_COMPILER /path/to/program.c will produce an executable at /path/to/program
Exit code of zero for success
Following options are supported: --lex --parse --codegen -S to emit assembly file
