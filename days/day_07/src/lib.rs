#![allow(dead_code)]

use std::{collections::HashMap, ops::AddAssign, path::PathBuf};

#[must_use]
pub fn calc_a(input: &str) -> String {
    let (_, folders) = get_files(input);
    folders
        .into_iter()
        .filter(|(_, size)| *size < 100_000)
        .map(|(_, size)| size)
        .sum::<usize>()
        .to_string()
}

#[must_use]
pub fn calc_b(input: &str) -> String {
    let goal = 40_000_000_usize;
    let (_, folders) = get_files(input);
    let used_space = folders.get(&PathBuf::from("/")).unwrap();
    if *used_space < goal {
        return "0".to_string();
    } else {
        let to_remove = used_space - goal;
        let mut folders = folders
            .into_iter()
            .filter(|(_, size)| *size > to_remove)
            .map(|(_, size)| size)
            .collect::<Vec<_>>();
        folders.sort_unstable();
        folders.first().unwrap().to_string()
    }
}

fn get_files(input: &str) -> (Vec<(PathBuf, usize)>, HashMap<PathBuf, usize>) {
    let mut files = Vec::new();
    let commands = parse_commands(input).skip(1).collect::<Vec<_>>();
    let mut current_dir = PathBuf::from("/");
    for command in commands {
        match command {
            Command::Cd { path } => {
                if path == ".." {
                    current_dir.pop();
                } else {
                    current_dir.push(path);
                }
            }
            Command::Ls { content } => {
                for entry in content.iter() {
                    match entry {
                        RawDirEntry::File { name, size } => {
                            files.push((current_dir.join(name), *size));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    let mut folders = HashMap::<PathBuf, usize>::new();
    for (path, size) in &files {
        let mut path = path.to_path_buf();
        while let Some(parent) = path.parent() {
            path = parent.to_path_buf();
            if let Some(f) = folders.get_mut(&path) {
                f.add_assign(size);
            } else {
                folders.insert(path.clone(), *size);
            }
        }
    }

    (files, folders)
}

fn parse_commands<'a>(input: &'a str) -> impl Iterator<Item = Command<'a>> + 'a {
    input
        .trim_matches(['\n', '$', ' '].as_slice())
        .split("\n$ ")
        .map(Command::from)
}

enum Command<'a> {
    Cd { path: &'a str },
    Ls { content: Box<[RawDirEntry<'a>]> },
}

enum RawDirEntry<'a> {
    File { name: &'a str, size: usize },
    Dir { name: &'a str },
}

impl<'a> From<&'a str> for Command<'a> {
    fn from(s: &'a str) -> Self {
        match &s[..2] {
            "cd" => Command::Cd { path: &s[3..] },
            "ls" => Command::Ls {
                content: s[3..].split('\n').map(RawDirEntry::from).collect(),
            },
            cmd => panic!("Invalid command: {}", cmd),
        }
    }
}

impl<'a> From<&'a str> for RawDirEntry<'a> {
    fn from(s: &'a str) -> Self {
        match &s[..3] {
            "dir" => RawDirEntry::Dir { name: &s[4..] },
            _ => {
                let (raw_size, name) = s.split_once(" ").expect("Invalid file entry");

                RawDirEntry::File {
                    name,
                    size: raw_size.parse().expect("Invalid file size"),
                }
            }
        }
    }
}
