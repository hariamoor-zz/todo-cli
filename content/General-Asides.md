# General CS Asides

These are points about broader concepts in programming that come up in the tutorial but aren't quite isolated to Rust.

## Aside: Sum Types

This, to the uninitiated, must be strange: the enum seems to be able to reference other fields, like the index or the description. This is not a struct. In type theory, this is a sum type.

Sum types are "labelled choices." The choices may be different per label too, and different labels can have the same "choices." In particular: for our `Instruction`, there are 4 labels: `Add`, `Remove`, `Modify`, and `Print`. Each label has different choices: such as the `usize` index in `Remove`.

These are called `enum`s since Java, C, and C++ enumerations are the labels without the choices necessarily enforced. Another similar construct is a `union` from C and C++, but those are the choices without the labels enforced. Rust `enum`s are both. Have your labels and choices too!

If you'd like to read more about sum types, know that there is not much of an end to what you'd be reading. As a teaser, here's some magic:

```rust
pub enum BinaryTree<T> {
    Node(Box<BinaryTree<T>>, T, Box<BinaryTree<T>>),
    Empty
}
```

If you read `Box` as "pointer to", you may recognise a binary tree structure. Such recursively-defined trees are broadly applicable as parts of finite state machines or programming language parse trees. But there's more! These structures arise in a confluence of category theory and type theory, while appearing in useful code.

This appears under various names in various languages. In particular:

| Alias          | Where it Appears            | Link                                                                                                     |
|----------------|-----------------------------|----------------------------------------------------------------------------------------------------------|
| Disjoint Union | Set theory                  | [Wikipedia](https://en.wikipedia.org/wiki/Disjoint_union)                                                |
| Tagged Union   | It's a synonym for sum type | [Wikipedia](https://en.wikipedia.org/wiki/Tagged_union) which includes various implementations of it too |
| Coproduct      | Category theory             | [Wikipedia](https://en.wikipedia.org/wiki/Coproduct)                                                     |

## Aside: The Builder Pattern

The above structure where `.` are chained to form a representation of some data and then a final `.get` function converts or uses the representation, providing a more useful form to the user of the library. This is a useful pattern for libraries to expose complicated and flexible functions while staying legible its to users.

We supply a [Java-based tutorial](https://www.geeksforgeeks.org/builder-design-pattern/) for those with OOP backgrounds and [a boarder tutorial](https://refactoring.guru/design-patterns/builder) to address the general case.

## Aside: Parallelism and Functional Programming

One of Rust's most salient claims-to-fame is that it provides "fearless concurrency". More particuarly, it provides concurrency or parallelism via an arbitrary concurrent executor, be it Unix processses, Unix threads, or single-threaded asynchronous executors defined in libraries such as [tokio](https://docs.rs/tokio/0.2.18/tokio/). What makes it so easy to write asynchronous code in Rust?

First of all, variables are mutable by default; you can't have race conditions if you nobody can write to data! Immutability is a common design pattern that makes it easy for functional programming languages such as Clojure, through `core.async`, and Erlang through the language itself; with this, you can exploit lock-free concurrency to your heart's content!

Can't do it without mutable state? Mutable shared data can still be synchronized between concurrent contexts via structures in `std::sync`; high-level abstractions provided by the standard library and, in some rare cases, by library maintainers, have got you covered!

## Aside: Category Theory and Type Theory

The amount of value this adds to a productive Rust experience, or for that matter, any kind of productive programming experience, is very debatable; nonetheless, the maintainers, who are hobbyists in algebra, combinatorics, category theory, and type theory, provide the following resources for the mathematically-inclined.

There is more to category theory and type theory than one could possibly hope to cover in any sane number of asides. The following resources are appropriate for hobbyist mathematicians in this aspect:

1. Category Theory for Programmers, by Bartosz Milewski: This is the golden standard for computer scientists and engineers who are also mathematicians. It provides a high-level overview of categories as _algebraic structures_ with enough exposition that a programmer with some level of knowledge in functional design patterns should be able to follow it. One might, in fact, liken it to Abbott's Real Analysis in terms of writing style. However, there are many subjects intrinsic to category theory, such as _sheaves_ and _abelian categories_, that this does not cover.

2. Categories and Sheaves, by Masaki Kashiwara and Pierre Schapira: This book is not for the light-of-heart; it is intended to _rigorously_ cover category theory, as well as touch on some algebraic topology and homotopic type theory. It is touted as an excellent read for algebraists and engineers with strong mathematical backgrounds.

3. Homotopy Type Theory: Univalent Foundations of Mathematics: This book is intended for algebraists and logicians interested in a characterization of mathematics outside of canonical, i.e. Zermelo-Frankel, set theory. With this, it is possible to achieve succinct formalisms for higher-order logic, which is, albeit tangentially, related to that seen in certain functional programming languages like Haskell and Rust.
