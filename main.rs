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
struct Tok {
    loc: Loc,
    val: String,
}
fn lex(string: &str) -> Vec<Tok> {
    let splitted = string.split_whitespace();
    let result: Vec<Tok> = splitted.map(|x| Tok { loc: Loc { line: 1488, index: 1488, filename: String::from("[null]") }, val: String::from(x) }).collect();
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
