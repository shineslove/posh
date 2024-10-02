use poush::token;
fn main() {
    let item = token::Token::ILLEGAL('=');
    println!("{:?}", item);
}
