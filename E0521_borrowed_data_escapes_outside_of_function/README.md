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