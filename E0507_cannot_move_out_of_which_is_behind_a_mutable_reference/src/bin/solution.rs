use std::mem;

struct AContainer {
    entries: Vec<String>
}

impl AContainer {
    fn enhance(&mut self) {
        let new_entries = vec![];

        let entries = mem::replace(&mut self.entries, new_entries);

        for entry in entries {
            if !entry.contains("ignore") {
                self.entries.push(entry);
            }
        }

    }
}

fn main() {
    let entries = vec![String::from("val")];
    let mut container = AContainer {entries};
    container.enhance();
}