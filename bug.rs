fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Length: {}", vec.len());
    let third: &i32 = vec.get(2).unwrap(); //Error prone line
    println!("Third element: {}", third);
}