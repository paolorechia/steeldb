# SteelDB

This is a study repository. This is mostly for personal use.

## Useful Links:
1. https://cstack.github.io/db_tutorial/parts/part1.html
2. https://www.sqlite.org/arch.html
3. https://build-your-own.org/database/


## What do we need to build a Database like SQLite from scratch?

1. A REPL shell
3. A tokenizer
4. A parser
5. A code generator
6. A virtual machine that interprets the generated code
7. A B+ Tree
8. Pager
9. OS Interface


## How do we build each of these components in Rust?


### REPL Shell

This is pobably the simplest component. We just need to implement a CLI shell that reads lines of input until the command end symbol is presented (like ';').
It then forwards the parsed string into the next layers of our system, and returns the result.


### A Tokenizer and A Parser
One should be able to generate them using https://github.com/lalrpop/lalrpop.
Tutorial: http://lalrpop.github.io/lalrpop/tutorial/001_adding_lalrpop.html

### A Code Generator
