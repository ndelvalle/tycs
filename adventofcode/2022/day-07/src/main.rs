use std::collections::HashMap;
use std::error::Error;
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

    println!("Result: {}", state.sum_small_dirs(100000));
}
