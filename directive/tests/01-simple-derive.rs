use directive::directive;

#[directive(keyword = "jump")]
pub struct Jump {}

fn main() {
    let j = Jump {};
}
