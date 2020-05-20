# dmenu-rs - dynamic menu
dmenu is an efficient dynamic menu for X.  
dmenu-rs is a 1:1 port of dmenu rewritten in Rust. It looks, feels, and
runs pixel-for-pixel exactly the same. However, it has a few improvements.

## Why Rust?
### Inspiration
This project started with [`dmenu-calc`](https://github.com/sumnerevans/menu-calc).
Initially, I wanted much more function than what is provided by `bc`. However, I
found the bottleneck to be a lack of functionality and modability in `dmenu(1)`
itself. So, the choice I had was to either mod `dmenu(1)` or rewrite it. Because
dmenu source is horrendously annoying to read, I decided to rewrite it in a
language which lends itself to writing code that is easier to modify. There are
other languages for this, but I like Rust.
### Improvements
As mentioned earlier, `dmenu-rs` runs exactly the same as `dmenu`. However, there
are some significant performance enhancements under the hood. The most impactful
is memmory usage: `dmenu-rs` uses 21.65% less memmory<sup>[1]</sup>, while managing it much
more safely **without** any performance impacts.

## Requirements
- Xlib header files  
- Cargo / rustc  
- A working C compiler

## Installation
### Standalone
Edit config.mk to match your local setup (dmenu is installed into
the /usr/local namespace by default).

Afterwards enter the following command to build and install dmenu
(if necessary as root):  
```make clean install```

## Running dmenu
See the man page for details. For a quick test, run:  
```make test```

<br/><br/>
<sup>[1]</sup>: According to `valgrind(1)`