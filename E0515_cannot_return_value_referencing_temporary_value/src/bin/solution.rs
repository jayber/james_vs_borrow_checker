use std::slice::Iter;

struct AContainer<'a> {
    entries: Vec<AnEntry<'a>>
}

impl<'a> AContainer<'a> {
    fn add(&mut self, value: &str) {
        self.entries.push(AnEntry::Plain(String::from(value)));
    }

    fn add_summed(&mut self, entry: AnEntry<'a>) {
        self.entries.push(entry);
    }
}

#[derive(Debug)]
enum AnEntry<'a> {
    Plain(String),
    Summed(Vec<&'a String>)
}

impl AContainer<'_> {

    fn iter(&self) -> Iter<'_, AnEntry<'_>> {
        self.entries.iter()
    }
}

fn main() {
    let mut container = AContainer {entries: vec![]};
    container.add("Stay single");

    let string = String::from("summarize this text");
    let string1 = String::from("summarize this text too");
    let string2 = String::from("summarize me like a pro");
    let summs = vec![&string, &string1, &string2];
    container.add_summed(AnEntry::Summed(summs));

    for entry in container.iter() {
        println!("{:?}", entry);
    }
}
