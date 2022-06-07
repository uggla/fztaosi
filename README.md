# From Zero To A Old School Intro

This is a live demo/lab to discover Rust smoothly by creating an
[intro](https://en.wikipedia.org/wiki/Crack_intro).

Options have been passed to Cargo.toml to reduce the size of the binary.
As well all assets are embedded into the binary to deploy it easilly.
A rpm can be built for the project using `cargo rpm build`.

## Authors

- [@Uggla](https://www.github.com/Uggla)

## Run Locally (mainly for development purposes)

1. Clone the project

```bash
  git clone https://github.com/uggla/fztaosi
```

2. Go to the project directory

```bash
  cd fztaosi
```

### Native
1. Install Rust following the instructions [here](https://www.rust-lang.org/fr/learn/get-started).

   *Tips: the rustup method is the simplest one.*

2. Install required library for macroquad

* Ubuntu system dependencies
```bash
apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev
```

* Fedora system dependencies
```bash
dnf install libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel
```

* Windows and MacOS system
```
No dependencies are required for Windows or MacOS
```

3. Run
```bash
cargo run --release
```

#### Wasm32 client

1. Follow the above instruction of the native build.

2. Install basic-http-server
```bash
cargo install basic-http-server
```

3. Add the wasm32 compilation target
```bash
rustup target add wasm32-unknown-unknown
```

4. Run
```bash
cargo build --target wasm32-unknown-unknown --release
```

5. Serve the files and open the browser
```bash
basic-http-server
xdg-open http://127.0.0.1:4000
```

## Lab steps:

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
    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn fake_test(){
            assert_eq!(1,1);
        }
    ```

18. Add balls / ferris (if time)

19. Add music
    * Show diff beetween cargo build and cargo build --release

20. Show a debug session with gdb. (if time)

21. Compile to wasm and run in a browser

22. Show Bastien's games
