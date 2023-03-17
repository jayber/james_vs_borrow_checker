# [E0726] implicit elided lifetime not allowed here 

Ok, here all I want to do is have a reference member in a struct that implements a trait

The trait in question is `cursive::View`.

The error text suggests
```
help: indicate the anonymous lifetime
   |
14 | impl View for MyView<'_> {
   |                     ++++

```

But doing so just gives me:
`error[E0478]: lifetime bound not satisfied`
Next I tried
```
impl<'a> View for MyView<'a> {

    fn draw(&self, _: &Printer<'_, '_>) { todo!() }
}
```

And got `error[E0478]: lifetime bound not satisfied, note: but lifetime parameter must outlive 
the static lifetime`

Using `impl View for MyView<'static>` gets this part to compile but that doesn't work as now my 
ref must live for static (see the version of `main.rs` that has the commit comment `tried using 
<'static>`

Can I not implement a train in a class that holds a reference?