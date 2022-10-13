use std::vec;
use rand::{distributions::Uniform, prelude::Distribution};
fn main() {
    let pessoas = vec![
        "Vinícius Toni",
        "Elisandro Junior",
        "Eric Silveira",
        "Luís Bertelli",
    ];

    let sorteio_que_sorteio = Uniform::from(0..pessoas.len());

    let mut rng = rand::thread_rng();

    let mut sorteio_pau: usize = 999;
    for _ in 0..100 {
        sorteio_pau = sorteio_que_sorteio.sample(&mut rng);
        print!("{}", sorteio_pau);
    }
    println!("");

    println!("O sorteado foi: {}!", pessoas[sorteio_pau]);
}
