# Personal notes

Rust is statically typed.

*Panicking* is exiting with an error.

Integer overflow is handled through wrapping (restarting from 0 and going on) in release env. This can be handled with some prefixes each of which has its behaviour (wrapping_*, checked_*)

## conventions

snake_case for variable and function names. 
constants must be in caps lock
variable names preceded by "_" will not give the usual "unused" warning

## shadowing

reuse variable names with different or same type from a point to another in the code (see variables_mutability).
mutable variables **cannot** change type. Through shadowing, this is kind of possibile (but not the same thing)

## constants vs immutable

constants are **always** immutable

## cargo

Dependency manager and compiler based on *Cargo.toml* and *Cargo.lock* files.
*Cargo.lock* is a snapshot of a working build, it needs to be versioned.

### Commands
#### cargo new

generate a "hello world" project with a Cargo.toml dependencies file

#### cargo build

compile and generate an executable

#### cargo run

compile, generate an executable and then runs it

#### cargo check

compile without generating any executable

#### cargo update

update all the crates in Cargo.toml ignoring Cargo.lock  

#### cargo doc

Build a documentation of the project. --open flag will open the built doc on the browser