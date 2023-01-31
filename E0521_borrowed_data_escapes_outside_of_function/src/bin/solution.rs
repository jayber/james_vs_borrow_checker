use cursive::{Cursive, CursiveRunnable};
use cursive::views::{EditView, LinearLayout};

struct MyTree {
    name: String,
    entries: Vec<MyTree>
}

fn main() {
    let tree = build_tree();

    let mut siv = cursive::default();
    display(&tree, &mut siv);
    siv.run();
}

fn display<'a>(tree: &'a MyTree, siv: &'a mut Cursive) {
    let mut layout = LinearLayout::vertical();

    let vec = &tree.entries;
    for branch in vec.iter() {
        let mut field = EditView::new();
        field = field.on_submit(|cursive_again, _y| {
            cursive_again.pop_layer();
            display(branch, cursive_again);
        });
        layout.add_child(field);
    }


    siv.add_layer(layout);
}

fn build_tree() -> MyTree {
    MyTree { name: "test".to_string(), entries: vec![]}
}