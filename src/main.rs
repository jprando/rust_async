use futures::executor::block_on;
use std::{thread, time::Duration};

fn main() {
    println!("main: inicio app");

    {
        let _future = teste();
        println!("main: apos executar -> async fn teste");

        let _future_valor = block_on(_future);
        println!("\nmain: retorno de -> async fn teste -> {}", _future_valor);
    };

    println!("main: fim app");
}

async fn teste<'async_teste>() -> &'async_teste str {
    println!("\n# async fn teste");
    for i in 1..6 {
        thread::sleep(Duration::from_secs(1));
        println!("#...{}", i);
    }
    const VALUE: &str = "funcionou!!!";
    println!("# retornando {}", VALUE);
    VALUE
}
