#[derive(Debug, Clone)]
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
    Alpha,
    Num,
    SomeSpace,
    NewLine,
    Spec,
}
fn char_type(c: char) -> CharType {
use crate::CharType::*;
    return if c >= 'a' && c <= 'z' ||
              c >= 'A' && c <= 'Z' ||
              c >= 'а' && c <= 'я' ||
              c >= 'А' && c <= 'Я' {
        Alpha
    } else if c >= '0' && c <= '9' {
        Num
    } else if c == ' ' || c == '\t' {
        SomeSpace
    } else if c == '\n' || c == '\r' || c == '\x0b' || c == '\x0c' {
        NewLine
    } else {
        Spec
    };
}

#[derive(Debug)]
enum TokType {
    Id,
    Oper,
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

    #[derive(Debug, Clone)]
    enum State {
        Null,
        Alpha,
        SomeSpace,
        NewLine,
        Spec,
    }
use State::*;
    let mut state = Null;
    let mut buf = String::new();
    let mut ind: usize = 0;
    let mut line: u32 = 1;
    let mut index: u32 = 1;
    let mut prevloc: Option<Loc> = None;
    let mut push_new_tok = |buf: &mut String,
                            prevloc: &mut Option<Loc>,
                            line: u32, index: u32,
                            tokType: TokType| {
        if buf.is_empty() {
            return;
        }
        let tok = Tok {
            loc: prevloc.clone().unwrap(),
            tokType,
            val: buf.clone(),
        };
        result.push(tok);
        *buf = String::new();
    };
    while let Some(c) = string.nth(0) {
        dbg!(c);
        dbg!(char_type(c));
        dbg!(state.clone());
        match state {
            Null|SomeSpace => {
                match char_type(c) {
                    CharType::Alpha|CharType::Spec => {
                        prevloc = Some(Loc {
                            line,
                            index,
                            filename: String::from("unknown"),
                        });
                        state = Alpha;
                    },
                    CharType::SomeSpace => {
                        state = SomeSpace;
                    },
                    CharType::NewLine => {
                        state = NewLine;
                    },
                    CharType::Spec => {
                        state = Spec;
                    },
                    _ => unreachable!(),
                }
            },
            _ => {},
        }
        //dbg!(state.clone());
        match state {
            Alpha => {
                match char_type(c) {
                    CharType::Alpha => {
                        buf.push(c);
                    },
                    CharType::Spec => {
                        push_new_tok(&mut buf, &mut prevloc, line, index, TokType::Id);
                        buf.push(c);
                        prevloc = Some(Loc {
                            line,
                            index,
                            filename: String::from("unknown"),
                        });
                        state = Spec;
                    },
                    CharType::NewLine => {
                        line += 1;
                        index = 0;
                        state = NewLine;
                        push_new_tok(&mut buf, &mut prevloc, line, index, TokType::Id);
                    },
                    CharType::SomeSpace => {
                        push_new_tok(&mut buf, &mut prevloc, line, index, TokType::Id);
                        state = SomeSpace;
                    },
                    _ => unreachable!(),
                }
            },
            Spec => {
                match char_type(c) {
                    CharType::Alpha => {
                        push_new_tok(&mut buf, &mut prevloc, line, index, TokType::Id);
                        buf.push(c);
                        prevloc = Some(Loc {
                            line,
                            index,
                            filename: String::from("unknown"),
                        });
                        state = Alpha;
                    },
                    CharType::Spec => {
                        buf.push(c);
                    },
                    CharType::NewLine => {
                        line += 1;
                        index = 0;
                        state = NewLine;
                        push_new_tok(&mut buf, &mut prevloc, line, index, TokType::Id);
                    },
                    CharType::SomeSpace => {
                        push_new_tok(&mut buf, &mut prevloc, line, index, TokType::Oper);
                        state = SomeSpace;
                    },
                    _ => unreachable!(),
                }
            },
            NewLine|SomeSpace => {
                match char_type(c) {
                    CharType::Alpha => {
                        //push_new_tok(&mut buf, &mut prevloc, line, index);
                        buf.push(c);
                        prevloc = Some(Loc {
                            line,
                            index,
                            filename: String::from("unknown"),
                        });
                        state = Alpha;
                    },
                    CharType::Spec => {
                        //push_new_tok(&mut buf, &mut prevloc, line, index);
                        buf.push(c);
                        prevloc = Some(Loc {
                            line,
                            index,
                            filename: String::from("unknown"),
                        });
                        state = Spec;
                    },
                    CharType::NewLine => {
                        line += 1;
                        index = 0;
                        state = NewLine;
                    },
                    CharType::SomeSpace => {
                        state = SomeSpace;
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
        ind += 1;
        index += 1;
    }
    //dbg!(state.clone());
    if buf.is_empty() {
        return result;
    }
    push_new_tok(&mut buf, &mut prevloc, line, index, match state {
        Alpha => TokType::Id,
        Spec => TokType::Oper,
        _ => unreachable!(),
    });
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
