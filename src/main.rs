//
// author: Slyshyk Oleksiy
//

mod utils;

struct Args {
    from_file: String,
    to_file: String,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        let args = Args {
            from_file: args[1].clone(),
            to_file: args[2].clone(),
        };

        match utils::file_content(&args.from_file) {
            Err(e) => {
                eprintln!("Error:{} [{}:{}]", e, file!(), line!());
            }
            Ok(src) => {
                let res = remove_comments(&src);
                match utils::rewrite_file_content(&args.to_file, &res) {
                    Err(e) => {
                        eprintln!("Error:{} [{}:{}]", e, file!(), line!());
                    }
                    _ => {}
                }
            }
        }
    } else {
        eprintln!("XXX");
        std::process::exit(1);
    }
}

fn remove_comments(prgm: &[u8]) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];

    enum PrsState {
        Idle,
        StartComment,
        SingleLineConnent,
        MultiLineComment,
        MultiLineCommentEnd,
    };

    let mut prs_state: PrsState = PrsState::Idle;

    let asterisk = 0x2A_u8;  // '*'
    let slash = 0x2F_u8;     // '/'
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
