# SanScript
Programming language for BadUSB devices

## Setup
To get started, install Rust on your machine. Check out [official Rust guide](https://www.rust-lang.org/tools/install) for more info.<br>

## Build
To build the project run the following command:
- ```cargo build```

## Running the binary
SanScript is organised as a Rust workspace, meaning each directory is actually a Rust project. There are currently two binaries in the workspace (SanScript-Frontend and SanVM), 
so to run the project you must specify which binary you would like to run with the following command:
- ```cargo run --bin name_of_project```
