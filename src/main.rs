use std::cmp::Ordering;

#[derive(PartialOrd, Eq, Clone, Copy)]
struct Jogador {
    nome: &'static str,
    nivel: u8,
    sexo: char,
    partidas_ganhas: u8,
    partinhas_jogadas: u8,
}

impl PartialEq for Jogador {
    fn eq(&self, other: &Jogador) -> bool {
        self.sexo == other.sexo
    }
}

impl Ord for Jogador {
    fn cmp(&self, other: &Jogador) -> Ordering {
        (self.nivel).cmp(&(other.nivel))
    }
}

fn main() {
    let p1 = Jogador {
        nome: "Francisco",
        nivel: 1,
        sexo: 'M',
        partidas_ganhas: 0,
        partinhas_jogadas: 0,
    };

    let p2 = Jogador {
        nome: "Bruno",
        nivel: 1,
        sexo: 'M',
        partidas_ganhas: 0,
        partinhas_jogadas: 0,
    };

    if p1 > p2 {
        println!("{} é de nível maior que {}.", p1.nome, p2.nome);
    } else {
        print!("{} é de nível maior que {}.", p2.nome, p1.nome);
    }

    print!("---------/---------");

    if p1 == p2 {
        println!("{} e {} são do mesmo sexo.", p1.nome, p2.nome);
    } else {
        print!("{} e {} são de sexo opostos.", p1.nome, p2.nome);
    }
}
