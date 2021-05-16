mod parser;

fn main() {
    //println!("Hello, world!");
    let file = std::env::args().nth(1).unwrap();
    println!("file read from: {}", file);
    //let _ = parser::parse_file(file);
    println!("{:?}", parser::parse_file(file))
}
