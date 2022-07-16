use std::{fs::{self, File}, io::{self, BufReader, BufRead}};

use super::{directives::Directive, error::{Error, Span, ErrorCause}};

pub struct Script {
    buf: BufReader<File>,
    file: String,
    line: u64,
    last_line: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Dialogue {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub enum ScriptContext {
    Directive(Directive),
    Dialogue(Dialogue),
}

impl Script {
    pub fn new(path: &str) -> io::Result<Self> {
        let file = fs::File::open(path)?;

        Ok(Self { buf: BufReader::new(file), file: path.to_owned(), line: 0, last_line: None })
    }    

    // pub fn line(&self) -> u64 {
    //     self.line
    // }
}

impl Iterator for Script {
    type Item = Result<ScriptContext, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        self.line += 1;

        // TODO: handle the case that read_line doesn't go through
        let mut s = String::new();
        if let Some(ctx) = self.last_line.take() {
            s = ctx;
        } else {
            loop {
                self.line += 1;
                match self.buf.read_line(&mut s) {
                    Ok(0) => return None,
                    Ok(_) if s.trim().is_empty() => continue,
                    Ok(_) => break,
                    Err(e) => panic!("Unhandled IO error {:#?}", e),
                }
                
            }
        }
        s = s.trim().to_string();
        // Directive case
        if s.starts_with('@') {
            match Directive::parse(&s) {
                Some(directive) => match directive {
                    Ok(directive) => Some(Ok(ScriptContext::Directive(directive))),
                    Err(e) => Some(Err(Error::new(&self.file, Span::new(self.line, 1), e.into()) )),
                }
                None => Some(Err(Error::new(&self.file, Span::new(self.line, 1), ErrorCause::Unrecognized(s.get(1..).unwrap().to_owned())))),
            }
        // Dialogue case (hopefully comment support soon)  
        } else if s.starts_with('[') {
            let mut dialogue = String::new();
            let mut line = String::new();

            while let Ok(n) = self.buf.read_line(&mut line) {
                if n != 0 {
                    let c = line.chars().next().unwrap();
                    if c == '@' || c == '[' {
                        self.last_line = Some(line);
                        break;
                    } 
                    dialogue.push_str(line.trim());
                    line.clear();
                } else {
                    break;
                }
            }
            
            Some(Ok(ScriptContext::Dialogue(Dialogue {
                name: s.get(1..s.len() - 1).map_or( String::new(), |v| v.to_string()),
                content: dialogue,
            })))
        } else {
            None
        }   
    }
}
