# From Zero To A Old School Intro

This is a live demo/lab to discover Rust smoothly by creating an
[intro](https://en.wikipedia.org/wiki/Crack_intro).

Disclaimer, no work like striping will be done to reduce the size of the
produced binary and meet the usual binary size of such a program.
However this could be done in a future release.

Lab steps:

1. Welcome, goal of the lab
    * Show that rust is not so difficult
    * Learn some rust concept working on a simple visual program
    * Disclaimer, we can not go into all details, some parts will not be
      explained

2. Audience question about people backgroud in order adapt speed and details

2. Quickly explain what is an intro

3. Quickly explain Rust benefits and particularities
    * alternative to C and C++ Mozilla
    * safety, strict, try to avoid developer mistakes
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
    * introduce impl, constructor and methods
    * introduce mutability
    * create update_pos method
    * create gen_val function

    See "Create star object" commit

13. Update color and size of object

14. Create starfield
    * iterate over a Vec  --> show pb we need a reference
    * start introducing block and explain objects deletion
    ```rust
    fn main() {
    let myvec = vec![1, 2, 3, 4];

    dbg!(myvec);
    }
    ```

    ```rust
    fn main() {
        {
            let myvec = vec![1, 2, 3, 4];
        }

    dbg!(myvec);
    }
    ```
    Above code doesn't compile myvec out of scope

    * Explain 1 memory problem (double free)
        * Explain memory is organized in 2 parts stack (stack of plates), heap (shelve)
        * stack --> known + fixed size heap --> unknow size, or size that can change.
        * schema showing type on the head (Vec) use a pointer on the stack and are shallow copy to save perf
        ```rust

        fn main() {
            let myvec = vec![1, 2, 3, 4];
            let myvec2 = myvec;

            dbg!(myvec2);
            dbg!(myvec);
        }
        ```
        Above code without ownership will free twice the same memory

    * Explain ownership prevent above error kind and more
        * Above code will not compile as myvec is moved
        * Ownership is a set of rules to avoid memory errors.
            * Each value in Rust has a variable that’s called its owner.
            * There can only be one owner at a time.
            * When the owner goes out of scope, the value will be dropped.
            * Assigning a value to a new variable will move ownership.
        * How to deal with ownership
            * cloning (deep copy)
            ```rust
            fn main() {
                let myvec = vec![1, 2, 3, 4];
                let myvec2 = myvec.clone();

                dbg!(myvec2);
                dbg!(myvec);
            }
            ```
            * borrowing
            ```rust
            fn main() {
                let myvec = vec![1, 2, 3, 4];
                let myvec2 = &myvec;

                dbg!(myvec2);
                dbg!(myvec);
            }
            ```
            * At any given time, you can have either one mutable reference or any number of immutable references.
            * References must always be valid.
            * update memory schema ref --> stack --> heap
            ```rust
            fn main() {
                let mut myvec = vec![1, 2, 3, 4];
                let myvec2 = &mut myvec;

                myvec2.push(5);
                dbg!(myvec2);
                dbg!(myvec);
            }
            ```

    * Conclusion: do not fight compiler use it as a pair.

15. Refactor starfield as a module.

16. Add a horizontal scrolling

17. Extend to sinus scrolling
    * Use standard loop with indices for rainbow loop.
    * Refactor loop to use an iterator and cycle.
    * Add a unitary test.

18. Add balls / ferris (if time)

19. Add music

20. Show a debug session with gdb. (if time)

21. Compile to wasm and run in a browser

22. Show Bastien's games
