use std::io;

fn main() {
    println!("Bitte gib eine Dezimal Zahl ein: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Fehler beim Einlesen");

    match decimal_to_binary(&input) {
        Ok(result) => println!("Binärzahl: {}", result),
        Err(_) => println!("Das ist keine gültige Zahl"),
    }
}

fn decimal_to_binary(input: &str) -> Result<String, std::num::ParseIntError> {
    let mut number: i32 = input.trim().parse()?;

    let mut results = Vec::new();

    while number > 0 {
        let rest = number % 2;
        results.push(rest);
        number /= 2;
    }

    let binary: String = results
        .iter()
        .rev()
        .map(|digit| digit.to_string())
        .collect();
    Ok(binary)
}

fn binary_to_decimal() {
    todo!()
}

fn decimal_to_hexadecimal() {
    todo!()
}

fn hexadecimal_to_decimal() {
    todo!()
}

fn binary_to_hexadecimal() {
    todo!()
}

fn hexadecimal_to_binary() {
    todo!()
}
