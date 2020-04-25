# A Theoretical Background on Rust

This is a opinionated lay of the land, explaining the scenery around which Rust has been built. Rust has not been created in a vacuum and knowing the ideas that existed before should guide people, hopefully, to two things: better ideas, and an ability to accept steady change.

First, here are a lot of links to readings the maintainers have generally used to in developing their views on the theoretical aspects expounded here.

| Link | Type | Why this is here |
|---|---|---|
| [Awodey's Category Theory](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.211.4754&rep=rep1&type=pdf) | Textbook | This is a lot of the mathematics we discuss on the ADTs and we do borrow words from category theory here and there. |
| [Simon Peyton Jones on the development of Haskell](https://youtu.be/re96UgMk6GQ) | Talk | This explains the ideas that founded Haskell. These ideas are ranted against or begged for in various places all over the tutorial. |
| [Bjarne Stroustrup on Modern C++](https://youtu.be/86xWVb4XIyE) | Talk | The whole talk is great, but there is an excellent bunch of [slides on move semantics](https://youtu.be/86xWVb4XIyE?t=2411 "This is an approximate timestamp.") in the middle that are the reason to include it here. |
| [SICP](https://mitpress.mit.edu/sites/default/files/sicp/index.html) | Textbook | The book is an eye-opening introduction to programming and it steadily builds a model of programming that can be very inspirational. [This section](https://mitpress.mit.edu/sites/default/files/sicp/full-text/book/book-Z-H-26.html#%_sec_4.1.3) is the [traditional programming model](#rust-uses-linear-types) mentioned below. |

So, the landscape.

## Simon Peyton Jones' Graph of Languages

Simon Peyton Jones has a graph he likes to use as a mental model for programming languages. So far, this chart holds true, particularly with today's trends. We have:

| Case | Unsafe | Safe |
|---|---|---|
| Fast | C, C++ | (Almost) Rust |
| Slow | N/A | Haskell |

(The languages are an approximation and we mustn't stop dreaming just because of how awesome Rust is. It isn't everything yet, as we may see later on.)

The graph is really about a process over time: aliasing became common, and has led to struggles with safety but it is a very powerful tool. Over time, the side favoring speed has realized that its lack of safety is too costly and the side favoring safety has come to realize that it is too slow for many applications. Rust isn't really some gloarious treaty between the warring factions: somebody independently solved many of the problems found on the speed-favoring side by applying the techniques used in safer languages.

We will follow a similar lay out in this lengthy aside, discussing the theory from safer languages before applying them to practical issues we see in C++. There will then be some discussion on the missing parts and more hypotheses on why other solutions were not chosen.

In order to explain this world Rust is in, there are three sections:

1. Some type theory
1. Some rants against C++ and Haskell
1. Extensions to logic that Rust may or may not yet support

## Some Type Theory

This is too broad and quick an overview of some type theory concepts that make good type systems an absolute joy to read. This goes over the basics of how type theory views types and then tries, very quickly, to apply that view to Rust.

### Predicates as Types

A first difference in perspective is to fundamentally shift what code means. To some enveloped in the theory of coding and proofs, code is a proof. This doesn't (yet) mean that all proofs have a corresponding program, or that it is useful to thing of all programs as the proof of some theorem. This frame of mind helps in the smaller functions and helpers one would write.

In particular, this approach converts types into facts. The basic facts are "I can make a T" where T is a type. If you have a tuple "(T, U)", you know that you can make a T and a U. Furthermore, if you have some sum type:

```rust
pub enum Either<T, U>{
    Left(T),
    Right(U)
}
```

you make a T or a U. Furthermore, functions `T -> U` are implications: once you have a T you can get a U.

Simply put then, every function is a proof of some implication. More concretely, it is a construction (this is the key limitation of type theory in mathematics: that it can't frame the non-constructive matters simply).

Concretely, this means that a function like:

```
pub fn discard_error<T, E>(in: Result<T, E>) -> Option<T> {
    match in {
        Ok(t) => Some(t)
	Err(_) => None
    }
}
```

is a proof that for all result types, there is a corresponding option type. The function body is the following proof:

> Let in be a Result<T, E>. Then, WLOG, it is an Ok(t) for some T t or an Err(e) for some E e. Suppose it is an Ok(t). We can then construct Some(t) as an Option<T>. Otherwise, we can construct None as our option type.

Of course, this is a little meaningless at this level. But, this means that there is a strong contract enforced in the function signature. In particular, we can read a library function's signature to understand what it does. For example, here's `fold`'s signature in Rust:

```rust
// Self is an Iterator
fn fold<B, F>(self, init: B, f: F) -> B
where
    F: FnMut(B, Self::Item) -> B
```

This tells us a few things right off the bat: we need an iterator instance (which we consume), we need a `B` and an `f` that can combine a `B` with the `Item` type we're iterating over to get a new `B` and this will give us a `B`.

Naturally, providing the inductive proof of how `fold` is actually implemented is beyond the scope of Rust's type system. In fact, only extensions of Haskell (or a real proof assistant like Coq, LEAN, Idris, or Adga) could dare take on such a task. However, we can build some smaller gadgets that are rather neat, like fixed-length arrays where the compiler can verify that concatenation indeed adds the sizes (Rust is not yet capable of this cleanly -- I will get back to you after a lot of experimentation).

The main take-away, though, is that theoreticians don't look at types as mere markers like we do, but as propositions about the reality of the world the code is modelling.

### Impls as Theorems

This is really the juicy part. Let's say we have a trait. A trait really is a definition of a class of objects -- like "numbers" or "lists". And the nicest part about Rust's `impl` statement is that it proves theorems about traits. For instance, if the following were an `impl`:

```rust
impl<T: Ord> Ord for ToDoList<T> { ... }
```

we'd be saying:

> For all T that can be totally ordered, the ToDoList<T> can be totally ordered.

This puts us in even better shape for saying general things about our model of the world and having an automatic verification tool (`rustc`, our best friend).

### GADTs, HKTs and the Rest

The above are the concepts Rust fully implements. This section is to mention somethings that remain.

We talk [about sum types](General-Asides.md#aside-sum-types) at length. Product types are structs or tuples. These are fully implemented by Rust and are called **A**lgebraic **D**ata **T**ypes. Here we will extend them some.

In particular, what is a `ToDoList<T>`? It's not just the product of some `Vec<T>` and a `String`. For all `T`, the `ToDoList<T>` is a separate type (each perhaps with its own properties). To generalize this, we look into HKTs.

To start with HKTs, we can ask a foundational question. Where do all the types live? This, in set theory, is a problem, and so that Bertrand Russell doesn't haunt us, we use a layered universe in type theory. In particular, types inhabit a universe. We know these types: they're the ADTs mentioned above or functions. But, to keep our theory simple, universes should really be types too (lest we be set theorists and come up with a new word everytime we need an infinity that much fundamentally larger). We can do this. If the universe, U0 is a type, we put it in U1, a larger universe of types. We then have an infinite hierarchy and all types we can talk about fit in there somewhere. In particular, we can talk about functions to and from Un. These would live in U(n + 1). In fact, Haskell does this and we shall use its notation hereafter.

Let us define Haskell's equivalent to Rust's `Result`:

```hs
data Either a b = Left a | Right b
```

If we ask for what the type of `Either` is (pronounced `:k Either` on ghci, Haskell's interactive REPL), we see `* -> * -> *` which really means it's an arity 2 higher-kinded type (HKT). What elevates this "kind" so that we call it "higher"? The level of the universe it must inhabit!

Rust has no built-in formalism to discuss this. In particular, in Haskell, we can partially invoke HKTs (currying, as the acolytes know) and state facts about them:

```hs
instance Monad (Either e) where
  Left l  >>= _ = Left l
  Right r >>= k = k r
```

This is to say (as we try to earlier) that for a fixed error (`Left e`), we have a Monad instance. Rust cannot express this without explicitly involving the error type. Furthermore, in Haskell, we don't have to state the arity too explicity in the theorems (link in the `instance` block above) until we have to provide the constructions in the implementations themselves.

GADTs are also thought of as dependent types. These are not just higher-kinded types, but those that have lower type parameters. This would be something like `FixedVec: Int -> * -> *`. That is, the type is distinguished by a value from another type. This means that the type checker can, while checking types, make statements about _values_. This is the holy grail of convenient verifiable coding. There is more to verifying all code and this is really a frontier in programming language (and even mathematical in some circles) theory.

(In a cruel twist of fate, C++ templates do allow this. [Example here.](https://gist.github.com/amrali/716d4c342a8f7fc3f23fee8c2b82e300 "And honestly, it's not even ugly."))

## Some Rants Against C++ and Haskell

This is going in-depth on why the status quo actually needed fixing.

### Rant about C++

C++ is complicated. It has a few really good excuses to be, but nobody knows all of it and it's beyond the point where anybody should. In fact, most recommendations leave you with a completely usable subset of the programming language. The rest is relegated for only desparate needs.

Worse yet, C++ is unsafe. And not just in the sense that "Rust is safe -- yay!" We need unsafe code sometimes, but in the sense that Rust uses `unsafe` and C++ drops the distinction between safe and unsafe code in this sense. Really, it means that all code C++ is unsafe. This isn't to say all C++ code is wrong (after all, we're not yet extinct), but that there's no distinction between parts that the compiler has verified and parts where it's trusting the programmer to do the right thing.

Rust has this distinction: it is very unlikely to have a memory error in safe code. And it lets you have full control in distinguished `unsafe` blocks if you need it.

Hence, simply, C++ is fast but dangerous.

The other issue is that templates are not as up-front about the theorem they seek to state. Like [any impl is a theorem](#impls-as-theorems), we'd like templated functions to be theorems, but the complier doesn't actually prove statements about the function being defined until the template is instantiated. [C++20 concepts](https://en.wikipedia.org/wiki/Concepts_(C%2B%2B)) are a fix for some of this issue, but don't actually turn function bodies into proofs yet.

### Rant about Haskell

Haskell is harder to want to rant about, ["avoiding success at all costs,"](https://youtu.be/re96UgMk6GQ?t=715 "A bit more than needed, but Simon Peyton Jones is a personal hero, so ignore me and watch his talks.") it does not have the goals that Rust does. In fact, none of these complaints should tell a Haskell developer to necessarily bother with Rust. It depends on their goals -- in particular: do they care about speed?

Haskell is problematic in this way: it exists for mathematical interest, really bridging the gap between proof assistants and executable code. However, in its purity, it is very difficult to profile. Haskell tries to execute as one would like to execute a mathematical proof: it only evaluates what it absolutely must. Hence, Haskell lists can be infinite and the code can be as slow as one pleases. Here, for instance is a linear search to find the mininal value in an array:

```hs
qs :: (Ord a) => [a] -> [a]
qs []     = []
qs (x:xs) = (filter (< x) xs) ++ [x] ++ (filter (>= x) xs)

min = head . qs
```

The name `qs` may not have been leading enough: it means quick-sort and we will violate the purpose of Haskell and ponder its performance. To find the min is actually linear since to get the `head` (zeroth) element of the list, we have to compute  only the first value, so we only try the left hand side of `++` (the concatenation operator for lists). Recursively, this amounts to comparing each `x` in the list only once. However, `qs`, if evaluated in its entirity, is quadratic time: the `++` operator is linear to compute and we must do such a computation per element of the list.

This is infeasible for critical code. Rust, while losing some of the mathematical purity and elegance of Haskell, does gain a lot of speed and deterministic performance.

## Extensions

This is the most free-form of the sections: wherein I mention all sorts of various topics in mathematical logic that may or may not become relevant to Rust, but is a great influence in ways of thinking about programming languages, particularly future ones. (The antecedent of the _I_ here is Heman -- I've been tasked to add some personal notes, so why not just use first person so you know who's bitching about the theory?)

### Rust and HKTs and GADTs

Full disclosure: I've tried this. [See here.](https://github.com/JasonShin/fp-core.rs/blob/master/fp-core/src/hkt.rs) These are a mess since Rust has yet to really support this. The current progress is documented [here](https://github.com/rust-lang/rfcs/issues/324).

For GADTs, [phantom types](https://doc.rust-lang.org/stable/rust-by-example/generics/phantom.html) are useful since they do not appear at runtime, so the abstraction is (wait for it...) zero cost! This means we can have phantom types that act similarly to the values we'd want to use. In particular, the following (advanced) insanity is supported in [a crate](https://docs.rs/type-operators/0.3.5/type_operators/). This insanity gives us numbers:

```rust
pub trait Nat {
    fn reify(Self) -> u32;
}

// Zero is a number
pub struct ZNat;
impl Nat for ZNat {
     fn reify(self) -> u32 { 0 }
};

// 1 + a number is a number (the successor)
pub struct SNat<A: Nat = ZNat>(<A>);
impl<A: Nat> Nat for SNat<A> {
    fn reify(self) -> u32 {
        let Self(n) = self;
        1 + reify(n)
    }
}
```

So GADTs are just weaker (since the lack of good HKTs limits them) and messier in Rust, but not inaccessible per say.

### Rust Uses Linear Types

The above sections are one approach to building programming languages from mathematical foundations. In particular, they look at type theory to form a full logical system and explore some of the semantics therein.

This is not really the most traditional logical system, though, and can get rather confusing in practice: if types become dependent, how do we know what values are parts of dependent types and what values or actually used by the runtime. If we care about allocation, where are we allocating the data needed to execute branches based on the various types? How are we actually modelling I/O, memory, and state changes? These questions become critical in a complete discussion of programming, and type theory is not the only way to think about it.

In particular, we can build the language from the ground up: we define a syntax as a set of rules telling us what can go where (you would be familiar with these even though it's rare to see them fully written out for a modern programming language -- one tends to learn them by breaking them nowadays). Then, given a well-formed statement in the language, a set of re-write rules can define everything about how to evaluate the programming language. They re-write memory in abstract computers to describe state, so that the underlying mathematics can be stateful if desired.

There is a little more to this model of semantics. In fact, it can be empowered by changing the predicates used. For instance, consider the following Rust:

```rust
let x: NonCopyableType; // just assume there is some default
f(x, x);
```

This doesn't work: `x` is used after it's moved into `f`. In normal (copy-based or reference-based) semantics, this is fine, but Rust doesn't use this commonplace system. To understand Rust's logical system, it is worth knowing [linear logic](https://en.wikipedia.org/wiki/Linear_logic). This is a part of why Rust is so elegantly safe. It uses a semantic system that is more closely tied to a performant model of memory use. I will let you wrestle the borrow checker to understand Rust's model of linear logic.

### Monads Explained

This is an alternative short explanation on what, practically, a Monad is. We have brought them up earlier, but this section is in the abstract.

A Monad is a wrapper of your type with the state of the whole world. This is a way of modelling how Monads create state. They don't: you just make a new world where your state changes have happened each time you use them. Here is how the two functions we mentioned work with this:

- Return: this puts your variable next to a world state. It's you labelling the status quo with your data so that you know how to deal with it later.
- Bind: (recall the signature `m a -> (a -> m b) -> m b`). This lets you change states. It gives you the current label from the world state and lets you run a function on from that label to a new world state. It then spits out the new world state.

OK, this doesn't mean that the Haskell compiler is omniscient. This is a metaphor for what's going on. The metaphor is more literally used in Modal Logic which is the system programming languages may come to care about to reason about stateful code with various dependencies on the real world.

If you know the category theory mantra:

> A monad is just a monoid in the category of endofunctors.

The analogy would be that moving along the endofunctor is changing the world state since that is what the `bind` function we use corresponds to. Return is the identity element of the monoid and doesn't change the world.