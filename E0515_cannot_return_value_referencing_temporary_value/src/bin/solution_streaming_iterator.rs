struct AContainer<'a> {
    entries: Vec<AnEntry<'a>>,
}

impl AContainer<'_> {
    fn add(&mut self, value: &str) {
        self.entries.push(AnEntry::Plain(String::from(value)));
    }
}

#[derive(Debug)]
enum AnEntry<'a> {
    Plain(String),
    Summed(Vec<&'a String>),
}

impl<'a> AnEntry<'a> {
    fn new_summed(entries: Vec<&'a AnEntry>) -> AnEntry<'a> {
        Self::Summed(
            entries
                .into_iter()
                .map(|entry| match entry {
                    AnEntry::Plain(value) => value,
                    AnEntry::Summed(_) => panic!("shouldn't happen"),
                })
                .collect(),
        )
    }

    fn needs_summarizing(&self) -> bool {
        match self {
            AnEntry::Plain(value) => value.contains("summarize"),
            AnEntry::Summed(_) => false,
        }
    }
}

impl<'a> AContainer<'a> {
    fn to_streaming_iter(&'a self) -> StreamingIter<'a> {
        let mut si = StreamingIter::default();
        let mut summed = vec![];
        for entry in self.entries.iter() {
            if entry.needs_summarizing() {
                summed.push(entry)
            } else {
                si.normal.push(entry);
            }
        }
        if !summed.is_empty() {
            si.summed = Some(AnEntry::new_summed(summed));
        }
        StreamingIter {
            normal: si.normal,
            summed: si.summed,
            index: 0,
        }
    }
}

#[derive(Default)]
struct StreamingIter<'a> {
    normal: Vec<&'a AnEntry<'a>>,
    summed: Option<AnEntry<'a>>,
    index: usize,
}

impl<'a> StreamingIter<'a> {
    // Use explicit lifetimes to show how
    // the returned Option borrows for as long as the borrow initiazted by next (i.e. 'b')
    fn next<'b>(&'b mut self) -> Option<&'b AnEntry<'a>> {
        match self.index.cmp(&(self.normal.len())) {
            std::cmp::Ordering::Less => {
                self.index += 1;
                Some(self.normal[self.index - 1])
            }
            std::cmp::Ordering::Equal => {
                self.index += 1;
                self.summed.as_ref()
            }
            std::cmp::Ordering::Greater => None,
        }
    }
}

fn main() {
    let mut container = AContainer { entries: vec![] };
    container.add("Stay single");
    container.add("summarize this text");
    container.add("summarize this text too");
    container.add("summarize me like a pro");

    let mut si = container.to_streaming_iter();
    while let Some(entry) = si.next() {
        println!("{:?}", entry);
    }
}
