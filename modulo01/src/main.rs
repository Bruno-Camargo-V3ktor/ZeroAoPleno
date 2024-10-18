
use std::io;
use std::io::{stdin, BufRead, Write};
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
        resultados[ (index-1) as usize ] =  format!("{}º Aluno - {}", index, { if media >= 6 { "Passou" } else { "Reprovou" } } );
        index += 1;
    }

    for r in resultados {  println!("{}", r); }

    // -----------------------------------------------------

    println!("\nCalcuradora Maior Divisor Comum");

    print( "Primeiro numero: " );
    let input_num1 = read_int();

    print( "Segundo numero: " );
    let input_num2 = read_int();

    let mut divisor = 1;
    let mut maior_divisor = divisor;
    loop {

        if (input_num1 < divisor ) || (input_num2 < divisor) { break; }

        if input_num1 == input_num2 {
            maior_divisor = input_num1;
            break;
        }

        if (input_num1 % divisor == 0 ) && (input_num2 % divisor == 0) { maior_divisor = divisor; }

        divisor += 1;
    }

    println!( "O maior divisor comum entre {} e {} = {}", input_num1, input_num2, maior_divisor );

    // -----------------------------------------------------

    println!( "O dobro do numero 5 eh {}", dobro(5) );
    println!( "O maior numero entre 10 e 5 eh {}", maior(10, 5) );

    // -----------------------------------------------------

    let animais = vec!["Coelho", "Gamba", "Macaco"];

    for n in 1..5 {
        println!( "{}", n )
    }

    for n in 10..=20 {
        println!( "{}", n )
    }

    for animal in animais {
        println!( "animal: {}", animal )
    }

    // -----------------------------------------------------

    let numeros = vec![10, 2, 3, 41, 34];
    let maior = encontar_maior_numero( numeros.clone() );
    println!( "maior numero: {:?}", maior );

    // -----------------------------------------------------

    println!( "2 eh primo? {}", eh_primo( 2 ) );

    // -----------------------------------------------------

    print( "Qual numero que gerar a tabuada: " );
    gerar_tabuada( read_int() as u32 );

    // -----------------------------------------------------

    let mut numero_em_string = String::new();

    print( "--> " );
    stdin().read_line( &mut numero_em_string ).expect("Falha ao ler a linha");
    println!( "Soma dos numeros pares: {}", soma_numeros_pares_em_str( &numero_em_string ) );

    // -----------------------------------------------------

    let notas: [u8; 5] = [6, 8, 3, 5, 6];
    println!( "notas: {:?} || media: {}", notas, calcular_media( &notas ) );

    // -----------------------------------------------------

    println!( "A str 'Bruno' tem caracteres unicos? {}", tem_caracteres_unicos( "Bruno" ) );
    println!( "A str 'Casa' tem caracteres unicos? {}", tem_caracteres_unicos( "Casa" ) );

    // -----------------------------------------------------


    println!( " 'abc' eh permutacao 'cab'? {}", eh_permutacao( "abc", "cab" ) );
    println!( " 'abc' eh permutacao 'def'? {}", eh_permutacao( "abc", "def" ) );

    // -----------------------------------------------------

    println!( " '121' eh palindromo ? {}", eh_palindromo( 121 ) );
    println!( " '10' eh palindromo ? {}", eh_palindromo( 10 ) );

    // -----------------------------------------------------

}

fn eh_palindromo( num: i32 ) -> bool {

    let em_texto = num.to_string();
    let invertido = em_texto.chars().rev().collect::<String>();

    num == convert_to_int( &invertido )
}

fn eh_permutacao( a: &str, b: &str ) -> bool {

    if ( a.len() <= 0 ) || ( b.len() <= 0 ) { return false; }
    if a.len() != b.len() { return false; }

    let mut historico: Vec<usize> = vec![];
    for c in a.chars() {

        let igual = {

            let mut mtc = false;

            for i in 0..b.len() {

                if ( c == b.chars().nth(i).unwrap() ) && (!historico.contains(&i)) {
                    mtc = true;
                    historico.push(i);
                    break;
                }
            }

            mtc
        };

        if !igual { return false; }

    }

    true
}

fn tem_caracteres_unicos( input: &str ) -> bool {
    let mut caracteres = vec![];

    for c in input.chars() {
        if caracteres.contains( &c ) { return false; }
        caracteres.push(c);
    }

    return true;
}

fn calcular_media( notas: &[u8] ) -> u8 {

    let mut soma = 0;

    for nota in notas {
        soma += nota;
    }

    soma / notas.len() as u8
}

fn soma_numeros_pares_em_str( text: &String ) -> i32 {
    let mut total = 0;

    for c in text.chars() {
        let num = convert_to_int( &c.to_string() );
        if ( num != 0 ) && ( num % 2 == 0 ) { total += num; }
    }

    total
}

fn gerar_tabuada( num: u32 ) {

    for n in 1..=10 {
        println!( " {} * {} = {}", num, n, num * n )
    }

}

fn eh_primo( num: u32 ) -> bool {
    if num == 1 { return false }

    for n in 2..=num {
        if ( num % n == 0 ) && (num != n) { return false }
    }

    true
}

fn encontar_maior_numero( nums: Vec< usize > ) -> Option< usize > {
    let mut maior = *nums.get( 0 ).unwrap_or_else( || { return &0 } );

    for num in nums {
        if num > maior { maior = num }
    }

    Some(maior)
}

fn dobro( num: i32 ) -> i32 { 2*num }

fn maior(a: i32, b: i32) -> i32 {
    {
        if a >= b { a }
        else { b }
    }
}

// -----------------------------------------------------

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