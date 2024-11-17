# Rocket Safe 🚀

Static analyser for shipping to the moon.

## How it works

Using the Abstract Syntax Tree generated by Clang, the program recursively traverses this tree to detect any C code
that violates a select set of rules from [NASA's Power of 10](https://en.wikipedia.org/wiki/The_Power_of_10:_Rules_for_Developing_Safety-Critical_Code). These standards are designed to ensure that embedded code written in C for safety-critical applications, such as those used in rockets, is as easy to debug and as free from errors as possible.

The set of rules currently supported are:

- No complex control flow such as goto, longjmp and setjmp
- No recursion
- No heap allocation
- No global variables
- Check all function return values or cast to void if the return value is useless
- Limit pointer use to a single dereference

## Example

```
cargo run -- path/to/main.c
```

**Output**

```
Dynamic memory allocation at line 7 column 3 in "main.c"
Function fibonacci called recursively at line 9 column 5 in "main.c"
goto usage at line 16 column 5 in "main.c"
goto usage at line 18 column 5 in "main.c"
return value of printf ignored at line 20 column 3 in "main.c". If the function does not return anything it should be cast to void.
```

## Running

1. Build the project with cargo build
2. Test with cargo test
3. Run with cargo run -- `path/to/main.c`
