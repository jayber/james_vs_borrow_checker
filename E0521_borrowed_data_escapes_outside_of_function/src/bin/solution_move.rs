use cursive::{CursiveRunnable, ScreenId};
use cursive::views::{EditView, LinearLayout, ScreensView, StackView, TextView};

struct MyTree {
    name: String,
    entries: Vec<MyTree>,
}

fn main() {
    let tree = build_tree();
    let mut siv = cursive::default();
    let (mut siv, _) = display(tree, siv, None);
    siv.run();
}

fn display(tree: MyTree, mut siv: CursiveRunnable, prev_screen_id: Option<ScreenId>) -> (CursiveRunnable, ScreenId) {
    let (current_screen_id, mut layout) = create_new_screen(&tree, &mut siv);

    for branch in tree.entries {
        siv = create_child_field(siv, current_screen_id, &mut layout, branch);
    }

    create_back_field(prev_screen_id, &mut layout);

    siv.set_screen(current_screen_id);
    siv.add_fullscreen_layer(layout);
    (siv, current_screen_id)
}

fn create_new_screen(tree: &MyTree, siv: &mut CursiveRunnable) -> (ScreenId, LinearLayout) {
    let id = siv.add_screen();
    let mut layout = LinearLayout::vertical();
    layout.add_child(TextView::new(tree.name.clone()));
    (id, layout)
}



fn create_back_field(prev_screen_id: Option<ScreenId>, layout: &mut LinearLayout) {
    if let Some(id) = prev_screen_id {
        let mut field = EditView::new().content("up");
        field = field.on_submit(move |siv, _y| {
            siv.set_screen(id);
        });
        layout.add_child(field);
    }
}

fn create_child_field(mut siv: CursiveRunnable, prev_screen_id: ScreenId, layout: &mut LinearLayout, branch: MyTree) -> CursiveRunnable {
    let mut field = EditView::new().content(branch.name.clone());
    let (siv, child_screen_id) = display(branch, siv, Some(prev_screen_id));
    field = field.on_submit(move |siv, _y| {
        siv.set_screen(child_screen_id);
    });
    layout.add_child(field);
    siv
}fn build_tree() -> MyTree {
    let vec1 = vec![MyTree { name: "one".to_string(), entries: vec![] }, MyTree { name: "two".to_string(), entries: vec![] }];
    MyTree { name: "test".to_string(), entries: vec1 }
}