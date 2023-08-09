fn main() {
    // Tipo de variáveis - Escalares

    // Inteiros
    let x = 10;
    let y = -5;

    let soma = y + x;
    println!("Soma entre os números {} e {} é igual a {}", y, x, soma);

    // Ponto flutuante
    let a = 2.5;
    let b = 13.7;

    let multiplicacao = a + b;
    println!(
        "A multiplicação entre os números {} e {} é igual a {}",
        a, b, multiplicacao
    );

    // Caracteres
    let emoji_sorrindo = '\u{1F604}';
    println!("Emoji sorrindo: {}", emoji_sorrindo);

    // Composto

    // Tupla - tipos diferentes
    let tupla_teste = (emoji_sorrindo, soma, multiplicacao);
    let (caracter, inteiro, ponto_flutuante) = tupla_teste;

    println!(
        "{}, {}, {}, {}",
        caracter, inteiro, ponto_flutuante, tupla_teste.0
    );

    // Matriz - tipos iguais
    let matriz_teste = [5, 10, 15, 20, 25];
    println!("Primeiro elemento da matriz {}", matriz_teste[0]);
}
