fn main() {
    let names1 = vec!["Bob", "Frank", "Ferris"];

    for name in names1.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    let names2 = vec!["Bob", "Frank", "Ferris"];
    for name in names2.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    let mut names3 = vec!["Bob", "Frank", "Ferris"];
    println!("names: {:?}", names3);
    for name in names3.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names3);
}
