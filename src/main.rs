use std::io;

fn main() {
    
    let mut input = String::new();

println!("Bitte gib eine Dezimal Zahl ein: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Fehler beim Einlesen");
   
    let mut number: i32 = input
        .trim()
        .parse()
        .expect("Bitte eine gültige Zahl eingeben!");
    

    let mut results = Vec::new();
    
    while number > 0 {
        let rest = number % 2;
        results.push(rest);
        number /= 2;
    }
    println!("Binärzahl: ");
    for reversed in results.iter().rev() {
        print!("{}", reversed);
    }
    println!();



}