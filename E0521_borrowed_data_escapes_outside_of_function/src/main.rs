use cursive::views::{EditView, LinearLayout};

struct MyTree {
    name: String,
    entries: Vec<MyTree>
}

fn main() {
    let tree = build_tree();

    let mut siv = cursive::default();
    siv.add_layer(display(&tree));
    siv.run();
}

fn display(tree: &MyTree) -> LinearLayout {
    let mut layout = LinearLayout::vertical();

    for branch in tree.entries.iter() {
        let mut field = EditView::new();
        field = field.on_submit(|cursive_again, _y| {
            cursive_again.pop_layer();
            cursive_again.add_layer(display(branch));
        });
        layout.add_child(field);
    }

    layout
}

fn build_tree() -> MyTree {
    MyTree { name: "test".to_string(), entries: vec![]}
}