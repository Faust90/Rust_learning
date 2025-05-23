# Personal notes

Rust is statically typed.

*Panicking* is exiting with an error. 

When a function returns a *Result* or *Option*, error can be propagated with "?". This makes a function end early and return a error message without any boilerplate

Integer overflow is handled through wrapping (restarting from 0 and going on) in release env. This can be handled with some prefixes each of which has its behaviour (wrapping_*, checked_*)

Build a documentation of the project. --open flag will open the built doc on the browser

bringing items in with **use** is usually written with hte full path specified

## Ownership

Memory is managed through ownership rules check at compile time.
Each value in Rust has a owner and only a owner at runtime. When the owner goes out of scope, value is dropped

double free error: when memory of a value is freed twice 
### Stack  
Last in, first out

### Heap
Allocate and Point 

## Traits

Copy and Drop are Traits that special types can be annotated with to implement specific behavious during application phases

## Borrowing

Through reference you can borrow one value. You can only have **either** **one** mutable or **any** immutable references, that **must always** be valid

## Crate
Smallest Rust entity.

### Binary Crate
Is executable and MUST have a fn main(); Usually have just the minimum required code to start the executable that calls the code of the library crate

### Library Crate
Does not have a fn main() and is usable in other crates

## Package

Bundle of one or multiple crates. Needs a Cargo.toml file to explain how to build it

## Module

Organize code and code visibility inside a crate

## conventions

snake_case for variable and function names. 
constants must be in caps lock
variable names preceded by "_" will not give the usual "unused" warning
*methods* are functions within a struct

## shadowing

reuse variable names with different or same type from a point to another in the code (see variables_mutability).
mutable variables **cannot** change type. Through shadowing, this is kind of possibile (but not the same thing)

## constants vs immutable

constants are **always** immutable

## enums

Enums are complex types and cannot be simply iterable 

## Generics

Generics are converted in colpile time to monotypes version. This improves performances


## cargo

Dependency manager and compiler based on *Cargo.toml* and *Cargo.lock* files.
*Cargo.lock* is a snapshot of a working build, it needs to be versioned.

### Commands
#### cargo new <project_name>

generate a "hello world" <project_name> binary crate with a Cargo.toml dependencies file

#### cargo build

compile and generate an executable

#### cargo run

compile, generate an executable and then runs it

#### cargo check

compile without generating any executable

#### cargo update

update all the crates in Cargo.toml ignoring Cargo.lock  

#### cargo doc
