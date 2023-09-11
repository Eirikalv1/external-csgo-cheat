#External CSGO cheat application

##About
Rust application using raw winapi bindings.
The purpose of this program is to use memory manipulation to modify the game, nothing advanced. 
Is C++ a better option for doing this? Yes, yes it is...

This is just for educational/testing purposes. 

###This program requires to be run in 32-bit mode:

rustup target install i686-pc-windows-msvc

cargo run --target=i686-pc-windows-msvc
