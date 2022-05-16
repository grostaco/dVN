use derive_directive::directive;

#[derive(Debug)]
#[directive(keyword = "jump")]
pub struct Jump {
    endpoint_a: String,
    endpoint_b: Option<String>,
    choice: Option<String>,
}

fn main() {
    let j = match Jump::parse("@jump(x.script, y.script)").unwrap() {
        Ok(j) => {
            println!("{:#?}", j);
            j
        }
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    assert_eq!(j.endpoint_a, "x.script");
    assert_eq!(j.endpoint_b, Some("y.script".to_string()));
    assert_eq!(j.choice, None);
}
