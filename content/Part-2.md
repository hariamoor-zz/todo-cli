# Serializing Application State to Disk

Here, in Part 2, we'll proceed to modeling our persistent-state. Here, we develop the app's "backend", which provides an interface for the "frontend" to call upon the core logic.

## Strategies for Managing Application State

Generally, there are two kinds of memory you can use to model application state - volatile and persistent memory.

The former is usually implemented with a small disk device that interfaces with the CPU via a communication medium known as a _bus_; this is what we generally know today as RAM. This kind of memory is ideal for CPU-intensive computation with a relatively small amount of state to keep track of. However, it is _volatile_ in that it is associated with operating-system level, i.e. Unix, [processes](https://heather.cs.ucdavis.edu/~matloff/UnixAndC/Unix/Processes.pdf). A program's application state will, in general, *not* be preserved in volatile memory after its process terminates. Thus, volatile memory is insufficient for our current task; we wouldn't be able to _save_ the the to-do list for later use.

Instead, we use persistent memory, i.e. in a hard disk. While this is intrinsically slower than volatile memory in general, it can guarantee that our state will be preserved for as long as we need.

## Data Representation in Persistent Memory

Writing our to-do list to persistent memory is all well and good, but how exactly do we do that? We generally can't write arbitrarily complex Rust types to disk; only explicit sequences of bytes.

We apply a paradigm called [serialization](https://en.wikipedia.org/wiki/Serialization) to solve this problem. Serialization is the process of translating data structures and object state represented in a program into a format that can be stored in disk, transmitted across a network connection, and reconstructed later. This practice is widely applied in many different fields of software engineering; it's useful just about anywhere that multiple components of software need to interface with each other, which is to say, everywhere.

Once we know that the user is finished interacting with the to-do list, e.g. when it is scheduled by the Rust compiler to be freed from memory, we can serialize it and write to disk. Similarly, once we know that the user wants to once again interact with the to-do list, we load it from disk and _deserialize_ it back into the original format.

The Rust community provides the [serde-rs](https://serde.rs/) framework, which provides an interface to generically serialize and deserialize Rust data structures. We will extensively use this framework to provide the desired API to our frontend.

For more information on serialization and `serde-rs`, please see the [pertinent aside](Rust-Asides.md).

## Setting Up

Fortunately, Cargo takes care of most of this for us. We simply need to specify in our `Cargo.toml` that the crates `serde-rs` and `serde_json` (we'll be using [the ubiquitous JSON format](https://en.wikipedia.org/wiki/JSON)) needs to be available at build-time; Cargo takes care of the rest on our behalf.

The configuration used by the maintainers is [supplied in the source repository](https://github.com/hariamoor/todo-cli/blob/c05ab448365495f25a5f9b1eede81622a8d2d2a0/Cargo.toml#L9-L13).

## Step 1: Define Appropriate Data Types

Last week, we defined our `Instruction` type as follows:

```rust
pub enum Instruction {
    Add(String),
    Remove(usize),
    Modify(usize, String),
    Print,
}
```

<!-- However, we must first ask ourselves; when and why did we assume that each task would be stored as a string? It would indeed be convenient if that's all we needed to store, but what if we wanted to also store a due date at some point in the future? What if we wanted to expand our definition of what a task in our to-do list looks like? -->

We first ask ourselves: why does our identifier _have to_ be a string? Why not a `struct`? Why not an arbitrary type, as long as you can serialize it and print it to the console?

In software, rapidly changing requirements are all too common. In general, we'd like to make code as _generic_ and _extensible_ as possible. Fortunately, Rust is built to support this use case idiomatically. We add a _parametriized generic type_ as follows:

```rust
pub enum Instruction<T> 
    Add(T),
    Remove(usize),
    Modify(usize, T),
    Print,
}
```

Now, our _identifier_ type can effectively be whatever we want! We also add a macro using the [_derive_ attribute](https://doc.rust-lang.org/stable/rust-by-example/trait/derive.html) to tell the compiler to auto-generate code that:

1. Allows this type to interface with `serde-rs`
2. Output debug information where necessary

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

Since we'll be serializaing and displaying (printing to memory) our generic type `T` later, it must implement the `Serialize` and `Display `traits. Otherwise, the compiler won't know what we want when we ask it to do these things.

## Step 2: Implement core logic

This will be the core logic of the application. We couple two functions, `new` and `run`, with our `ToDoList<T>`. The former will return a new instance of `ToDoList` and the latter will perform the required operation.

`new` is a very simply-defined function:

```rust
pub fn new(name: String) -> ToDoList<T> {
    ToDoList {
	tasks: Vec::new(),
	name: name,
    }
}
```

Much less trivial is the `run` function. Here, we print the to-do list using the [prettytable-rs](https://docs.rs/prettytable-rs/0.8.0/prettytable/index.html) crate. Below is an example of the desired output.

```
hamoor's To-Do List:

+---+-------------+
| 1 | First task  |
+---+-------------+
| 2 | Second task |
+---+-------------+
| 3 | Third task  |
+---+-------------+
```

Finally, we present the `run` function. We make use of Rust's [pattern-matching syntax](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html) to dynamically destructure our `Instruction` type:

```rust
pub fn run(&mut self, inst: Instruction<T>) {
    match inst {
	Instruction::Add(task) => self.tasks.push(task),
	Instruction::Modify(i, t) => self.tasks[i - 1] = t,
	Instruction::Remove(i) => {
	    self.tasks.remove(i - 1);
	}
	Instruction::Print => {
	    if !self.tasks.is_empty() {
		let mut table = Table::new();
		
		for (i, s) in self.tasks.iter().enumerate() {
		    table.add_row(row![(i + 1).to_string(), s]);
		}
		
		println!("\n\n{}'s To-Do List:\n", self.name);
		table.printstd();
	    } else {
		println!("No tasks to print for {}", self.name);
	    }
	}
    }
}
```

Note that we expect user input of indices to be in terms of the presented format, i.e. with indexing starting at 1; this is why we subtract 1 from the same before applying it to the underlying vector type `tasks`.

Finally, we implement the `Drop` trait for `ToDoList<T>`. The `Drop` trait implements a single function, `drop`, which specifies the appropriate way to destroy a `struct` (see the [pertinent documentation ](https://doc.rust-lang.org/std/ops/trait.Drop.html)). We apply it here so that our `ToDoList` serializes itself to a JSON file upon deallocation:

```rust
fn drop(&mut self) {
    let userdata = json::to_string_pretty(self).unwrap();
    fs::write(BACKUP_FILE, userdata).unwrap();
}
```

With these three functions, we implement an _API_ of sorts for frontends for our core logic. Theoretically, a user could use the `new` and `run` functions to configure input/output to the application arbitrarily.

BONUS: Use the [linefeed crate](https://docs.rs/linefeed/0.6.0/linefeed/) to implement a REPL that allows for multiple modifications on the fly.\
ANOTHER BONUS: In [our source code](../src/api.rs), we provide an automated test for our core API; this can be run with `cargo test`. Apply the [best practices for automated testing](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) to test the other parts of the application.

## Step 3: Providing a User-Interface

We now have a fully-functional to-do list app! The final step is to write a `main` function, which provides an entry-point to our application. We can call it with the shell command `cargo run -- <subcommand> <args>`.

Our `main` function first checks to see if there is an existing to-do list. If so, it loads it from disk and performs the required operation on it; otherwise, it creates a new to-do list.

In either case, it uses the environment variable `$USER` to distinguish the user's name: how might we make this configurable in the future?

```rust
fn main() -> Result<(), Box<dyn Error>> {
    let mut list: ToDoList<String> = match File::open(BACKUP_FILE) {
        Ok(file) => {
            // file exists - deserialize and go with existing list
            let file = BufReader::new(file);
            serde_json::from_reader(file)?
        }
        Err(_) => {
            // file does not exist - make a new list
            ToDoList::new(env!("USER").to_string())
        }
    };

    match parse() {
        Some(inst) => list.run(inst),
        None => panic!("Arguments could not be parsed"),
    }
    
    Ok(())
}
```

## Step 4: Possibilities for Extension

Congrats! We've now built a functional to-do list CLI application in Rust!

How can we make this even more awesome than it is now? Some ideas for future development:

1. We currently represent our identifiers as strings, but all we need is a generic type that implements the `Serialize` and `Display` traits. Extend the identifier type to store additional information about tasks: Deadlines? Partners? Associated notes? The possibilities are endless!

2. Currently, we store our to-do list in a flat JSON file: What if it got too big for that? What if the user wants his information stored in a different format? What if he wants to sync it with other applications? Can we extend this persistent representation of our state to support some of these requests?

3. How can we distribute our to-do list to potential users via the Rust ecosystem? Naturally, the end user would install it via Cargo; where would we need to make it available for them to do this?

These are some potential future considerations you might explore in your continuing journey as a Rustacean. Good luck!
