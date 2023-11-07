/**
 * Dada uma lista de inteiros, use um vetor e retorne a média,
 * a mediana (quando classificado, o valor na posição do meio)
 * e modo (o valor que ocorre com mais frequência; um hash map
 * será útil aqui) da lista.
 */

use rand::{thread_rng, Rng};
use std::collections::HashMap;

fn main() {
    let mut v: Vec<u32> = Vec::new();

    v.push(thread_rng().gen_range(1..=10));
    v.push(thread_rng().gen_range(1..=10));
    v.push(thread_rng().gen_range(1..=10));
    v.push(thread_rng().gen_range(1..=10));
    v.push(thread_rng().gen_range(1..=10));
    
    v.sort();

    let soma: u32 = v.iter().sum();
    let soma: f32 = soma as f32;

    let media: f32 = soma / (v.len() as f32);
    println!("{:?}", media);

    let posicao_meio: usize = ((v.len() as f32) / 2.0 - 1.0).round() as usize;
    println!("{:?}", v.get(posicao_meio));

    let mut map = HashMap::new();

    for valor in v {
        let count = map.entry(valor).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let key_with_max_value = map.iter().max_by_key(|entry| entry.1).unwrap();
    println!("{:?}", key_with_max_value);
}
