# Personal notes

## conventions

snake_case for variable and function names

## shadowing

reuse variable names with different type from a point to another in the code (see guessing_game)

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