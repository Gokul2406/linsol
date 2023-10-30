fn main() {
    let mut elements: String = String::new();
    println!("Type 4 elements spearated by spaces ");
    std::io::stdin().read_line(&mut elements).unwrap();
    println!("{:?}", elements.trim());

    let mut elem_vec: Vec<&String> = Vec::new();


}
