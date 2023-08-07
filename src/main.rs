fn main() {
    // variáveis são imatáveis por padrão.
    let nome = "Francisco";
    println!("Meu nome é {}", nome);

    // para uma variável se tornar mutável é necessário
    // acrescentar o termo mut antes do nome da variável.
    // O tipo não pode ser alterado.

    let mut x = 5;
    x = x * 2;
    println!("Valor de X é {}", x);

    // com Shadowing é possível alterar o valor de
    // uma variável imutável definindo ela novamente
    // com let, mas nesse caso é possível alterar o
    // tipo da variável.

    let espacos = "        ";
    let espacos = espacos.len();
    println!("Número de espaços {}", espacos);
}
