use std::process::exit;

use parser::script::{ScriptContext, Script};

mod parser;
mod core;

fn main() {
    // let ctx = "@jumpA()";
    // match Directive::parse(ctx) {
    //     Some(directive) => {
    //         println!("Recognized directive {}", ctx);
    //         match directive {
    //             Ok(directive) => println!("And well formed\n {:#?}", directive),
    //             Err(e) => println!("But ill-formed due to {}", e),
    //         }
    //     }
    //     None => println!("Unrecognized directive {}", ctx),
    // }
//    Directive::parse("@jumpA()");
    for ctx in Script::new("test_script.script").unwrap() {
        match ctx {
            Ok(ctx) => match ctx {
                ScriptContext::Dialogue(dialogue) => println!("Dialogue of {} with content \"{}\"" , dialogue.name, dialogue.content),
                ScriptContext::Directive(directive) => println!("Directive dump:\n{:#?}", directive),
            },
            Err(e) => { println!("{}", e); exit(1); },
        }
    }
    
}
