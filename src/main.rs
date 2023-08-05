use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivinhe o número!");

    let numero_secreto: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Digite o seu palpite.");

        let mut palpite = String::new();

        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler entrada");

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você digitou: {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!!"),
            Ordering::Greater => println!("Muito alto!!"),
            Ordering::Equal => {
                println!("Você acertou!!");
                break;
            }
        }
    }
}
