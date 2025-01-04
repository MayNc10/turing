use std::{fmt::Debug, io::{stdin, stdout, Write}};

pub mod cli;
pub mod state;
pub mod tape;

pub fn loop_read<T, F, P>(
    buf: &mut String, msg: &str, repeat_msg: &str, transform: F, predicate: P) -> T
where 
    F: Fn(&String) -> T,
    P: Fn(&T) -> bool,
{
    print!("{}", msg);
    stdout().flush().unwrap();
    stdin().read_line(buf).unwrap();
    let mut val = transform(buf);
    buf.clear();
    while !predicate(&val) {
        println!("{}", repeat_msg);
        stdout().flush().unwrap();
        stdin().read_line(buf).unwrap();
        val = transform(buf);
        buf.clear();
    }
    val
}

pub fn loop_read_res<T, S: Debug, F>(
    buf: &mut String, msg: &str, transform: F) -> T
where 
    F: Fn(&String) -> Result<T, S>
{
    print!("{}", msg);
    stdout().flush().unwrap();
    stdin().read_line(buf).unwrap();
    let mut val = transform(buf);
    buf.clear();
    while let Err(err) = val {
        println!("{:?}", err);
        stdout().flush().unwrap();
        stdin().read_line(buf).unwrap();
        val = transform(buf);
        buf.clear();
    }
    val.unwrap()
}