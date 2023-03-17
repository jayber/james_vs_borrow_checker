struct AContainer {
    entries: Vec<String>,
}

impl AContainer {
    fn enhance(&mut self) {
        let mut new_entries = vec![];
        let entries = self.entries.to_owned();

        for entry in entries {
            if !entry.contains("ignore") {
                new_entries.push(entry);
            }
        }

        self.entries = new_entries;
    }
}

fn main() {
    let entries = vec![String::from("val"), String::from("val ignore")];
    let mut container = AContainer { entries };
    container.enhance();
}
