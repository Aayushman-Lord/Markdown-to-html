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
    BackQoute,
    NL,
    ParaGraph,
}

// Lexer function
fn lexer(code: &str) -> Vec<Tokens> {
    let mut tokens: Vec<Tokens> = vec![Tokens::START];

    let chars: Vec<char> = code.chars().collect();
    let mut counter = 0;

    while counter < chars.len() {
        let chr = chars[counter];

        let prev_is_text =!( counter > 0 && chars[counter - 1].is_whitespace());

        let next_is_text =! ( counter + 1 < chars.len() && chars[counter + 1].is_whitespace());

        if chr == '#' {
            tokens.push(Tokens::Heading);
        } else if chr == '\'' {
            if prev_is_text || next_is_text {
                tokens.push(Tokens::TEXT(chr.to_string()));
            } else {
                tokens.push(Tokens::QOUTE);
            }
        } else if chr == '"' {
            if prev_is_text || next_is_text {
                tokens.push(Tokens::TEXT(chr.to_string()));
            } else {
                tokens.push(Tokens::DoubleQOUTE);
            }
        } else if chr == '*' {
            if prev_is_text || next_is_text {
                tokens.push(Tokens::TEXT(chr.to_string()));
            } else {
                tokens.push(Tokens::STAR);
            }
        } else if chr == '~' {
            if prev_is_text || next_is_text {
                tokens.push(Tokens::TEXT(chr.to_string()));
            } else {
                tokens.push(Tokens::Tilde);
            }
        } else if chr == '`' {
            if prev_is_text || next_is_text {
                tokens.push(Tokens::TEXT(chr.to_string()));
            } else {
                tokens.push(Tokens::BackQoute);
            }
        } else if chr == ';' {
            if prev_is_text || !next_is_text {
                tokens.push(Tokens::NL);
            } else {
                tokens.push(Tokens::TEXT(chr.to_string()));
            }
        } else if chr == '|' {
            tokens.push(Tokens::ParaGraph);
        } else {
            tokens.push(Tokens::TEXT(chr.to_string()));
        }

        counter += 1;
    }

    tokens.push(Tokens::END);
    tokens
}

// Processor function
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
        // New line
        else if matches!(token, Tokens::NL) {
            a_code.push_str("<br>");
            counter += 1;
            continue;
        } else if matches!(token, Tokens::TEXT(_)) {
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
