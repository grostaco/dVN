#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-simple-derive.rs");
    t.pass("tests/02-parser.rs");
    t.pass("tests/03-missing-positional.rs");
}
