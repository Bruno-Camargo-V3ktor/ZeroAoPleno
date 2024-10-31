use std::fs::{File, OpenOptions};
use std::io;
use std::io::BufReader;
use std::io::prelude::*;


struct Pessoa {
    nome: String,
    idade: u8
}

trait Voz {
    fn falar(&self);

    fn tem_voz(&self) -> bool;
}

impl Pessoa {
    fn new (nome: String, idade: u8) -> Pessoa {
        Self { nome, idade }
    }
}

impl Voz for Pessoa {

    fn falar(&self) {
        if self.tem_voz() { println!("Ola meu nome eh {}", self.nome) };
    }

    fn tem_voz(&self) -> bool { self.idade > 2 }
}


fn main() {

    // -- Leitura de Arquivos

    let mut arquivo = File::open("rust_wiki.txt").expect("file not found");
    let mut conteudo = String::new();
    arquivo.read_to_string(&mut conteudo).expect("something went wrong reading the file");

    println!("{}", conteudo);

    // ----------------------


    // -- Criando e Escrevendo em um Arquivo

    let mut arquivo = File::create( "socorro.txt" ).expect("error creating file");
    let conteudo = String::from( "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA" );
    arquivo.write_all( conteudo.as_bytes() ).expect("error writing file");

    // ----------------------


    // -- Tarefa: Trabalhando com arquivos em Rust

    println!("\n");

    let file = File::open("socorro.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap();
        println!("{}", l);
    }

    let mut file = OpenOptions::new()
        .append(true)
        .open("socorro.txt")
        .unwrap();
    let novo_conteudo = "\nNova linha de texto";
    file.write_all( novo_conteudo.as_bytes() ).unwrap();

    let file = File::open("socorro.txt").unwrap();
    qtd_linas_arquivo( &file );

    // ----------------------


    // -- Definindo Traits

    println!("\n");

    let pessoa = Pessoa::new( String::from("Thomas"), 12 );
    println!( "{} pode falar? {}", pessoa.nome, pessoa.tem_voz() );
    pessoa.falar();

    // ----------------------


    // -- Pattern Matching

    println!("\n");

    let numero = 2;
    match numero {
        1 => { println!("O numero eh 1") },
        2 | 3 => println!("O numero eh 2 ou 3"),
        5..10 => println!("O numero esta entre 5 a 9"),
        10..=20 => println!("O numero esta entre 10 a 20"),
        _ => println!("Não sei que numero eh"),
    }

    let nome = "Rodrigo";
    match nome {
        "Joao" => println!("Joao eh dentista"),
        "Rodrigo" => println!("Rodrigo eh programador"),
        _ => println!("não sei sua profissão")
    }

    // ----------------------


    // -- Input de dados no Match

    println!("\n");

    let mut mensagen_usuario = String::new();
    println!("Digite algo? \n");
    
    match io::stdin().read_line(&mut mensagen_usuario) {
        Ok( _ ) => { println!("\nSucesso. voce digitado: {}", mensagen_usuario.to_uppercase()); }
        Err(error) => println!( "error: {}", error )
    }



    // ----------------------


}

fn qtd_linas_arquivo( arquivo: &File ) -> usize {
    let reader = BufReader::new(arquivo);
    let num_lines = reader.lines().count();
    println!("Número de linhas: {}", num_lines);
    num_lines
}