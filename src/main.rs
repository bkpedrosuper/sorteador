use std::vec;
use rand::{distributions::Uniform, prelude::Distribution};
fn main() {
    let pessoas = vec![
        "Miguel",
        "Arthur",
        "Théo",
        "Heitor",
        "Gael",
        "Davi",
        "Bernardo",
        "Gabriel",
        "Ravi",
        "Noah",
        "Samuel",
        "Pedro",
        "Benício",
        "Benjamin",
        "Matheus",
        "Isaac",
        "Anthony",
        "Joaquim",
        "Lucas",
        "Lorenzo",
        "Rafael",
        "Nicolas",
        "Henrique",
        "Murilo",
        "João",
        "Lucca",
        "Guilherme",
        "Henry",
        "Bryan",
        "Gustavo",
    ];

    let seed = Uniform::from(0..pessoas.len());

    let mut rng = rand::thread_rng();

    let mut selected: usize = 999;
    for _ in 0..100 {
        selected = seed.sample(&mut rng);
        print!("{}", selected);
    }
    println!("");

    println!("O sorteado foi: {}!", pessoas[selected]);
}
