//
// author: Slyshyk Oleksiy
//

pub const VERSION: &str = concat!("Ver.:", env!("CARGO_PKG_VERSION"), " .\0");
static DESCRIPTION: &str = "rmc - remove C comments.";
static HELP: &str = "usage:\n    rmc path/to/source.c path/to/destination.c";

use bstr::ByteSlice;
mod utils;

struct Args {
    from_file: String,
    to_file: String,
}

fn _main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        let args = Args {
            from_file: args[1].clone(),
            to_file: args[2].clone(),
        };

        let src = utils::file_content(&args.from_file)?;
        let res = remove_blank_lines(&remove_comments(&src)[0..]);
        utils::rewrite_file_content(&args.to_file, &res)?;

    } else {
        eprintln!("{} {}", DESCRIPTION, VERSION.trim());
        eprintln!("{}", HELP);
        std::process::exit(1);
    }

    Ok(())
}

fn main() {
    if let Err(e) = _main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn remove_comments(prgm: &[u8]) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    res.reserve(prgm.len());

    enum PrsState {
        Idle,
        StartComment,
        SingleLineConnent,
        MultiLineComment,
        MultiLineCommentEnd,
    }

    let mut prs_state: PrsState = PrsState::Idle;

    let asterisk = 0x2A_u8; // '*'
    let slash = 0x2F_u8; // '/'
    let line_feed = 0x0A_u8; // '\n'

    for b in prgm.iter() {
        match prs_state {
            PrsState::Idle => {
                if b == &slash {
                    prs_state = PrsState::StartComment;
                } else {
                    res.push(*b);
                };
            }
            PrsState::StartComment => {
                match b {
                    _ if b == &asterisk => prs_state = PrsState::MultiLineComment,
                    _ if b == &slash => prs_state = PrsState::SingleLineConnent,
                    _ => {
                        res.push(slash);
                        prs_state = PrsState::Idle;
                    }
                };
            }
            PrsState::SingleLineConnent => {
                if b == &line_feed {
                    prs_state = PrsState::Idle;
                };
            }
            PrsState::MultiLineComment => {
                if b == &asterisk {
                    prs_state = PrsState::MultiLineCommentEnd;
                }
            }
            PrsState::MultiLineCommentEnd => {
                if b == &slash {
                    prs_state = PrsState::Idle;
                } else {
                    prs_state = PrsState::MultiLineComment;
                };
            }
        }
    }

    res
}

fn remove_blank_lines(prgm: &[u8]) -> Vec<u8> {
    let mut res:Vec<u8> = vec![];
    res.reserve(prgm.len());

    for ln in prgm.lines_with_terminator() {
        if !ln.chars().all(|x| {" \t\n\r".chars().any(|s| s == x)}) {
            res.extend(ln);
        }
    }

    res
}