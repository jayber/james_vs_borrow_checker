# [E0521] borrowed data escapes outside of function
```
error[E0521]: borrowed data escapes outside of function                                                                                                                                                                                            
  --> src\main.rs:21:17
   |
16 |   fn display(tree: &MyTree) -> LinearLayout {
   |              ----  - let's call the lifetime of this reference `'1`
   |              |
   |              `tree` is a reference that is only valid in the function body
...
21 |           field = field.on_submit(|cursive_again, _y| {
   |  _________________^
22 | |             cursive_again.pop_layer();
23 | |             cursive_again.add_layer(display(branch));
24 | |         });
   | |          ^
   | |          |
   | |__________`tree` escapes the function body here
   |            argument requires that `'1` must outlive `'static`

For more information about this error, try `rustc --explain E0521`.
```

Here the error reference doesn't much help:
https://doc.rust-lang.org/error_codes/E0521.html
"A type annotation of a closure parameter implies a new lifetime declaration. Consider to drop it, the compiler is reliably able to infer them."
Does not seem to be my problem here

Firstly I started making the tree owned, which lead me to

"[E0507]: cannot move out of `branch`, a captured variable in an `Fn` closure".
I found my way to:
https://doc.rust-lang.org/book/ch13-01-closures.html#closure-type-inference-and-annotation
It slowly dawned on me that the type of the closure was `Fn` which would mean that the move was 
not allowed, because if the closure can run more than once, after the first time, the value has 
already been moved. It would have to be a `FnOnce` type closure to allow that, but the specified 
type is `Fn`.

So I would have to build the display only once and just switch the displayed view/layer/screen on 
the event

src/bin/solution_move.rs is a "solution" that accomplishes this functionality, but it does mean 
that the whole view tree is build at once, rather than building a view when it is required. 