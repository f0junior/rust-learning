fn main() {
    println!("A área de um retângulo {}x{} é {}", 3, 4, area_retangulo(3, 4));
}

/**
 * Funções tem seu nome escrito de forma snake case.
 * Deve informar o tipo dos parâmetros e se tiver
 * retorno informar o tipo do retorno conforme exemplo
 * abaixo.
 */

fn area_retangulo(base: i32, altura: i32) -> f64 {
    let base = base as f64;
    let altura = altura as f64;

    //return base * altura;
    base * altura
}
