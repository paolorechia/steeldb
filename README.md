# SteelDB

This is a study repository. This is mostly for personal use. Building a Database from scratch in Rust. Why not? :)

## Useful Links:
1. https://cstack.github.io/db_tutorial/parts/part1.html
2. https://www.sqlite.org/arch.html
3. https://build-your-own.org/database/


# Roadmap
This is not a binding roadmap.

## What do we need to build a Database like SQLite from scratch?

1. A REPL shell
3. A tokenizer
4. A parser
5. A code generator
6. A virtual machine that interprets the generated code
7. A B+ Tree
8. Pager
9. OS Interface


### For the first iteration: the bare bones (v0.1.0)
We can simplify some components in the first iteration, so we have first a working end-to-end system.
We can then tweak the individual components to have more capabilities.

Here are some simplifications we can do for our first iteration:

1. Support only a subset of the SQL Syntax, for instance, start with only insert and select operations.
2. Do not implement a B+ tree in the first iteration. Instead, use a vector or list of structs. 
3. Do not handle Pager in first iteration.
4. Keep the database persisted into a single file.
5. Use a single statically defined table.

Our roadmap for the first iteration might end up looking like this:

1. A REPL shell [x]
3. A tokenizer [x]
4. A parser [x]
5. Add support for the SELECT clause [x]
6. A code generator [x]
7. A virtual machine that interprets the generated code [x]
8. A table struct that stores data in HashMap of Vectors [x]
9. A hardcoded table for testing [x]
10. Proper error propagation [x]
11. Pretty printing of table in REPL [x]
12. A serialization / desserialization method to write/read data from file [x]
13. Clean up [x]
    * Handle select columns properly in read method [x]
    * Test that everything is working as expected [x]
    * Tag v1.0 [x]


### Second iteration: making it usable (v0.2.0)
1. Add another API besides the REPL to query the database []
   * This can be either a traditional TCP or a HTTP server. It should be as simple as possible, and just receive a string of the SQL command
   * Make REPL support both backends: Standalone process or network server
2. Adds proper logging strategy to the server []
3. Add configuration file []
4. Add create table command []
5. Add drop table command []
6. Add alter table command []
7. Multiple tables query support (add FROM clause support) []
8. Support filters (add basic WHERE clause support) []



### Third iteration: making it scalable (v0.3.0)
1. Handle concurrency: needs research on approaches to use []
3. Implement B+ or similar algorithm.
   * Ideally keep support for current columnar support []
   * Support both in-memory and persistent
4. Adds caching (or pagination) []
5. Support for transactions []


### Fourth iteration: making it useful (v0.4.0)
1. Implement inner join []
2. Implement left / right join []
3. Implement outer join []
4. Implement nested operations, including WHERE IN (SELECT) []
4. Implement aggregations []



### Fifth iteration: making it time-aware (v0.5.0)
1. Implement advanced SQL features
  * Window []
  * Having []
3. Add date and timestamp types []
4. Implement more SQL functions []


### Sixth iteration: making it complete (v0.6.0)
1. Anything important missing in SQL standards



### Seventh iteration: making it compatible (v0.7.0)
1. Implement the Spark.SQL API in Rust


### Eighth iteration: making it attractive (v0.8.0)
1. Build a SDK in Rust to use it


### Nineth iteration: making it snaky (v0.9.0)
1. Build Python bindings for the Rust SDK


### Tenth iteration: placeholder (v1.0.0)
If we reached this point, we actually have an impressive system :)


# Knowledge base

## How do we build each of these components in Rust?

### REPL Shell
This is pobably the simplest component. We just need to implement a CLI shell that reads lines of input until the command end symbol is presented (like ';').
It then forwards the parsed string into the next layers of our system, and returns the result.


### A Tokenizer and A Parser
One should be able to generate them using https://github.com/lalrpop/lalrpop.
Tutorial: http://lalrpop.github.io/lalrpop/tutorial/001_adding_lalrpop.html


### A Code Generator
This should be somehow integrated into the parser. It will become clearer once the first parser is built, how this can be written (if possible, avoiding too much coupling).


### Virtual Machine
Probably the easiest way is to implement it as a stack, so it can handled nested commands.
For each parsed command, we push it to the stack, and the virtual machine pop it, executing it.
This works assuming we parse the commands in a depth-first way, e.g., the most inner command is always parsed first, and it's result is available to the next command execution.
This should happen naturally using an auto-generated lexer/parser with lalrpop, as long as the grammar is correctly defined.

To keep it simple, however, we should start not supporting nested commands. We can still have the stack in place and ready for extension in the future.

### B+ Tree
The B+ Tree is wide-known algorithm, and is described in several places, for instance: https://en.wikipedia.org/wiki/B%2B_tree
This will be skipped until much later! The goal is to first get an usable system/product that supports concurrency and possibly exposes an (HTTP) API to query data.
This means things like transactions have to be possibly implemented first.  
