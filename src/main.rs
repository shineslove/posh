use poush::token;
fn main() {
    let item = token::Token::ILLEGAL(b'=');
    println!("{:?}", item);
}
