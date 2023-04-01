# Computerraria Rust Template

This is a template for building Rust applications targeting [Computerraria](https://github.com/misprit7/computerraria). 

## Setup

To set up a new project, run the following: 
```bash
cp -r template PATH/NAME_OF_PROJECT
cd PATH/NAME_OF_PROJECT
sed -i s/template/NAME_OF_PROJECT/ Cargo.toml copy_bin.sh .vimspector.json
cargo build --release
```
To actually use the project, you can convert the file to the required [WireHead](https://github.com/misprit7/WireHead) file format with the provided script: 
```bash
./copy_bin.sh /tmp/in.txt
```
You can then load it into Computerraria with the following command, written in the in-game chat (NOT your terminal):
```
/bin write /tmp/in.txt
```
Once you start the clock the screen should update to be on in the outer horizontal border.

## Development
The entrance point of the program is `main` in [`src/main.rs`](src/main.rs), make changes there.
