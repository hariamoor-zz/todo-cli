# More General CS Asides

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

If you read `Box` as "pointer to", you may recognise a binary tree structure. Such recursive trees are broadly applicable as parts of finite state machines or programming language parse trees. But there's more! These structures arise in a confluence of catagory theory and type theory while appearing in useful code.

This appears under various names in various languages. In particular:

| Alias | Where it Appears | Link |
|---|---|---|
| Disjoint Union | Set theory | [Wikipedia](https://en.wikipedia.org/wiki/Disjoint_union) |
| Tagged Union | It's a synonym for sum type | [Wikipedia](https://en.wikipedia.org/wiki/Tagged_union) which includes various implementations of it too |
| Coproduct | Category theory | [Wikipedia](https://en.wikipedia.org/wiki/Coproduct) |

## Aside: The Builder Pattern

The above structure where `.` are chained to form a representation of some data and then a final `.get` function converts or uses the representation, providing a more useful form to the user of the library. This is a useful pattern for libraries to expose complicated and flexible functions while staying legible its to users.

Here is a [Java-based tutorial](https://www.geeksforgeeks.org/builder-design-pattern/) and here is [a boarder tutorial](https://refactoring.guru/design-patterns/builder).