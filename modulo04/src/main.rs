use std::collections::HashSet;

fn main() {

    // -- Media, Mediana e Moda
    println!();

    let notas: Vec<u8> = vec![10, 5, 3, 5, 6, 7];
    println!( "Media: {}", media( &notas ) );
    println!( "Mediana: {}", mediana( &notas ) );
    println!( "Moda: {}", moda( &notas ) );

    // --------------------------------

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