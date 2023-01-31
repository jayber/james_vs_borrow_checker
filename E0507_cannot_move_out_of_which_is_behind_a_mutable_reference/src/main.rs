struct AContainer {
    entries: Vec<String>
}

impl AContainer {
    fn enhance(&mut self) {
        let mut new_entries = vec![];

        for entry in self.entries {
            if !entry.contains("ignore") {
                new_entries.push(entry);
            }
        }

        self.entries = new_entries;
    }
}



fn main() {

}