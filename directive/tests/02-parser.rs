use directive::directive;

#[derive(Debug)]
#[directive(keyword = "jump")]
pub struct Jump {
    endpoint_a: String,
    value: u64,
}

fn main() {
    println!("{:#?}", Jump::parse("A, 12"))
}
