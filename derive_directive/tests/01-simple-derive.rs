use derive_directive::directive;

#[directive(keyword = "jump")]
pub struct Jump {}

fn main() {
    let _j = Jump {};
}
