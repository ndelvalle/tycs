use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::io::Read;
use std::str::FromStr;

type FileSizes = HashMap<Path, usize>;

type DirSizes = HashMap<Path, usize>;

#[derive(Debug)]
struct State {
    current_dir: Path,
    file_sizes: FileSizes,
}

impl State {
    fn new(initial: Path) -> Self {
        State {
            current_dir: initial,
            file_sizes: HashMap::new(),
        }
    }

    fn visit_file(&mut self, path: Path, size: usize) {
        self.file_sizes.insert(path, size);
    }

    fn visit_dir(&mut self, path: Path) {
        self.current_dir = self.current_dir.join(&path);
    }

    /// Add the size of each file to its diredtory and all its parent directories.
    fn dir_sizes(&mut self) -> DirSizes {
        let mut dir_sizes = DirSizes::new();

        for (path, size) in &self.file_sizes {
            let mut dir = path.dir();

            while !dir.parts.is_empty() {
                let dir_size = dir_sizes.entry(dir.clone()).or_insert(0);
                *dir_size += size;

                dir = dir.dir();
            }
        }

        dir_sizes
    }

    fn sum_small_dirs(&mut self, threshold: usize) -> usize {
        let dir_sizes = self.dir_sizes();

        dir_sizes
            .iter()
            .filter_map(|(_, size)| Some(size).filter(|size| **size < threshold))
            .sum()
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Path {
    parts: Vec<String>,
}

impl Path {
    fn new(parts: Vec<String>) -> Self {
        Path { parts }
    }

    fn dir(&self) -> Path {
        let mut parts = self.parts.clone();
        parts.pop();

        Path { parts }
    }

    fn join(&self, other: &Path) -> Path {
        let mut parts = self.parts.clone();
        parts.extend(other.parts.clone());

        Path { parts }
    }
}

impl FromStr for Path {
    type Err = Box<dyn Error>;

    fn from_str(path: &str) -> Result<Self, Self::Err> {
        let parts = path.split("/").map(|part| part.to_string()).collect();

        Ok(Path::new(parts))
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let path = self
            .parts
            .iter()
            .map(|part| part.to_string())
            .collect::<Vec<_>>()
            .join("/");

        write!(f, "{}", path)
    }
}

#[derive(Debug)]
enum LsItem {
    Dir(String),
    File(String, usize),
}

impl FromStr for LsItem {
    type Err = Box<dyn Error>;

    fn from_str(ls_line: &str) -> Result<Self, Self::Err> {
        match ls_line.split_once(" ").unwrap() {
            ("dir", name) => Ok(LsItem::Dir(name.to_string())),
            (size, name) => Ok(LsItem::File(
                name.to_string(),
                size.parse::<usize>().unwrap(),
            )),
        }
    }
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls(Vec<LsItem>),
}

impl FromStr for Command {
    type Err = Box<dyn Error>;

    fn from_str(cmd: &str) -> Result<Self, Self::Err> {
        let mut lines = cmd.lines().collect::<Vec<&str>>();
        let cmd: &str = lines.remove(0);

        match cmd.split_at(2) {
            ("ls", _) => {
                let items = lines
                    .into_iter()
                    .map(|line| line.parse::<LsItem>().unwrap())
                    .collect::<Vec<_>>();

                Ok(Command::Ls(items))
            }
            ("cd", dir) => Ok(Command::Cd(dir.trim().to_string())),
            (cmd, _) => panic!("Unknown command {cmd}"),
        }
    }
}

fn read_stdin() -> Result<String, Box<dyn Error>> {
    let mut buf: Vec<u8> = vec![];

    io::stdin().read_to_end(&mut buf)?;

    Ok(String::from_utf8(buf)?)
}

const MAX_SIZE: usize = 70000000;
const WANTED_FREE_SPACE: usize = 30000000;

fn main() {
    let input = read_stdin().unwrap();

    let commands = input
        .split("$")
        .map(|cmd| cmd.trim())
        .filter(|cmd| !cmd.is_empty())
        .map(|cmd| cmd.parse::<Command>().unwrap())
        .collect::<Vec<_>>();

    let state = &mut State::new(Path::new(vec![]));

    // TODO: iterate once instead of twice (here and in `State::dir_sizes`)
    for cmd in &commands {
        match cmd {
            Command::Cd(dir) => match dir {
                dir if dir == ".." => {
                    state.current_dir = state.current_dir.dir();
                }
                dir if dir == "/" => {}
                dir => {
                    state.visit_dir(dir.parse::<Path>().unwrap());
                }
            },
            Command::Ls(items) => {
                for item in items {
                    match item {
                        LsItem::Dir(_) => {}
                        LsItem::File(name, size) => {
                            let path = state
                                .current_dir
                                .clone()
                                .join(&name.parse::<Path>().unwrap());

                            state.visit_file(path, *size);
                        }
                    }
                }
            }
        };
    }

    println!("Part 1: {}", state.sum_small_dirs(100000));

    let used_space = state.file_sizes.values().sum::<usize>();
    let free_space = MAX_SIZE - used_space;

    let mut dirs = state.dir_sizes().into_iter().collect::<Vec<_>>();

    dirs.sort_by(|(_, s1), (_, s2)| s1.cmp(s2));

    let to_delete = dirs
        .into_iter()
        .find(|(_, size)| (*size + free_space) >= WANTED_FREE_SPACE);

    println!("Part 2: {}", to_delete.unwrap().1);
}
