use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let path = "file.test";
    let code = fs::read_to_string(path).expect("Error: failed to read file.");
    let code = lexer(&code);
    let code = processor(code);
    compiler(&code);
}

// Tokens
#[derive(Debug, PartialEq, Eq)]
enum Tokens {
    START,
    END,
    Heading,
    TEXT(String),
    QOUTE,
    DoubleQOUTE,
    STAR,
    Tilde,
    NL,
    ParaGraph,
    BackTick,
}

// Lexer function
fn lexer(code: &str) -> Vec<Tokens> {
    let mut tokens: Vec<Tokens> = vec![Tokens::START];

    let chars: Vec<char> = code.chars().collect();
    let mut counter = 0;

    while counter < chars.len() {
        let chr = chars[counter];

        if chr == '#' {
            tokens.push(Tokens::Heading);
        } else if chr == '\'' {
            tokens.push(Tokens::QOUTE);
        } else if chr == '"' {
            tokens.push(Tokens::DoubleQOUTE);
        } else if chr == '*' {
            tokens.push(Tokens::STAR);
        } else if chr == '~' {
            tokens.push(Tokens::Tilde);
        } else if chr == ';' {
            tokens.push(Tokens::NL);
        } else if chr == '|' {
            tokens.push(Tokens::ParaGraph);
        } else if chr == '`' {
            tokens.push(Tokens::BackTick);
        } else {
            tokens.push(Tokens::TEXT(chr.to_string()));
        }

        counter += 1;
    }

    tokens.push(Tokens::END);
    tokens
}

// Processor function
fn processor(code: Vec<Tokens>) -> String {
    if code.first() != Some(&Tokens::START) {
        println!("Error: start is missing.");
    }

    let mut a_code = String::from("<!DOCTYPE html>\n<html><body>");

    let mut counter = 0;

    while counter < code.len() {
        let token = &code[counter];

        // H1
        if token == &Tokens::Heading && !matches!(code.get(counter + 1), Some(Tokens::Heading)) {
            a_code.push_str("<h1>");
            counter += 1;

            while counter < code.len() && matches!(code[counter], Tokens::TEXT(_)) {
                if let Tokens::TEXT(text) = &code[counter] {
                    a_code.push_str(text);
                }
                counter += 1;
            }

            a_code.push_str("</h1>");
            continue;
        }
        // H2
        else if token == &Tokens::Heading
            && matches!(code.get(counter + 1), Some(Tokens::Heading))
        {
            a_code.push_str("<h2>");
            counter += 2;

            while counter < code.len() && matches!(code[counter], Tokens::TEXT(_)) {
                if let Tokens::TEXT(text) = &code[counter] {
                    a_code.push_str(text);
                }
                counter += 1;
            }

            a_code.push_str("</h2>");
            continue;
        }
        // Paragraph
        else if matches!(token, Tokens::ParaGraph) {
            a_code.push_str("<p>");
            counter += 1;

            while counter < code.len() && !matches!(code[counter], Tokens::ParaGraph) {
                if let Tokens::TEXT(text) = &code[counter] {
                    a_code.push_str(text);
                }
                counter += 1;
            }

            a_code.push_str("</p>");
            counter += 1;
            continue;
        }
        // Italic
        else if matches!(token, Tokens::STAR)
            && !matches!(code.get(counter + 1), Some(Tokens::STAR))
        {
            a_code.push_str("<em>");
            counter += 1;

            while counter + 1 < code.len() {
                let token = &code[counter];

                if matches!(token, Tokens::STAR) {
                    break;
                }

                if let Tokens::TEXT(text) = token {
                    a_code.push_str(text);
                }

                counter += 1;
            }

            a_code.push_str("</em>");
            counter += 1;
            continue;
        }
        // Strong bold
        else if matches!(token, Tokens::STAR)
            && matches!(code.get(counter + 1), Some(Tokens::STAR))
        {
            a_code.push_str("<strong>");
            counter += 2;

            while counter + 1 < code.len() {
                let token = &code[counter];

                if matches!(token, Tokens::STAR)
                    && matches!(code.get(counter + 1), Some(Tokens::STAR))
                {
                    break;
                }

                if let Tokens::TEXT(text) = token {
                    a_code.push_str(text);
                }

                counter += 1;
            }

            a_code.push_str("</strong>");
            counter += 2;
            continue;
        }
        // Strike
        else if matches!(token, Tokens::Tilde)
            && matches!(code.get(counter + 1), Some(Tokens::Tilde))
        {
            a_code.push_str("<del>");
            counter += 2;

            while counter + 1 < code.len() {
                let token = &code[counter];

                if matches!(token, Tokens::Tilde)
                    && matches!(code.get(counter + 1), Some(Tokens::Tilde))
                {
                    break;
                }

                if let Tokens::TEXT(text) = token {
                    a_code.push_str(text);
                }

                counter += 1;
            }

            a_code.push_str("</del>");
            counter += 2;
            continue;
        }
        // Code
        else if matches!(token, Tokens::BackTick) {
            a_code.push_str("<code>");
            counter += 1;

            while counter < code.len() {
                let token = &code[counter];

                if matches!(token, Tokens::BackTick) {
                    break;
                }

                if let Tokens::TEXT(text) = token {
                    a_code.push_str(text);
                }

                counter += 1;
            }

            a_code.push_str("</code>");
            counter += 1;
            continue;
        }
        // New line
        else if matches!(token, Tokens::NL) {
            a_code.push_str("<br>");
            counter += 1;
            continue;
        }
        // Text
        else if matches!(token, Tokens::TEXT(_)) {
            if let Tokens::TEXT(text) = &code[counter] {
                a_code.push_str(text);
            }
        }

        counter += 1;
    }

    if code.last() != Some(&Tokens::END) {
        println!("Error: end token missing.");
    }

    a_code.push_str("</body></html>");
    a_code
}

// Compiler function
fn compiler(code: &str) {
    let mut file = File::create("output.html").expect("Error: cannor create file.");
    file.write_all(code.as_bytes())
        .expect("Error: cannot write to file");
}
