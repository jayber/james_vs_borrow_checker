use std::vec::IntoIter;

struct AContainer<'a> {
    entries: Vec<AnEntry<'a>>
}

impl AContainer<'_> {
    fn add(&mut self, value: &str) {
        self.entries.push(AnEntry::Plain(String::from(value)));
    }
}

#[derive(Debug)]
enum AnEntry<'a> {
    Plain(String),
    Summed(Vec<&'a String>)
}

impl<'a> AnEntry<'a> {
    fn new_summed(entries: Vec<&'a AnEntry>) -> AnEntry<'a> {
        Self::Summed(entries.into_iter().map(|entry| match entry {
            AnEntry::Plain(value) => value,
            AnEntry::Summed(_) => panic!("shouldn't happen")
        }).collect())
    }

    fn needs_summarizing(&self) -> bool {
        match self {
            AnEntry::Plain(value) => value.contains("summarize"),
            AnEntry::Summed(_) => false
        }
    }
}

impl AContainer<'_> {
    fn iter(&self) -> IntoIter<&AnEntry<'_>> {
        let mut new_entries = vec![];
        let mut summed = vec![];
        for entry in self.entries.iter() {
            if entry.needs_summarizing() {
                summed.push(entry)
            } else {
                new_entries.push(entry);
            }
        }
        if !summed.is_empty() {
            new_entries.push(&AnEntry::new_summed(summed));
        }
        new_entries.into_iter()
    }
}

fn main() {
    let mut container = AContainer {entries: vec![]};
    container.add("Stay single");
    container.add("summarize this text");
    container.add("summarize this text too");
    container.add("summarize me like a pro");
    
    for entry in container.iter() {
        println!("{:?}", entry);
    }
}
