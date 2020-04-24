# Building a CLI Parser

This week, we'll get started on our command-line interface. This is the app's "frontend", or, more precisely, the user-level component - a user of todo-cli would use terminal to interact with the application.

## Why not a web app?

A design pattern for such an app would involve a client-side, or "frontend", designed to run in the browser, usually in JavaScript. This frontend would interact with a server-side component, a "backend", such that the backend queries, indexes, and modifies the application state in response to web requests (generally through HTTP(s)) from the frontend.

This project pursues a CLI instead for a simple reason: web servers are difficult to start writing in a programming language, particularly a new one. Furthermore, the user experience is not too degraded (especially if the user enjoys the commandline).

## Requirements

As stated, we assume familiarity with [Cargo](https://doc.rust-lang.org/cargo/), Rust's official package manager, and [Rustup](https://rustup.rs/), its official toolchain installer.

Any assistance in this matter can be directed to the maintainer(s). The Rust team provides [an excellent documentation system](https://doc.rust-lang.org/beta/), in any case, so this is hopefully not a problem.

### Installing Dependencies

Cargo requires a file called `Cargo.toml` in the root directory of our project; this should be our single source of truth for the packages that our program depends on. The `Cargo.toml` file provided in the root of this repository suffices as an example -- feel free to tweak it for your other projects.

We also recommend the [cargo-edit](https://docs.rs/cargo-edit/0.6.0/cargo_edit/) tool to easily add, remove and upgrade dependencies by modifying `Cargo.toml`. Note, however, that this is only useful for general Rust development; it should not be needed for this tutorial.

## Step 1: Design

The clap-rs crate provides numerous ways to specify the procedure for a CLI; at time of writing, our options include native Rust objects, macros supplied with the package itself, and a separate YAML file.

todo-cli must support, at minimum, the CRUD (Create Read Update Delete) API. Thus, the core logic we design next week should be based on some kind of data type that can tell us which instruction to process. Fortunately, we can do this rather simply with an [enum](https://doc.rust-lang.org/stable/rust-by-example/custom_types/enum.html) type. If you want to know more about where these come from see our [aside on sum types](General-Asides.md).

```rust
pub enum Instruction {
    Add,
    Remove,
    Modify,
    Print
}
```

But wait! Is this data type expressive enough to contain all the data we'll need? If, say, we wanted to add a new task, would we find just an instance of `Instruction::Add` sufficient for it? Or might we also need some other pieces of data, say an _identifier_ for the task within our persistent store to operate on?

Good catch! If we're adding a new task, we'd need a `String` to give us a task description. If we're removing a task, we'd need an identifier for it within our data store that'd help us do that. Fortunately, there's another feature of Rust's `enum`s that can bail us out:

```rust
pub enum Instruction {
    Add(String),
    Remove(usize),
    Modify(usize, String),
    Print,
}
```

The `usize` type is a numeric value that can be used to index array-based data structures in Rust. We can assume for sake of simplicity that we'll be using an array as the data store for our task records, so this is sufficient for now.

Now, we need a function that can read command-line parameters and return the appropriate instruction. We design `parse` with the following function header:

```rust
fn parse() -> Instruction {
    // code
}
```

But wait! What if the user inputs data that doesn't make sense? We won't know what to return! How can we *guarantee* the Rust compiler that we'll return `Instruction` in all cases? That's essentially what we're telling the compiler we'll do in that function header, right?

In other words, we ask ourselves: what about error-conditions? How can we emulate good practices for error handling in Rust?

By using the [Option](https://doc.rust-lang.org/beta/std/option/enum.Option.html) type, of course! The Rust standard library provides data types `Result<T, E>` and `Option<T>` (bonus: What do the `T` and `E` signify within each type declaration?) Both, like our `Instruction` type above, are enums.

We use `Option` to design our function `parse`. We now replace our previous code:

```rust
fn parse() -> Option<Instruction> {
    // code
}
```

Now, we handle errors safely by returning `Some(Instruction)` if the function succeeds and `None` otherwise. You can read more about error handling in rust in a [dedicated aside](Rust-Asides.md).

A finalized CLI parser for todo-cli, written using the `clap::clap_app` macro provided in clap-rs, is supplied [here](https://github.com/hariamoor/todo-cli/blob/b574ad84b5bae1a4c9ebce3780972884339e7cb0/src/main.rs#L27-L46). To read more on macros, see [this aside](Rust-Asides).

## Step 2: Implementation

Now that we know (roughly) what we're supposed to write, how do we get started?

The first step is to represent the command-line arguments with some easily queryable data structure. Fortunately, the clap-rs API provides exactly this; command-line arguments are read through the `clap::App` data type and served to the user via `clap::ArgMatches`.

First, we set up a parser that recognizes just the `add` operation.

```rust
let matches = clap::App("todo-cli")
    .subcommand(
        SubCommand::with_name("add")
            .arg("NEW")
            .required(true)
            .takes_value(true)
            .help("Add to task")
    )
    .get_matches();
```

Now, we have an instance of `clap::ArgMatches` to help us. However, it has yet to support `rm`, `modify`, and `print`.

We can start by adding a second subcommand for `rm` as follows:

```rust
let matches = clap::App("todo-cli")
    .subcommand(
        SubCommand::with_name("add")
            .arg("NEW")
            .required(true)
            .takes_value(true)
            .help("Add to task")
    )
    .subcommand(
        SubCommand::with_name("rm")
            .arg("NUM")
            .required(true)
            .takes_value(true)
            .help("Remove task")
    )
    .get_matches();
```

The `modify` and `print` operations are left as an exercise to the reader. Note that the pattern above is a universal design pattern -- here's a [brief aside](General-Asides.md).

Finally, we address the logic required to query `matches` and return our `Instruction`. Fortunately, this is the most intuitive part: we need only a branch (an if-statement) for each subcommand (case of `Instruction`).

```rust
fn parse() -> Result<Instruction, Box<dyn Error>> {
    let matches = clap::App("todo-cli")
    .subcommand(
        SubCommand::with_name("add")
            .arg("NEW")
            .required(true)
            .takes_value(true)
            .help("Add to task")
    )
    .subcommand(
        SubCommand::with_name("rm")
            .arg("NUM")
            .required(true)
            .takes_value(true)
            .help("Remove task")
    )
    .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
		return Ok(Instruction::Add(
            matches
                .value_of("NEW")
                .expect("Need task to add")
                .to_string(),
        ));
    } else if let Some(matches) = matches.subcommand_matches("rm") {
		return Ok(Instruction::Remove(
            matches.value_of("NEW").expect("Need task to add").parse()?,
        ));
    }
    
    return None;
}
```

The above code is a complete parser for the `add` and `rm` options; extending it to support `modify` and `print` is left as an exercise to the reader. If the uses of `let`, especially an `if let` are new to you, [here's an aside](Rust-Asides.md).

## Step 3: Testing

Generally, it is a good practice to supply unit-tests using Rust's [automated testing support](https://doc.rust-lang.org/book/ch11-00-testing.html). However, `parse` should not need this because most of its behavior is produced directly from the clap-rs documentation. We will be writing automated tests in Week 2.

If you'd like, `cargo build` your app to make sure you didn't make any mistakes. `cargo run` with some flags to see if you can parse instructions properly and see what `clap` has made for you.

## Reflections

In this week's tutorial, we learned about Rust's design principles and type system. We know now that Rust's core values include safety, performance, and ergonomics above all else.

Why did the open-source community decide that this was a necessity when they built Rust? What makes it a competitive language relative to its predecessors?

For that matter, who _are_ Rust's predecessors? Some would consider the target runtime and argue that Rust was designed as a safer version of C or C++, while others would consider the type system and argue that it's supposed to be a faster and more boardly usable Haskell. A third subset might even see Rust as a successor to Python or JavaScript. Are any of them _objectively_ right or wrong? Or will Rust continue to defy our attempts to classify it as the successor to any one particular language?

Also, will this go a week without name-dropping category theory? How deeply nested will the asides be?
