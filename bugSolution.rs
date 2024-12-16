fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Length: {}", vec.len());
    match vec.get(2) {
        Some(third) => println!("Third element: {}", third),
        None => println!("There is no third element.")
    };
    //Alternative solution using if let
    if let Some(third) = vec.get(2){
        println!("Third element: {}", third);
    } else {
        println!("There is no third element");
    }
}