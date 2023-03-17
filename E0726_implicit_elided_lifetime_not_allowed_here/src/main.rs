use cursive::{Printer, Vec2};
use cursive::View;
use cursive::views::TextView;

fn main() {
    let thing = MyElse { data: "".to_string() };
    let a_view = MyView::new(&thing);

    let mut siv = cursive::default();
    siv.add_fullscreen_layer(a_view);
    siv.run();
}

struct MyElse {
    data: String,
}

struct MyView<'a> {
    something_else: &'a MyElse,
    inner_view: TextView
}

impl MyView<'_> {
    fn new(thing: &MyElse) -> MyView {
        MyView { something_else: &thing, inner_view: TextView::new(thing.data.clone()) }
    }
}

impl View for MyView<'static> {

    fn draw(&self, printer: &Printer) { self.inner_view.draw(printer); }
    fn layout(&mut self, size: Vec2) { self.inner_view.layout(size); }
    fn required_size(&mut self, constraint: Vec2) -> Vec2 { self.inner_view.required_size(constraint) }
}
