# Rust Asides

These asides supplement the tutorial with bits on the Rust syntax covered in the code.

## Aside: Macros in Rust

Firstly, what is a macro? A macro is a recording of specific instructions. You can think of it as a function from code to code. This is a bit dangerous if the code produced can affect the code around it. If you invoke a macro, it might introduce a variable, or change a variable that was being used by the code around it.

Rust is blessed with [an extremely powerful macro system](https://doc.rust-lang.org/1.7.0/book/macros.html). It's not only Turing-complete (theoretically just as powerful as Rust itself -- with a power tool called procedural macros), but also _hygienic_; the compiler can make the guarantee that there is no accidental capture of identifiers in macro expansion. Also: *important convention*: all things ending with a `!` in Rust are a macro.

This is a common reason why metaprogramming in C++ is under-utilized - it's not hygienic, and is in fact rather unsafe. The Rust community, however, prides itself both on high-quality metaprogramming primitives and learning from the flaws in other languages. For this reason, we can safely use macros to our hearts' content (at least until the build times get overwhelming).

It is worth noting that macros are not a necessary part of code. Like all tools, there is a time and place for it. `clap::clap_app` uses it to make its use of the builder pattern cleaner. More broadly, when using a mini-language to express a specific part of a program (like the command-line arguments or string formatting (`println!`)).

Macros can also help inform the future of the language. The `?` we used above actually used to be `try!` until the committee saw it everywhere and realized that a nicer syntax was in order.

## Aside: Good Error Handling

Consider the following pristine Java code:

```java
public static Instruction parse() throws ThatCustomExceptionForBadUserInputIJustMade {}
```

Though this is considered to be idiomatic Java, there are some rather unsavory details we see here. In particular, the onus is on the developer to remember which functions can result in thrown exceptions, and which of those exceptions to catch, i.e. with a `try-catch` block such as the following:

```java
try {
    Instruction args = parse();
} catch (ThatCustomExceptionForBadUserInputIJustMade tcefbuiijm) {
    // complain to user or something
}
```

In general, the idea of "throwing" an "exception" in the middle of the code when something doesn't go the way you want it to, and unwinding the call-stack until someone catches it. This has its roots in C-style error-handling with POSIX signals (did someone say "Segmentation fault (core dumped)"?). This is a more complex model as functions suddenly have two types of exits.

Rust makes it simpler through the option and result types: you merely return the errors and the compiler checks that you check for the errors since you'd otherwise have a type mistmach (or you'd explicitly `unwrap` the error to ignore it).

### Aside in the Aside: Monads as a Generalization of the Above

A much more succinct model of error-handling is Haskell's [monad](https://www.haskell.org/tutorial/monads.html) construct. Observe the following definition:

```haskell
infixl 1  >>, >>=
class Monad m  where
    (>>=)            :: m a -> (a -> m b) -> m b
    (>>)             :: m a -> m b -> m b
    return           :: a -> m a
    fail             :: String -> m a

    m >> k           =  m >>= \_ -> k
```

For the uninitiated, the above is absolute gibberish; however, in essence, Haskell provides a generic type called `Monad` defined by (1) a generic type `a`, and; (2) functions called `bind` and `return`. The `bind` function (written `>>=`, said to form the Haskell logo) applies another monadic transformation respecting the same monad `m` iff `a` is valid, and the `return` function "unwraps" `a` so that it can be used as a normal value.

In general, this is a much more succinct pattern for error-handling. In cases where you want to bother the user (which, for small applications such as this one, is most of them), it DWIMs (**D**oes **W**hat **I** **M**ean (ok, ostensibly what _you_ mean)) and propagates the error, defined here as a `String`, up the call stack. On the other hand, if the developer wants to explicitly handle the error, he may do so with Haskell's pattern-matching syntax.

How? Haskell's `Result` type (more broadly named `Either` has a `Monad` instance).

Idiomatic Rust follows a very similar system; the `Result` and `Option` types are isomorphic to trivially-defined monads in Haskell (said isomorphism is seen in the `and_then` function in Rust and the `Ok` and `Some` constructors, more on why this isomorphism is up to various finicky details is provided in our lengthy discussion on the underlying mathematics [here](some-theory.md#rust-and-hkts "There's also some bitching about the notion of Monads if you didn't like this section.")). In general, it is considered unsavory to `panic!`, especially explicitly. You either propagate the error up the stack or manually address it; however, if you don't want to do either of those things, there is also an `.unwrap()` function that returns the underlying type or `panic!`s internally.

### Aside in the Aside: Similarity to the Ubiquitous `null` type

Rustaceans with backgrounds in C-based languages can be quick to retort that `Option` is a glorified null-pointer. However, this is not true. Historically, `null` in OO languages has been implemented as a invariant pointer to an immutable memory address, e.g. `const void *NULL = 0` in C. It's traditionally used as a "default" type or a placeholder, which is intrinsically an unsafe notion, since null-checks aren't rigorously enforced by the compiler.

(In fact, if the Java above wasn't be thoroughly code reviewed you may not even throw an exception and just return null if the user messes up, and you'll have to check for that, and others (me in three months if I were writing it) would forget to.)

The `None` enumeration in Rust's `Option` type, on the other hand, is a very different notion. The idea is that, in these exceptional situations, you can either explicitly address the error case or, if the current function returns a type of the same enumeration, use the `?` operator to [propagate the errors up the call-stack](https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html).

#### Aside in the Aside in the Aside: Zero Cost Abstractions

Yay! We have a type. The issue, however, with types, is that they can be slower to manage after the code is compiled. After all, in C, bare pointers are tiny -- fitting in registers. But what about some `Option` thing? What's its size?

Here, the Rust compiler has a trick up its sleeve; if the underlying type implements the `Deref` trait, i.e. it's "like" a pointer, then this idiomatic Rust code compiles to what the equivalent C code that applies a null-check would.

This is a rather useful pattern in Rust: a lot of safer and elegant parts of the code actually come at no runtime cost.

## Aside: Bindings

The way to make variables in rust is to use `let`. However, it does more than that. This has to do with a broader notion called [pattern matching](https://en.wikipedia.org/wiki/Pattern_matching).

Rust uses pattern matching in various cases -- see the associated [chapter in the Rust book](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html) that describes this. `let` bindings can use a special type of pattern matching called [destructuring](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-to-break-apart-values).

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

### Haskell's Serialization with `Data.Serialize`

Who the fuck goes to production with Haskell, in the first place...?

Well, that's not really an argument, if Haskell were doing the right thing. Fortunately, we have avoided having to recant: `Data.Serialize` is a lot like the Boost system that exists for C++. The key difference is that the serialization contract is explicit, but it is messier as it lifts the `Builder` monad to form a custom serialization monad. Not only that, the serialization monad provided in Haskell isn't very extensible; see that most serialization libraries are implemented without using `Data.Serialize`.

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

In order to use modules elsewhere, you need to `use` them. This also has an edge case: if the module is another file in your application, you have to declare it with `mod filename;` in your `main.rs`.


To use your own modules from your application in your application, note that you have to `use crate::<module path>`. Inner modules are referred to after the `::`. All these edge cases appear in our `main.rs`.

### Aside in the Aside: impl blocks

These blocks implement functions on data types. These are the methods in Java. There are two flavors of impl block:

- Standalone
- impl trait

A standalone `impl` block reads `impl <type> {` and in the block, you list out methods and implement them. These will be the methods your object can use. `self` is a magic keyword in the block that refers to the instance of the object you're calling the method on (if it's absent, the method is like a static function in Java or C++). `Self` is a magic keyword for the type that you're implementing methods for.

Impls are powerful statements about code, as explained in [our math writeup](some-theory.md#impls-as-theorems).

### Aside in the Aside: Functions

These are the only parts that can contain executed code (the rest of the blocks being (arguably) mere data). OF course, the [Rust book](https://doc.rust-lang.org/beta/book/ch03-03-how-functions-work.html) is more thorough.

We just note a handful of salient features:

- Like in Ruby, `return` values can implicitly be the last evaluated expression's value.
- There is no function overloading (traits can be used if needed).
- The return type need not be "concrete" (it can be an opaque instance of an interface and the compiler does do this to support safe multi-threading).
- Functions are private to the module unless they are prefixed by `pub` (like the struct, enum, and trait declarations that we omit -- enums and structs appear in the tutorial and trait declarations are beyond the scope of this tutorial).