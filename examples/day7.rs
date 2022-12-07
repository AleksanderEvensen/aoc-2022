#[derive(Debug, PartialEq, Eq)]
enum EntryType {
    File,
    Directory,
}

#[derive(Debug)]
struct FSEntry {
    parent: usize,
    name: String,
    entry_type: EntryType,
    size: i32,
}

#[derive(Debug)]
struct FileSystem {
    entries: Vec<FSEntry>,
    current_dir: usize,
}

impl FileSystem {
    pub fn add_entry(&mut self, entry: FSEntry) -> usize {
        self.entries.push(entry);

        self.entries.len() - 1
    }

    pub fn move_into(&mut self, folder_index: usize) {
        self.current_dir = folder_index;
    }

    pub fn move_up(&mut self) {
        self.current_dir = self.entries.get(self.current_dir).unwrap().parent;
    }

    pub fn get_children_of(&self, index: usize) -> Vec<(usize, &FSEntry)> {
        self.entries
            .iter()
            .enumerate()
            .filter(|(_, v)| v.parent == index)
            .collect::<Vec<(usize, &FSEntry)>>()
    }

    pub fn calculate_size_of(&self, index: usize) -> i32 {
        let mut size = 0;

        for (child_index, child) in self.get_children_of(index) {
            size += child.size + self.calculate_size_of(child_index);
        }

        size
    }
}

fn main() {
    let inputs = std::fs::read_to_string("./inputs/day7.txt").unwrap();
    let inputs = inputs.lines().collect::<Vec<&str>>();
    let mut line_iter = inputs.iter().peekable();

    let mut file_system = FileSystem {
        current_dir: 0,
        entries: vec![],
    };

    file_system.add_entry(FSEntry {
        parent: 69420,
        entry_type: EntryType::Directory,
        name: "/".to_string(),
        size: 0,
    });

    while let Some(line) = line_iter.next() {
        let is_command = line.starts_with("$");

        if is_command {
            let command = line
                .strip_prefix("$ ")
                .unwrap()
                .trim()
                .split_whitespace()
                .collect::<Vec<&str>>();

            match command[0] {
                "cd" => match command[1] {
                    "/" => {
                        file_system.move_into(0);
                    }
                    ".." => file_system.move_up(),
                    name => {
                        if let Some(v) = file_system.entries.iter().position(|v| {
                            v.parent == file_system.current_dir && v.name == name.to_string()
                        }) {
                            file_system.move_into(v);
                        } else {
                            let index = file_system.add_entry(FSEntry {
                                parent: file_system.current_dir,
                                name: name.to_string(),
                                entry_type: EntryType::Directory,
                                size: 0,
                            });
                            file_system.move_into(index);
                        }
                    }
                },
                "ls" => {
                    let mut output = vec![];
                    while let Some(next_line) = line_iter.peek() {
                        if next_line.starts_with("$") {
                            break;
                        }
                        output.push(line_iter.next().unwrap())
                    }

                    output.iter().for_each(|ls_out| {
                        let split = ls_out.split_whitespace().collect::<Vec<&str>>();

                        let file_name = String::from(split[1]);

                        match split[0] {
                            "dir" => {
                                file_system.add_entry(FSEntry {
                                    name: file_name,
                                    parent: file_system.current_dir,
                                    entry_type: EntryType::Directory,
                                    size: 0,
                                });
                            }

                            size => {
                                let size = size.parse::<i32>().unwrap();

                                file_system.add_entry(FSEntry {
                                    parent: file_system.current_dir,
                                    name: file_name,
                                    entry_type: EntryType::File,
                                    size,
                                });
                            }
                        }
                    });
                }
                _ => unreachable!(),
            }
        }
    }

    let sum_size: i32 = file_system
        .entries
        .iter()
        .enumerate()
        .filter(|(i, _)| file_system.calculate_size_of(*i) <= 100_000)
        .map(|(i, _)| file_system.calculate_size_of(i))
        .sum();

    let total_fs_size = file_system.calculate_size_of(0);
    let min_delete_size = total_fs_size - 40_000_000;

    let mut index_to_delete = 0;

    for (i, entry) in file_system.entries.iter().enumerate() {
        if entry.entry_type != EntryType::Directory {
            continue;
        }

        let size_of = file_system.calculate_size_of(i);
        if size_of < min_delete_size {
            continue;
        }

        let current_size_of = file_system.calculate_size_of(index_to_delete);

        if size_of < current_size_of {
            index_to_delete = i;
        }
    }

    println!("Part 1: {}", sum_size);
    println!("Part 2: {}", file_system.calculate_size_of(index_to_delete))
}
