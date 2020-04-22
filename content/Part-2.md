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


## Step 2: Implement core logic

This will be the core logic of the application. We couple two functions, `new` and `run`, with our `ToDoList<T>`; the former will return a new instance of `ToDoList`, given the user's name, and the latter will perform the required operation.

`new` is a very simply-defined function:

```rust
pub fn new(name: String) -> ToDoList<T> {
    ToDoList {
	tasks: Vec::new(),
	name: name,
    }
}
```

Much less trivial is the `run` function. Here, we print the to-do list using the [prettytable-rs](https://docs.rs/prettytable-rs/0.8.0/prettytable/index.html) crate. Below is an example of the desired output, which represents the to-do list in a tabular format:

```
hamoor's To-Do List:

+---+-------------+
| 1 | New task    |
+---+-------------+
| 2 | Second task |
+---+-------------+
| 3 | Third task  |
+---+-------------+
```

Note that we index our list starting with 1; how will this affect user input, and how must we adjust accordingly, given that our list is backed by an array-like structure?

Finally, we present the `run` function. Here, we make use of Rust's [pattern-matching syntax](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html) to dynamically destructure our `Instruction` type:

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

Finally, we implement the `Drop` trait for `ToDoList<T>`. The `Drop` trait implements a single function, `drop`, which specifies the appropriate way to destroy a `struct` (see the [documentation on the Drop trait](https://doc.rust-lang.org/std/ops/trait.Drop.html) for additional information). We apply it here so that our `ToDoList` serializes itself to a JSON file upon deallocation:

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

On completion of Step 2, we find ourselvses with a fully-functional to-do list application. The final step is to write a `main` function, which provides an entry-point to run the application with, i.e. using the shell command `cargo run -- <subcommand> <args>`.

Here, our `main` function simply checks to see if there is an existing to-do list. If so, it loads it from disk and performs the required operation on it; otherwise, it creates a new to-do list. Next, it calls the `parse()` function defined in Week 1 to parse user input and return in the appropriate format.

In either case, it uses the environment variable `$USER` to distinguish the user's name; how might we make this configurable in the future?

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

1. We currently represent each task as a string, but in reality, all we need is for it to be serializable via the `Serialize` trait provided by `serde-rs`, and to be able to display it via the `Display` trait provided in the standard library. Extend this to store additional information about tasks: Deadlines? Partners? Associated notes? The possibilities are endless!

2. Currently, we store our to-do list in a flat JSON file: What if it got too big to sustainably maintain that? What if the user wants his information stored in some more secure format? What if the user wants to sync it with other applications? Can we extend our persistent representation of our to-do list to support some of these requests?

3. How can we distribute our to-do list to potential users via the Rust ecosystem? Naturally, we would use Cargo as our build system; where would we need to make it available, so that potential users could have what they need with the simple shell command `cargo install todo-cli`?

These are some potential future considerations you might explore in your continuing journey as a Rustacean. Good luck!
