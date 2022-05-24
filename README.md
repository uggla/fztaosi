# From Zero To A Old School Intro

This is a live demo/lab to discover Rust smoothly by creating an
[intro](https://en.wikipedia.org/wiki/Crack_intro).

Disclaimer, no work like striping will be done to reduce the size of the
produced binary and meet the usual binary size of such a program.

Lab steps:

1. Welcome, goal of the lab
    * Show that rust is not so difficult
    * Learn some rust concept working on a simple visual program
    * Disclaimer, we can not go into all details, some parts will not be
      explained

2. Quickly explain what is an intro

3. Quickly explain Rust benefits and particularities
    * alternative to C and C++ Mozilla
    * safety
    * no GC
    * fast ideal for Game, OS, mainly low level programming...

4. Quickly explain Macroquad

5. Install rustup, cargo, rust and clippy.

6. Explain rust-analyzer and editor integration.

7. cargo init --bin

8. Hello world, cargo build, cargo run

9. Hello macroquad
    * dep in Cargo.toml
    * dep for Linux
    * prelude
    * main loop

    See "initial commit" commit

10. Draw a first star
    * introduce let
    * introduce screen_width(), screen_height()
    * float
    * draw_circle
    * color ?

11. Center function
    * introduce function
    * introduce Vec2 -> show doc src ?

12. Create a star object
    * introduce struct
    * introduce impl, constructor and method
    * start introducing block and explain objects deletion
    * create update_pos method
    * create gen_val function

    See "Create star object" commit

TBC...
