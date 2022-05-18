mod lex;

const TEST_FILE: &str = "test.txt";

fn main() {
    let mut foo: lex::Lexer = lex::Lexer::new();
    let bar: String = std::fs::read_to_string(TEST_FILE).expect(format!("couldn't read {}", TEST_FILE).as_str());

    foo.feed_file((String::from(TEST_FILE), bar));
    foo.lex();
}