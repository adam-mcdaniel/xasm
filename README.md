# xasm
A cross platform, compiled and dynamically typed programming / intermediate language

Xasm is meant to be an intermediate representation for compiled and dynamically typed programming languages. However, xasm is still itself suitable for scripting.

## Features
- Dynamic typing
- Lua-like speeds
- An easy to use Rust foreign function interface
- First class support for Functional and Object Oriented programming
- Simple syntax

## Documentation
You can find the virtual machine's documentation (here)[https://docs.rs/xmachine], and the compiler backend documentation (here)[https://docs.rs/xassembler]. Both of these components are called `xmachine` and `xassembler` respectively, and you can find them on my GitHub account.

```bash
$ xasm

xasm x.x.x
Adam McDaniel <adam.mcdaniel17@gmail.com>
Compiler for the xasm programming language

USAGE:
    xasm [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    compile    Compile a xasm file
    help       Prints this message or the help of the given subcommand(s)
    run        Run a xasm file
```

## Examples

Here's how you implement basic control flow structures in xasm.

```rust
value = 1

if value {
    println("`value` is not 0")
} else {
    println("`value` is 0")
}


while 1 {
    println("Still looping!")
}
```

Keep in mind that xasm is intended to be an intermediate representation!

#### Object Oriented Programming
To write object oriented code in xasm, first you write a class.
```rust
class Point {
    fn new(self, x, y) {
        self.goto(x, y)
        self
    }

    self.goto(self, x, y) {
        self.x = x
        self.y = y
    }
}
```

The `new` method is typically used to instantiate an object, but you can write other constructors if you'd like. The reason we use the `new` method is because the `new` function will call our class's `new` function to construct our object.

Here's how we would construct our `Point`.

```rust
p = new(Point, 2, 3)
```

#### Functional Programming
Because xasm supports closures, you can easily implement church encodings.

```rust
fn True(a) {
    fn(b) { a }
}

fn False(a) {
    fn(b) { b }
}

fn If(c) {
    fn(a) {
        fn(b) {
            (c(a))(b)
        }
    }
}
```

In addition, you can use more practical functional programming techniques.

```rust
multiply = fn(n) {
    fn(m) {
        mul(n, m)
    }
}

double = multiply(2)
triple = multiply(3)

println(double(3))
println(triple(3))
```

## Installation

Install Rust

```bash
# For *nix users
# If you're a windows user, go to https://rust-lang.org
curl https://sh.rustup.rs -sSf | sh
```

Install / update xasm

```bash
cargo install -f xasm
```

## Issues

If you run into a problem, (post an issue)[https://github.com/adam-mcdaniel/xasm/issues/new]!

## License

xasm is distributed under the terms of the Apache License (Version 2.0).

See LICENSE for details.