use derive_directive::directive;

#[derive(Debug)]
#[directive(keyword = "jump")]
pub struct Jump {
    _endpoint_a: String,
    _endpoint_b: String,
    _choice: Option<String>,
}

fn main() {
    match Jump::parse("@jump(x.script)").unwrap() {
        Ok(j) => {
            println!("{:#?}", j);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    };
}
