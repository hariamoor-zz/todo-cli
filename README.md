Building a Rusty To-Do List CLI
===============================

In this 2-week online programming tutorial, we will be building a to-do
list management CLI in Rust. Each weekly lesson is served as a
programming guide.

UPDATE: [Part 1 now available](content/Part-1.md)\
UPDATE: [Part 2 now available](content/Part-2.md)

To best follow the tutorials, look at the markdown write-ups above and the source code under the `src/` directory. The `cli.rs` file is part 1 and the other two files are part 2. The `Cargo.toml` file in this directory will also be mentioned and modified, but you can also take ours as-is for simplicity (we detail how it should be manipulated in our markdown).

One should be able to follow and implement the almost all the code supplied in the tutorial without it. With that said, it is intended as a reference for what idiomatic Rust code should look like.

Why Rust?
---------

[Rust](https://rust-lang.org/) is a systems-level programming langauge
that values runtime safety, performance, and ergonomics. The language
had its first stable release in March, 2020, and has since been used by
companies like Mozilla, Facebook, and Dropbox for systems-level project
where software performance and developer productivity are both
equally-pressing concerns.

Rust is most famously known for the following:

-   Memory/resource safety known at compile-time (can still get logic
    errors)
-   Expressive type system modeled after those of languages like Haskell
    and OCaml
-   Absence of bloat in the runtime and compiler optimizations that
    result in (anecdotally) very performant binaries

We use Rust for this tutorial project, together with the CLI parser
[clap-rs](https://docs.rs/clap/2.33.0/clap) and the serialization
framework [serde-rs](https:docs.rs/clap/2.33.0/clap/).

Installation and Usage
----------------------

To get set up with the Rust toolchain, see the [Getting Started
page](https://www.rust-lang.org/learn/get-started) provided on the
official website.

This tutorial assumes that the user is setup with Cargo, Rustup, and an
IDE/editor of his/her choice. Please refer to the Rust website or
contact the maintainer(s) if any more clarification is necessary. Rust
is very flexible in supporting a variety of general-purpose editors, so
if you enjoy crafting your tool to fit only your hand, look out for a Rust
plugin for your editor or IDE.

Tutorial Format
---------------

This will be a two-part online tutorial where, in each week, we
produce some usable and testable component of software.

Part 1 will focus on setting up the CLI with clap and representing the
application state with Rust\'s data structures. Finally, in Part 2, we will
serialize and deserialize our application state (tasks in our to-do
list) to disk.

We also provide _asides_, which are intended to provide information that is not necessary
for the tutorial, but is nonetheless very useful. Please see the
[general asides](content/General-Asides.md) and the 
[Rust asides](content/Rust-Asides.md). The general asides provide information on
language-agnostic design patterns, while the Rust asides discuss design
patterns appropriate specifically for Rust.

Please also see [PREREQUISITES.md](PREREQUISITES.md).

Further Resources
-----------------

Please also feel free to utilize the following resources provided by the
maintainers and the Rust community.

### Rutgers-specific Questions

Please contact the maintainers via a GitHub issue or a personal email.

### Questions about Systems Programming, Type Theory, any of the asides...

Please make a GitHub issue detailing your question(s), in case others
have the same ones.

### Questions about Rust

We provide the following (running) list of Rust resources:

-   [Maintainer David Tonlay\'s Rust FAQ](https:github.com/dtonlay/rust-faq)
-   Books:
    -   [Rust book](https://doc.rust-lang.org/book/)
    -   Rust [async book](https://www.rust-lang.org/learn/get-started)
        (for concurrent programming)
    -   The [Rust subreddit](https://www.reddit.com/r/rust/)
    -   [r4cppp tutorial](https://github.com/nrc/r4cppp) (recommended only
        with sufficient C/C++ background)
    -   [A Gentle Introduction to Rust](https:stevedonovan.github.io/rust-gentle-intro/)

Or, as always, your caring tutorial maintainers.
