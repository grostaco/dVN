use derive_directive::{directive, DirectiveEnum};

#[derive(Debug)]
#[directive(keyword = "jumpA")]
pub struct JumpA {
    endpoint_a: String,
    endpoint_b: u64,
    choice: Option<String>,
}

#[derive(Debug)]
#[directive(keyword = "jumpB")]
pub struct JumpB {
    pub endpoint_a: String,
    endpoint_b: u64,
    choice: Option<String>,
}

#[derive(Debug)]
#[derive(DirectiveEnum)]
pub enum Directives {
    JumpA(JumpA),
    JumpB(JumpB),
}

fn main () {
    let ctx = "@jumpA(1)";

    match Directives::parse(ctx) {
        Some(directive) => {
            println!("Recognized directive {}", ctx);
            match directive {
                Ok(directive) => println!("And well formed\n {:#?}", directive),
                Err(e) => println!("But ill-formed due to {}", e),
            }
        }
        None => println!("Unrecognized directive {}", ctx),
    }
}