I didn't figure out how to get started on this topic alone, this is all
following the great information provided by Nora Sandler on the topic of writing
a C compiler, I'd highly recommend checking out her [blog
series](https://norasandler.com/2017/11/29/Write-a-Compiler.html) and
[book](https://nostarch.com/writing-c-compiler)!

# How to run

For a C source code file `prog.c`, this can be compiled by running `cargo run
prog.c`:
```
> cat prog.c
int main() {
   return 2;
}
> cargo run prog.c
> cat prog.s
    .globl main
main:
    pushq %rbp
    movq %rsp, %rbp
    subq $0, %rsp
    movl $2, %eax
    movq %rbp, %rsp
    popq %rbp
    ret
```

The output file stem (filename without extension) will be the same as the input
file. Ie, `test.c` would be compiled to `test.s`.

# What is supported

## Target architecture

This is only targeting x86-64.

## C language features

Currently, this only supports compiling C programs that use a *very* limited
subset of the C programing language features:
- a single (main) function
- a single return statement
- the return statement can contain nested applications of unary operators, but
the innermost operand must be a numerical literal
