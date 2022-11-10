use std::io;

fn main(){
    println!("Olá!, tudo bem?");

    println!("Digite um numero abaixo: ");

    let mut tentativa = String::new();

    io::stdin() //input
        .read_line(&mut tentativa)
        .expect("falhou");

    println!("você digitou: {tentativa}");

}