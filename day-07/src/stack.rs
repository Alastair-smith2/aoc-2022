// Credit goes to https://fasterthanli.me/ for this

#[derive(Debug, PartialEq)]
pub struct FileEntry<'a> {
    pub path: &'a str,
    pub size: usize,
    pub children: Vec<FileEntry<'a>>,
}

impl<'a> FileEntry<'a> {
    pub fn total_size(&self) -> usize {
        self.size + self.children.iter().map(|c| c.total_size()).sum::<usize>()
    }

    pub fn all_dirs(&self) -> Box<dyn Iterator<Item = &FileEntry> + '_> {
        println!("How often do I get called");
        Box::new(
            std::iter::once(self).chain(
                self.children
                    .iter()
                    .filter(|c| !c.children.is_empty())
                    .flat_map(|c| c.all_dirs()),
            ),
        )
    }
}
