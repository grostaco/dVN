use derive_directive::directive;
use directive_errors::Directive;

#[derive(Debug)]
#[directive(keyword = "jump")]
pub struct Jump {
    endpoint_a: String,
    endpoint_b: u64,
    choice: Option<String>,
    v: Option<i32>,
}

fn main() {
    let j = match Jump::parse("@jump(x.script, 4)").unwrap() {
        Ok(j) => {
            println!("{:#?}", j);
            j
        }
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    assert_eq!(j.v, None);

    assert_eq!(j.endpoint_a, "x.script");
    assert_eq!(j.endpoint_b, 4);
    //assert_eq!(j.choice, None);
}
