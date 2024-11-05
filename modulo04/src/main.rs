use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use modulo04::executar_estatisticas_descritivas;

fn main() {

    // -- Media, Mediana e Moda
    println!();

    let notas: Vec<u8> = vec![10, 5, 3, 5, 6, 7];
    println!( "Media: {}", media( &notas ) );
    println!( "Mediana: {}", mediana( &notas ) );
    println!( "Moda: {}", moda( &notas ) );

    // --------------------------------

    // -- Tarefa: Estatísticas Descritivas com Structs
    println!();

    let estatistica =
        executar_estatisticas_descritivas( ler_numeros( "dados.txt".to_string() ) );

    // --------------------------------

    // -- Algoritimo Pig-Latin
    println!();

    let palavra1 = String::from( "amor" );
    let palavra2 = String::from( "time" );

    println!( "{}", convert_to_pig_latin( &palavra1 ) );
    println!( "{}", convert_to_pig_latin( & palavra2 ) );

    // --------------------------------

}

fn convert_to_pig_latin( word: &String ) -> String {

    let vogais = ['a', 'e', 'i', 'o', 'u'];
    let mut new_word = word.clone();
    let first_char = word.chars().next().unwrap();

    if vogais.contains( &first_char ) { new_word.push_str( "-hay" ); }
    else {
        new_word.remove(0);
        new_word.push_str( format!("-{}ay", first_char).as_str() );
    }


    new_word
}

fn ler_numeros( path: String ) -> Vec<i32> {

    let mut file = File::open( path ).expect( "file not found" );
    let mut buffer = BufReader::new( &file );
    let mut nums: Vec<i32> = Vec::new();

    for linha in buffer.lines() {
        let value = linha.unwrap().parse::<i32>().unwrap();
        nums.push(value);
    }

    nums
}

fn moda( vec: &Vec<u8> ) -> u8 {

    let mut moda: u8 = 0;
    let nums = vec.iter().map( |v| *v ).collect::<HashSet<u8>>();

    for &n in nums.iter() {
        let sum: u8 = vec.iter().map(|v| { if *v == n { 1 } else { 0 } }).sum();
        if (sum > moda) && (sum > 1) { moda = n }
    }

    moda
}

fn mediana( vec: &Vec<u8> ) -> f32 {
    let len = vec.len();

    if len == 0 { return 0.0 }
    if len == 1 { return vec[0] as f32 }

    let mut new_vec = vec.clone();
    new_vec.sort();

    let left: u8 = new_vec[ (len / 2) - 1 ];
    let right: u8 = new_vec[ len / 2 ];

    if len % 2 == 0 {
        media( &vec![left, right] )
        // ( (left + right) as f32 ) / 2.0
    }
    else { right as f32 }
}

fn media( vec: &Vec<u8> ) -> f32 {
    let soma: u8 = vec.iter().sum();
    ( soma as f32 ) / ( vec.len() as f32 )
}