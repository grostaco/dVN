use derive_directive::{directive, DirectiveEnum};
use directive_errors::VerifyError;

#[derive(Debug)]
#[derive(DirectiveEnum)]
pub enum Directives {
    Jump(Jump),
}

#[derive(Debug)]
#[directive(keyword = "jump", verify = jump_verify)]
pub struct Jump {
    endpoint_a: String,
    endpoint_b: Option<String>,
    choice: Option<String>,
}

fn jump_verify(j: &Jump) -> Result<(), VerifyError>{
    if j.endpoint_b.is_some() && j.choice.is_none() {
        return Err(VerifyError::Custom("jump and choice must both be set or not set at all".to_string()));
    }

    Ok(())
}

fn main() {
    let ctx = "@jump(A.txt, 1)";
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
