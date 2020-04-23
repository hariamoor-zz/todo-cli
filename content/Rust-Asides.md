# Rust Asides

These asides supplement the tutorial with bits on the Rust syntax covered in the code.

## Aside: Macros in Rust

Firstly, what is a macro? A macro is a recording of specific instructions. You can think of it as a function from code to code. This is a bit dangerous if the code produced can affect the code around it. If you invoke a macro, it might introduce a variable, or change a variable that was being used by the code around it.

Rust is blessed with [an extremely powerful macro system](https://doc.rust-lang.org/1.7.0/book/macros.html). It's not only Turing-complete (theoretically just as powerful as Rust itself -- with a power tool called procedural macros), but also _hygienic_; the compiler can make the guarantee that there is no accidental capture of identifiers in macro expansion. Also: *important convention*: all things ending with a `!` in Rust are a macro.

This is a common reason why metaprogramming in C++ is under-utilized - it's not hygienic, and is in fact rather unsafe. The Rust community, however, prides itself both on high-quality metaprogramming primitives and learning from the flaws in other languages. For this reason, we can safely use macros to our hearts' content (at least until the build times get overwhelming).

It is worth noting that macros are not a necessary part of code. Like all tools, there is a time and place for it. `clap::clap_app` uses it to make its use of the builder pattern cleaner. More broadly, when using a mini-language to express a specific part of a program (like the command-line arguments or string formatting (`println!`)).

Macros can also help inform the future of the language. The `?` we used above actually used to be `try!` until the committee saw it everywhere and realized that a nicer syntax was in order.

## Aside: Good Error Handling

Suppose you were writing this in perfect, code reviewed Java.

```java
public static Instruction parse() throws ThatCustomExceptionForBadUserInputIJustMade {}
```

Now, there's a few interesting issues. In particular a caller would have to write:

```java
try {
    Instruction args = parse();
} catch (ThatCustomExceptionForBadUserInputIJustMade tcefbuiijm) {
    // complain to user or something
}
```

Technically, this is a new control flow operation. `args` lives in its own scope. Finding the right bit of code that catches the exception is hard. These complications are not *huge* slowdowns, but they complicate the mental model of functions. Functions can two one of two things: return a value, or barf loudly. In Rust, this is simpler, but the only escape hatch is `panic!`ing which is generally avoided, and functions always return something.

As mentioned above, the standard library has two error-reporting types: `Option` if you don't care to tell callers what failed and `Result` if you do.

### Aside in the Aside: Similarity to the Ubiquitous `null` type

Rustaceans with backgrounds in C-based languages can be quick to retort that `Option` is a glorified null-pointer. However, this is not true. Historically, `null` in OO languages has been implemented as a invariant pointer to an immutable memory address, e.g. `const void *NULL = 0` in C. It's traditionally used as a "default" type or a placeholder, which is intrinsically an unsafe notion, since null-checks aren't rigorously enforced by the compiler.

(In fact, if the Java above wasn't be thoroughly code reviewed you may not even throw an exception and just return null if the user messes up, and you'll have to check for that, and others (me in three months if I were writing it) would forget to.)

The `None` enumeration in Rust's `Option` type, on the other hand, is a very different notion. The idea is that, in these exceptional situations, you can either explicitly address the error case or, if the current function returns a type of the same enumeration, use the `?` operator to [propagate the errors up the call-stack](https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html).

#### Aside in the Aside in the Aside: Zero Cost Abstractions

Yay! We have a type. The issue, however, with types, is that they can be slower to manage after the code is compiled. After all, in C, bare pointers are tiny -- fitting in registers. But what about some `Option` thing? What's its size?

Here, the Rust compiler has a trick up its sleave: if your type is like a pointer, rust will compile the matches against an `Option` to the nullity check that the good C programmer would've written. (So the Rustecean from the aside above is right in some cases, and in a literal sense.)

This is a general principle in Rust: a lot of safer and elegant parts of the code actually come at no runtime cost. There is one other classic example Rusteceans use to gloat: the memory safety comes at no cost -- to the programmer or the compiled program.

## Aside: Bindings

The way to make variables in rust is to use `let`. However, it does more than that. This has to do with a broader notion called [pattern matching](https://en.wikipedia.org/wiki/Pattern_matching).

Rust uses pattern matching in various cases -- here is the [chapter in the Rust book](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html) that describes this. `let` bindings can use a special type of pattern matching called [destructuring](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-to-break-apart-values).

Some destructuring will always work: a `struct` (or object) can be destructured into its fields, a tuple (fixed length list if you need an analogy) can be destructured into its contents. Rust calls this an irrefutable binding.

For `enums`, these bindings are not irrefutable: the value being destructured might be under a different label. For example, if you have an `a: Option<Instruction>`, `let Some(value) = a` would not always work.

The `if let` lets you conditionally bind variables to further aid managing `enum`s. This means, instead of only having `?` to boot errors further up the stack, you have `if let` to handle the error.

There are other ways to pattern match against more complex enums too: you use [match](https://doc.rust-lang.org/book/ch18-01-all-the-places-for-patterns.html#match-arms).

(For the category-theory inclined: this has to do with the universal mapping properties of products and coproducts. The irrefutable case is essentially stating that if you have a product over types `T` and `U` (say a tuple `(T, U)`), every map from a type `V` into `T` and `U` factors through the product type. The refutable case has to do with coproducts. The duality means that we cannot be sure we have such a similar factoring.)

## Aside: Serialization

The `serde-rs` library, used in this tutorial, is considered by the open-source community to be the gold standard for safe and predictable serialization. It provides a common API to interact with a multitude of formats, including JSON, Bincode, Pickle (in the Python world), YAML, and more! Both from a usability perspective and a safety perspective, `serde-rs` provides a much better framework for serialization than others of its kind. Below, we provide a cursory comparison between `serde-rs` and similar serialization frameworks in other languages:

### Java's Default Serialization

With Java, objects could be created arbitrarily with serialization and deserialization; this is intrinsically unsafe. In particular, you could inject a crafted serialized object into the program's input, and create any object - potentially one that could hijack the program - therein. Generally speaking, the Java community acknowledges that it was a bad idea to publicly use such a _flexible_ method of serialization, and users of this framework in industry generally tend to have to wrap it with their own paradigms.

With Rust and `serde-rs`, however, you have a much more strict and well-defined deserialization process. If you want a `struct` to deserialize from a JSON document, but only take in some of its fields, then that's all you're going to get. Similarly, arbitrary objects cannot be created the same way they can in Java; in fact, there's very little, if any, opportunity for external programs to hijack control of your Rust program through `serde-rs`.

### C++'s Serialization with Boost

This Rust API for serialization is generally considered more _expressive_ than that provided by the Boost framework in C++; this is because, while you can serialize/deserialize objects in C++ as you can in Rust, the contracts you must fulfill for custom formatting is less clear (since C++ templates are not as expressive as Rust's traits). Furthermore, the C++ libraries for serialization are not as all-encompassing as serde: there is no one library (known to the authors) that serializes into all the formats that serde does.

### Haskell's `Data.Serialize`

Who the fuck goes to production with Haskell, in the first place...?

Well, that's not really an argument, if Haskell were doing the right thing. Fortunately, we have avoided having to recant: `Data.Serialize` is a lot like the Boost system that exists for C++. The key difference is that the serialization contract is explicit, but it is messier as it lifts the `Builder` monad to form a custom serialization monad.

`Data.Serialize` is also not as complete as serde, lacking any in-built support for the variety of formats serde does.

## Aside: The Layout of a Rust Application

If you're reading this you either somehow like the schizoid writing style of this tutorial or you ignored our mild suggestion to read the Rust book. In case that suggestion was too mild, we recommend you read through [the Rust book](https://doc.rust-lang.org/book/) again.

Not yet suggestive enough? Luckily for you, the writer with this tone likes the sound of their own voice and will get you up to speed on some basics imminently. Again, this isn't a replacement for [the book](https://doc.rust-lang.org/book/), just, at best, a suppliment that focusses on points that will come up in starting on the app.

That is, what to start with? Let's really empty the canvas and suppose that the code wasn't actually already present under `../src`. The easiest thing to do is `cargo new app <name>` in a shell of your choice. This would make this sort of directory tree:

- <name>
  - src
    - main.rs
  - Cargo.toml
  - .git (yes, by default, cargo makes you a git repo too -- what a great friend!)

The `Cargo.toml` describes the app, naming it, versioning it, and listing its dependencies. Hence, if you need a dependency, it's easiest to add it to this list so that you have it when you rebuild. This is covered in more detail as we install clap-rs and serde. Hence, let's move on to the other file: `main.rs`.

Here, cargo will have given you a main function, but let's talk in broader strokes for this aside.

### Aside in the Aside: The Layout of Rust Code

This is a broad overview of what goes where in Rust. This, like the aside it's in, is not a replacement for [the book](https://doc.rust-lang.org/book/), but a sub-section to aid you in reading the source code we provide.

As a descendant of C, Rust uses `{}` for blocks. There are four levels of blocks we'll nest our way into (this is in order of the tutorial's presentation):

1. Module
1. Struct, Enum, or trait declaration
1. Function
1. Impl block

Here's a fuller arrangement of what the blocks are and how they nest (simplified to only include common patterns):

- Module
  - Struct, trait, or enum declaration
  - Impl block
    - Function
  - Function

Inside a function, there's control-flow. This is discussed in more detail as needed, so will be glossed over for our broad strokes. In fact, the broader strokes below only briefly sketch modules and impl blocks (the rest are assumed to be legible or covered as they are brought up).

### Aside in the Aside: Modules

Zerothly: the book has more details [here](https://doc.rust-lang.org/1.30.0/book/2018-edition/ch07-00-modules.html).

Firstly, a bit of philosophy:

> Namespaces are a honking great idea -- let's do more of those! --_Tim Peters, the Zen of Python_

So? The first thing is that every `filename.rs` is implicitly in a module `filename`. But there's more! Rust has a module keyword so you can nest as much as you'd like with just one file. `pub` makes things public because everything is private by default (yay, encapsulation!). Hence, you'll see that our `cli.rs` opens with `pub mod cli {`.

In order to use modules elsewhere, you need to `use` them. This also has a couple edge cases:

- If the module is another file in your application, you have to declare it with `mod filename;` in your `main.rs`.
- If the module is from a crate you're depending on, you have to forward-declare it with `extern crate toplevel_name_of_crate;`.

To use your own modules from your application in your application, note that you have to `use crate::<module path>`. Inner modules are referred to after the `::`. All these edge cases appear in our `main.rs`.

### Aside in the Aside: impl blocks

These blocks implement functions on data types. These are the methods in Java. There are two flavors of impl block:

- Standalone
- impl trait

A standalone `impl` block reads `impl <type> {` and in the block, you list out methods and implement them. These will be the methods your object can use. `self` is a magic keyword in the block that refers to the instance of the object you're calling the method on (if it's absent, the method is like a static function in Java or C++). `Self` is a magic keyword for the type that you're implementing methods for.