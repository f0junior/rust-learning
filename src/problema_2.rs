/**
 * Converta strings para Pig Latin, onde a primeira consoante de cada palavra é movida
 * para o final da palavra adicionado um "ay", então “first” se torna “irst-fay”.
 * Palavras que começam com uma vogal recebem “hay” adicionado ao final
 * (“apple” torna-se “apple-hay”). Lembre-se sobre a codificação UTF-8!
 */

fn main() {
    let vogais = ['a', 'e', 'i', 'o', 'u'];

    let mut texto = String::from("black");

    let primeiro_caractere = texto.chars().nth(0).unwrap();

    texto.push('-');

    if vogais.contains(&primeiro_caractere) {
        texto.push('h');
    } else {
        texto.remove(0);
        texto.push(primeiro_caractere);
    }

    texto.push_str("ay");

    println!("{:?}", texto);
}
