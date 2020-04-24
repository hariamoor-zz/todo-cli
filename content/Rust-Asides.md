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

In general, the idea of "throwing" an "exception" in the middle of the code when something doesn't go the way you want it to, and unwinding the call-stack until someone catches it, is a very object-oriented concept. This has its roots in C-style error-handling with POSIX signals (did someone say "Segmentation fault (core dumped)"?)

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

For the uninitiated, the above is absolute gibberish; however, in essence, Haskell provides a generic type called `Monad` defined by (1) a generic type `a`, and; (2) functions called `bind` and `return`. The `bind` function applies another monadic transformation respecting the same monad `m` iff `a` is valid, and the `return` function "unwraps" `a` so that it can be used as a normal value.

In general, this is a much more succinct pattern for error-handling. In cases where you want to bother the user (which, for small applications such as this one, is most of them), it DWIMs and propagates the error, defined here as a `String`, up the call stack. On the other hand, if the developer wants to explicitly handle the error, he may do so with Haskell's pattern-matching syntax.

Idiomatic Rust follows a very similar system; the `Result` and `Option` types are isomorphic to trivially-defined monads in Haskell. In general, it is considered unsavory to `panic!`, especially explicitly. You either propagate the error up the stack or manually address it; however, if you don't want to do either of those things, there is also an `.unwrap()` function that returns the underlying type or `panic!`s internally.

### Aside: Similarity to the Ubiquitous `null` type

Rustaceans with backgrounds in C-based languages can be quick to retort that `Option` is a glorified null-pointer. However, this is not true. Historically, `null` in OO languages has been implemented as a invariant pointer to an immutable memory address, e.g. `const void *NULL = 0` in C. It's traditionally used as a "default" type or a placeholder, which is intrinsically an unsafe notion, since null-checks aren't rigorously enforced by the compiler.

(In fact, if the Java above wasn't be thoroughly code reviewed you may not even throw an exception and just return null if the user messes up, and you'll have to check for that, and others (me in three months if I were writing it) would forget to.)

The `None` enumeration in Rust's `Option` type, on the other hand, is a very different notion. The idea is that, in these exceptional situations, you can either explicitly address the error case or, if the current function returns a type of the same enumeration, use the `?` operator to [propagate the errors up the call-stack](https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html).

#### Aside in the Aside: Zero Cost Abstractions

Yay! We have a type. The issue, however, with types, is that they can be slower to manage after the code is compiled. After all, in C, bare pointers are tiny -- fitting in registers. But what about some `Option` thing? What's its size?

Here, the Rust compiler has a trick up its sleeve; if the underlying type implements the `Deref` trait, i.e. it's "like" a pointer, then this idiomatic Rust code is isomorphic to the equivalent C code that applies a null-check.

This is a rather useful pattern in Rust: a lot of safer and elegant parts of the code actually come at no runtime cost.

## Aside: Bindings

The way to make variables in rust is to use `let`. However, it does more than that. This has to do with a broader notion called [pattern matching](https://en.wikipedia.org/wiki/Pattern_matching).

Rust uses pattern matching in various cases -- see the associated [chapter in the Rust book](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html) that describes this. `let` bindings can use a special type of pattern matching called [destructuring](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-to-break-apart-values).

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

### Haskell's Serialization with `Data.Serialize`

Who the fuck goes to production with Haskell, in the first place...?

OK, this isn't much of an argument. It just so happens that this Haskell module has the same limitation as the C++ library; there's no generic solution for serialization to an arbitrary format (or at least a collection of distinct formats).

Not only that, the serialization monad provided in Haskell isn't very extensible; see that most serialization libraries are implemented without using `Data.Serialize`.
