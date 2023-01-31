```
error[E0507]: cannot move out of `self.entries` which is behind a mutable reference
   --> E0507_cannot_move_out_of_which_is_behind_a_mutable_reference\src\main.rs:9:22
    |
9   |         for entry in self.entries {
    |                      ^^^^^^^^^^^^
    |                      |
    |                      `self.entries` moved due to this implicit call to `.into_iter()`
    |                      move occurs because `self.entries` has type `Vec<String>`, which does not implement the `Copy` trait
    |
note: this function takes ownership of the receiver `self`, which moves `self.entries`
   --> C:\Users\james\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\traits\collect.rs:261:18
    |
261 |     fn into_iter(self) -> Self::IntoIter;
    |                  ^^^^
help: consider iterating over a slice of the `Vec<String>`'s content to avoid moving into the `for` loop
    |
9   |         for entry in &self.entries {
    |                      +
```

The real story is found here, in the link to the error:
https://doc.rust-lang.org/error_codes/E0507.html

The last sections says "Moving a member out of a mutably borrowed struct will also cause E0507 error"
And this is our problem. The solution is to 
[use](https://doc.rust-lang.org/std/mem/fn.replace.html) `mem::replace`. Not something I'd even heard of 
until I saw it here.