#[derive(Debug)]
struct Retangulo {
    altura: u32,
    base: u32,
}

impl Retangulo {
    fn from(altura: u32, base: u32) -> Retangulo {
        Retangulo { altura, base }
    }

    fn area(&self) -> u32 {
        self.altura * self.base
    }

    fn pode_conter(&self, other: &Retangulo) -> bool {
        self.altura > other.altura && self.base > other.base
    }
}

fn main() {
    let rect1 = Retangulo::from(60, 30);
    let rect2 = Retangulo { altura: 50, base: 25 };

    println!("{:?}", rect1);
    println!("Área do retângulo 1 {}", rect1.area());
    println!("--------------");
    println!("{:?}", rect2);
    println!("Área do retângulo 2 {}", rect2.area());
    println!("--------------");
    println!("Retângulo 1 pode conter retângulo 2 {}", rect1.pode_conter(&rect2));
}
