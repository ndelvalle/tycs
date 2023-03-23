use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::io::Read;

fn read_stdin() -> Result<String, Box<dyn Error>> {
    let mut buf: Vec<u8> = vec![];

    io::stdin().read_to_end(&mut buf)?;

    Ok(String::from_utf8(buf)?)
}

fn main() {
    let input = read_stdin().unwrap();

    let (result, _) = input
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .enumerate()
        .find(|(_, window)| window.into_iter().collect::<HashSet<_>>().len() == 4).unwrap();

    println!("Start of packet marker: {}", result + 4);

    let (result, _) = input
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .enumerate()
        .find(|(_, window)| window.into_iter().collect::<HashSet<_>>().len() == 14).unwrap();

    println!("Start of message marker: {}", result + 14);
}
