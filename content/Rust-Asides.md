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

### Aside: Similarity to the Ubiquitous `null` type

Rustaceans with backgrounds in C-based languages can be quick to retort that `Option` is a glorified null-pointer. However, this is not true. Historically, `null` in OO languages has been implemented as a invariant pointer to an immutable memory address, e.g. `const void *NULL = 0` in C. It's traditionally used as a "default" type or a placeholder, which is intrinsically an unsafe notion, since null-checks aren't rigorously enforced by the compiler.

(In fact, if the Java above wasn't be thoroughly code reviewed you may not even throw an exception and just return null if the user messes up, and you'll have to check for that, and others (me in three months if I were writing it) would forget to.)

The `None` enumeration in Rust's `Option` type, on the other hand, is a very different notion. The idea is that, in these exceptional situations, you can either explicitly address the error case or, if the current function returns a type of the same enumeration, use the `?` operator to [propagate the errors up the call-stack](https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html).

#### Aside in the Aside: Zero Cost Abstractions

Yay! We have a type. The issue, however, with types, is that they can be slower to manage after the code is compiled. After all, in C, bare pointers are tiny -- fitting in registers. But what about some `Option` thing? What's its size?

Here, the Rust compiler has a trick up its sleave: if your type is like a pointer, rust will compile the matches against an `Option` to the nullity check that the good C programmer would've written.

This is a general principle in Rust: a lot of safer and elegant parts of the code actually come at no runtime cost. The better example will be seen with traits in the upcoming weeks.

## Aside: Bindings

The way to make variables in rust is to use `let`. However, it does more than that. This has to do with a broader notion called [pattern matching](https://en.wikipedia.org/wiki/Pattern_matching).

Rust uses pattern matching in various cases -- here is the [chapter in the Rust book](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html) that describes this. `let` bindings can use a special type of pattern matching called [destructuring](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-to-break-apart-values).

Some destructuring will always work: a `struct` (or object) can be destructured into its fields, a tuple (fixed length list if you need an analogy) can be destructured into its contents. Rust calls this an irrefutable binding.

For `enums`, these bindings are not irrefutable: the value being destructured might be under a different label. For example, if you have an `a: Option<Instruction>`, `let Some(value) = a` would not always work.

The `if let` lets you conditionally bind variables to further aid managing `enum`s. This means, instead of only having `?` to boot errors further up the stack, you have `if let` to handle the error.

There are other ways to pattern match against more complex enums too: you use [match](https://doc.rust-lang.org/book/ch18-01-all-the-places-for-patterns.html#match-arms).

## Aside: Serialization

The `serde-rs` library, used in this tutorial, is considered by the open-source community to be the gold standard for safe and predictable serialization. It provides a common API to interact with a multitude of formats, including JSON, Bincode, Pickle (in the Python world), YAML, and more! Both from a usability perspective and a safety perspective, `serde-rs` provides a much better framework for serialization than others of its kind. Below, we provide a comparison between `serde-rs` and similar serialization frameworks in other languages:

### Java's Default Serialization

With Java, objects could be created arbitrarily with serialization and deserialization; this is intrinsically unsafe. In particular, you could inject a crafted serialized object into the program's input, and create any object - potentially one that could hijack the program - therein. Generally speaking, the Java community acknowledges that it was a bad idea to publicly use such a _flexible_ method of serialization, and users of this framework in industry generally tend to have to wrap it with their own paradigms.

With Rust and `serde-rs`, however, you have a much more strict and well-defined deserialization process. If you want a `struct` to deserialize from a JSON document, but only take in some of its fields, then that's all you're going to get. Similarly, arbitrary objects cannot be created the same way they can in Java; in fact, there's very little, if any, opportunity for external programs to hijack control of your Rust program through `serde-rs`.

### C++'s Serialization with Boost

This Rust API for serialization is generally considered more _expressive_ than that provided by the Boost framework in C++; this is because, while you can serialize/deserializae objects in C++ as you can in Rust, you cannot use arbitrary formatting.

### Haskell's `Data.Serialize`

Who the fuck goes to production with Haskell, in the first place...?
