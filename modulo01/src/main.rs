
use std::io;
use std::io::{BufRead, Write};
use std::ptr::read;

fn main() {
    let mut name: &str = "Bruno";
    name = "Bruno Camargo";
    println!("Hello {}", name);

    // -----------------------------------------------------

    let idade: u8 = 23;
    let altura: f32 = 1.70;
    let peso: f64 = 64.22;
    let eh_casado: bool = false;

    // -----------------------------------------------------

    let numero1 = 24;
    let numero2 = 12;

    if ( numero1 > numero2 ) {  println!("Numero1 e maior que Numero2"); }
    else { println!("Numero1 e menor que Numero2"); }


    // -----------------------------------------------------

    let mut input1 = String::new();
    let mut input2 = String::new();

    io::stdout().write(b"Digite o primeiro numero: ");
    io::stdout().flush();

    io::stdin().read_line(&mut input1).expect("Failed to read line");

    io::stdout().write(b"Digite o segundo numero: ");
    io::stdout().flush();

    io::stdin().read_line(&mut input2).expect("Failed to read line");

    let num01 = convert_to_int( &input1 );
    let num02 = convert_to_int( &input2 );

    if( num01 > num02 ) { println!( "{} maior que {}", num01, num02 ); }
    else if( num01 == num02 ) { println!( "{} igual a {}", num01, num02 ); }
    else { println!("{} menor que {}", num01, num02 ); }

    // -----------------------------------------------------

    let mut soma = 0;
    let mut valor_entrada = String::new();

    io::stdin().read_line(&mut valor_entrada);

    for num in valor_entrada.chars() {
        soma += convert_to_int( &num.to_string() );
    }

    println!("Resultado da soma: {}", soma);

    // -----------------------------------------------------

    let mut entrada_fatorial = String::new();
    let mut fatorial = 1;

    io::stdin().read_line(&mut entrada_fatorial).expect("Falha ao ler a linha");

    let mut calc_fatorial = convert_to_int( &entrada_fatorial );
    loop {
        if calc_fatorial <= 1 { break }

        fatorial *= calc_fatorial;
        calc_fatorial -= 1;
    }

    println!("O Fatorial de '{}' e {}", convert_to_int( &entrada_fatorial ), fatorial );

    // -----------------------------------------------------

    print( "Entre com a quantidade de alunos: " );
    let qtd_alunos = read_int();
    let mut resultados = [String::new(), String::new(), String::new()];

    let mut index = 1;
    while index <= qtd_alunos {
        let media = read_int();
        resultados[index as usize] =  format!("{}º Aluno - {}", index, { if media >= 6 { "Passou" } else { "Reprovou" } } );
        index += 1;
    }

    for r in resultados {  println!("{}", r); }

    // -----------------------------------------------------

}

fn convert_to_int( data_input: &String ) -> i32 {
    data_input
        .trim()
        .parse::<i32>()
        .unwrap_or_else( | _ | { 0 } )
}

fn read_int() -> i32 {
    let stdin = io::stdin();
    let mut line = String::new();

    stdin.lock().read_line( &mut line ).expect("Falha ao ler a linha");
    convert_to_int( &line )
}

fn print( text: &str ) {
    let mut stdout = io::stdout();

    stdout.write( text.as_bytes() );
    stdout.flush();
}