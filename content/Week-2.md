<!---
Needed asides:
- 3-tier architecture and applications outside of webdev
- Problems with serialization in other languages - how Serde aims to solve them in Rust
-->

# Serializing Application State to Disk

In this, our second, week, we'll proceed to modeling our persistent-state. Here, we develop the app's "backend", which provides an API of some sort for the "frontend" to interact with to serve the users.

## Strategies for Managing Application State

Computers generally provide two kinds of memory for users to generically manage their application state - volatile and persistent memory.

The former is usually implemented with a small disk device that interfaces with the CPU via a communication medium known as a _bus_; this is what we generally know today as RAM. This kind of memory is generally ideal for CPU-intensive computation with a relatively small amount of state to keep track of. However, it is _volatile_ in that it is associated with operating-system level, i.e. Unix, [processes](https://heather.cs.ucdavis.edu/~matloff/UnixAndC/Unix/Processes.pdf); the operating system cannot guarantee that a program's application state will be preserved in volatile memory after its process terminates. Thus, volatile memory is insufficient for our task of having our app _save_ the the to-do list for later use.

Instead, we use persistent memory, i.e. in a hard disk, to store our to-do list. While this is intrinsically slower than volatile memory in general, it can guarantee that our content will be preserved for as long as we need.

## Data Representation in Persistent Memory

Writing our to-do list to persistent memory is all well and good, but how exactly do we do that? We generally cannot write arbitrarily-defined program variables to disk - we can only store sequences of characters, i.e. bytes.

In order to effectively represent our memory in a way that it can be stored on disk, we apply a paradigm called [serialization](https://en.wikipedia.org/wiki/Serialization). Serialization is the process of translating data structures and object state represented in a program into a format that can be stored in disk, transmitted across a network connection, and reconstructed later. This practice is widely applied in many different fields of software engineering; it's useful just about anywhere that multiple components of software need to interface with each other, which is to say, everywhere.

At a high level, once we know that the user is finished interacting with the to-do list, e.g. when it is scheduled by the Rust compiler to be freed from memory, we can serialize it and write to disk. Similarly, once we know that the user wants to once again interact with the to-do list, we load it from disk and _deserialize_ it back into the original format.

The Rust community provides the [serde-rs](https://serde.rs/) framework, which exposes an API to generically serialize and deserialize Rust data structures. In this part of the tutorial, we make extensive use of this framework to provide a simple API to our frontend that manages a user's to-do list and then saves it to disk when s/he doesn't need it anymore.

## Setting Up

Fortunately, Cargo takes care of most of this for us; we simply need to specify in the configuration file `Cargo.toml` that the crates `serde-rs` and `serde_json` (we'll be storing our list in [the ubiquitous JSON format](https://en.wikipedia.org/wiki/JSON)) needs to be available at build-time; it takes care of the actual downloading on our behalf.

The configuration used by the maintainer(s) is [supplied in the source repository](https://github.com/hariamoor/todo-cli/blob/c05ab448365495f25a5f9b1eede81622a8d2d2a0/Cargo.toml#L9-L13).

## Step 1: Define Appropriate Data Types

<!---
I made `Instruction` and `ToDoList` monadic since last time in favor of a possible bonus project
-->

Last week, we defined our `Instruction` type as follows:

```rust
pub enum Instruction {
    Add(String),
    Remove(usize),
    Modify(usize, String),
    Print,
}
```

However, we must first ask ourselves; when and why did we assume that each task would be stored as a string? It would indeed be convenient if that's all we needed to store, but what if we wanted to also store a due date at some point in the future? What if we wanted to expand our definition of what a task in our to-do list looks like?

In software, rapidly changing requirements like these are all too common. Thus, we would like to make code as _generic_ and _extensible_ as possible. Fortunately, Rust is built to support this exact use case easily and idiomatically. We add a _parametriized generic type_ as follows:

```rust
pub enum Instruction<T> 
    Add(T),
    Remove(usize),
    Modify(usize, T),
    Print,
}
```

This allows us to relax our backend-level assumption that the frontend will always represent tasks with the `String` type! We also add a macro using the [_derive_ attribute](https://doc.rust-lang.org/stable/rust-by-example/trait/derive.html) to tell the Rust compiler to auto-generate code that allows this type to interface with `serde-rs` and output debug information where necessary as follows:

```rust
#[derive(Serialize, Deserialize, Debug)]
pub enum Instruction<T> {
    Add(T),
    Remove(usize),
    Modify(usize, T),
    Print,
}
```

Next, we define a `ToDoList` type, which help us represent the list itself. It will have two fields; a list of tasks, represented as a `Vec` (a heap-allocated array), and the `String`-encoded name of the user.

```rust
#[derive(Debug)]
pub struct ToDoList<T>
where
    T: Display + Serialize,
{
    pub tasks: Vec<T>,
    name: String,
}
```

Of course, this has to be similarly compatible with `serde-rs`. Furthermore, since we'll be serializaing and displaying (printing to memory) our generic type `T` later, we must ensure that it implements the `Serialize` and `Display `traits; otherwise, the Rust compiler won't know what we want when we ask it to do these things.
