use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::io::Read;
use std::str::FromStr;

type FileSizes = HashMap<Path, usize>;

struct State {
    // $ cd <dir>
    current_dir: Path,
    // $ ls <dir>
    visited_dirs: HashSet<Path>,
    file_sizes: FileSizes,
}

#[derive(Debug, PartialEq, Eq)]
struct Path {
    parts: Vec<String>,
}

impl Path {
    fn full_path(&self) -> String {
        self.parts.join("/")
    }

    fn dir(&self) -> Path {
        let mut parts = self.parts.clone();
        parts.pop();

        Path { parts }
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

    let splits = input
        .split("$")
        .map(|cmd| cmd.trim())
        .filter(|cmd| !cmd.is_empty())
        .map(|cmd| cmd.parse::<Command>().unwrap())
        .collect::<Vec<_>>();

    println!("{:?}", splits);
}
