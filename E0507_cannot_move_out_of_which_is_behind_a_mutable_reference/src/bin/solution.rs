use std::mem;

#[derive(Debug)]
struct AContainer {
    entries: Vec<String>,
}

impl AContainer {
    fn enhance(&mut self) {
        let entries = mem::take(&mut self.entries);

        for entry in entries {
            if !entry.contains("ignore") {
                self.entries.push(entry);
            }
        }
    }
}

fn main() {
    let entries = vec!["val".into(), "val ignore".into()];
    let mut container = AContainer { entries };
    container.enhance();
    println!("{container:?}");
}
