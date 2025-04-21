driver has 3 steps
1. preprocess file
2. compile the preprocessed thing
3. assemble the compiled assembly

the driver will drive the compiler by using the compile function
compile() will lex, parse, gen asm, and code emission.
each output is sent to the next step of pipeline.
These compilation passes will be in their own module.
