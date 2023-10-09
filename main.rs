#[derive(Debug)]
struct Loc {
    line: u32,
    index: u32,
    filename: String,
}
fn get(filename: &str) -> std::io::Result<String> {
    let got = std::fs::read_to_string(filename)?;
    return Ok(got);
}

#[derive(Debug)]
enum CharType {
    ALPHA,
    NUM,
    SOME_SPACE,
    NEWLINE,
    SPEC,
}
fn char_type(c: char) -> CharType {
use crate::CharType::*;
    return if c >= 'a' && c <= 'z' ||
              c >= 'A' && c <= 'Z' ||
              c >= 'а' && c <= 'я' ||
              c >= 'А' && c <= 'Я' {
        ALPHA
    } else if c >= '0' && c <= '9' {
        NUM
    } else if c == ' ' || c == '\t' {
        SOME_SPACE
    } else if c == '\n' || c == '\r' || c == '\x0b' || c == '\x0c' {
        NEWLINE
    } else {
        SPEC
    };
}

#[derive(Debug)]
enum TokType {
    NUM,
}
#[derive(Debug)]
struct Tok {
    loc: Loc,
    tokType: TokType,
    val: String,
}
/* **********************************
 * Hello, world!                    *
 *      |<     |                    *
 * '   `" '   `"                    *
 *                                  *
 * *********************************/
fn lex(string: &str) -> Vec<Tok> {
    let mut result = Vec::<Tok>::new();
    let mut string = String::from(string);
    let mut string = string.chars();

    let mut buf = String::new();
    let mut ind: u32 = 0;
    while let Some(c) = string.nth(0) {
        dbg!(c);
        match char_type(c) {
            _ => todo!(),
        }
        ind += 1;
    }
    todo!();
    return result;
}

fn main() {
    if std::env::args().len() < 2 {
        panic!("no subcommand");
    }
    match std::env::args().collect::<Vec<String>>()[1].as_str() {
        "get" => {
            if std::env::args().len() < 3 {
                panic!("no source file(s)");
            }
            for i in std::env::args().collect::<Vec<String>>()[2..].iter() {
                println!("trying to get {}", i);
                let got: String = get(i).expect(&(String::from("cannot read file ") + i));
                println!("got: {}", got);
            }
        },
        "lex" => {
            if std::env::args().len() < 3 {
                panic!("no source file(s)");
            }
            for i in std::env::args().collect::<Vec<String>>()[2..].iter() {
                println!("trying to get {}", i);
                let got: String = get(i).expect(&(String::from("cannot read file ") + i));
                println!("got: {}", got);
                println!("trying to lex {}", i);
                let lexed: Vec<Tok> = lex(&got);
                println!("lexed: {:#?}", lexed);
            }
        },
        &_ => unreachable!(),
    }
}
