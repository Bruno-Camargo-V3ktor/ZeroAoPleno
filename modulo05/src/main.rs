mod solutions;

use std::{fs::File, io::Read};
use solutions::{Solution, max_uber_earnings};


fn main() {

    // -- Número Mínimo de Linhas para Representar um Gráfico de Ações Mercado Financeiro

    let acoes = vec![
        vec![1, 7],
        vec![2, 6],
        vec![3, 5],
        vec![4, 4],
        vec![5, 4],
        vec![6, 3],
        vec![7, 2],
        vec![8, 1],
        vec![9, 0],
    ];
    let linhas = Solution::minimum_lines( acoes );

    println!( "Numero de linhas: {linhas}" );

    println!( );
    // ---------------------------

    // -- Tarefa: Dinheiro mínimo exigido antes das transações

    let transacoes = vec![
        vec![2, 1],
        vec![5, 0],
        vec![4, 2]
    ];

    let res = Solution::dinheiro_minimo(transacoes);

    println!( "{res}" );

    let transacoes = vec![
        vec![3, 0],
        vec![0, 3],
    ];

    let res = Solution::dinheiro_minimo(transacoes);

    println!( "{res}" );

    println!( );
    // ---------------------------


    // -- Tarefa: Ganhos Máximos do Uber

    let n = 2;
    let rides = vec![
        vec![2, 5, 4],
        vec![1, 5, 1],
    ];

    let resultado = max_uber_earnings(n, rides);
    println!( "{resultado}" );


    let n = 6;
    let rides = vec![
        vec![1, 6, 1],
        vec![3, 10, 2],
        vec![10, 12, 3],
        vec![11, 12, 2],
        vec![12, 15, 1],
        vec![13, 18, 1],
    ];

    let resultado = max_uber_earnings(n, rides);
    println!( "{resultado}" );

    println!( );
    // ---------------------------


    // -- Tarefa: Utilizando Pattern Matching para tratar diferentes tipos de entradas em Rust

    let vetor = vec![1, 2, 3, 4];
    let texto = String::from("Caralho");
    let numero = 12;

    Solution::tratar_entrada( &vetor );
    Solution::tratar_entrada( &texto );
    Solution::tratar_entrada( &numero );

    println!( );
    // ---------------------------


    // -- Tarefa: Máximo Número de Moedas Que Você Pode Obter

    let mut piles1 = vec![2, 4, 1, 2, 7, 8];
    let mut piles2 = vec![2, 4, 5];
    let mut piles3 = vec![9, 8, 7, 6, 5, 1, 2, 3, 4];

    println!( "{}", Solution::max_coins( &mut piles1 ) );
    println!( "{}", Solution::max_coins( &mut piles2 ) );
    println!( "{}", Solution::max_coins( &mut piles3 ) );

    println!( );
    // ---------------------------


    // -- Tarefa: Algoritimo de Busca Binaria

    let v = vec![1, 2, 3, 4, 5];

    assert_eq!( Solution::binary_search(&v, 2), Some(1));
    assert_eq!( Solution::binary_search(&v, 9), None);

    println!( );
    // ---------------------------


    // -- Tarefa: Algoritimo de Busca Linear

    let v = vec![1, 2, 3, 4, 5];

    assert_eq!( Solution::linear_search(&v, 2), 1);
    assert_eq!( Solution::linear_search(&v, 4),  3);
    assert_eq!( Solution::linear_search(&v, 9), -1);

    println!( );
    // ---------------------------


    // -- Algoritimo de Two Sums

    let v = vec![1, 2, 3, 4, 5];

    assert_eq!( Solution::two_sums(&v, 9), vec![3, 4] );
    assert_eq!( Solution::two_sums(&v, 3), vec![0, 1] );
    assert_eq!( Solution::two_sums(&v, 5), vec![1, 2] );

    println!( );
    // ---------------------------


    // -- Algoritimo de Ordenacao por Insercao (Insertion Sort)

    let mut vec = vec![3, 4, 2, 34, -1, 5, 7];
    println!( "Inicial: {vec:?}" );
    insertion_sort(&mut vec);
    println!( "Ordenado: {vec:?}\n" );

    // ---------------------------


    // -- Tarefa: Leitura de arquivo e ordenação de números inteiros com Bubble sort
    let path = &String::from( "./nums.txt" );
    let mut nums: Vec<_> = read_file( path ).split( ',' ).map( |s| s.parse::<i32>().expect( "Numero Invalido" ) ).collect();

    bubble_sort( &mut nums );
    println!( "{nums:?}\n" );
    // ---------------------------


    // -- Algoritimo de Bubble Sort
    let mut array: [i32;7] = [10, 23, 4, 5, 66, 7, -3];
    println!( "{array:?}" );

    for i in 0 .. array.len() {
        for j in ( i+1 .. array.len() ).rev() {
            if array[j-1] > array[j] { array.swap(j-1, j); }
        }
    }

    println!( "{array:?}" );
    // ---------------------------

}

fn insertion_sort(vec: &mut Vec<i32>) {
    for i in 1 .. vec.len() {

        'search : for j in ( 1 ..= i ).rev() {
            if vec[j - 1] > vec[j] {
                vec.swap(j-1, j);
                continue 'search;
            }

            break 'search;
        }
    }
}

fn read_file(filename: &String) -> String {
    let mut file = File::open( filename ).expect( "Arquivo nao encontrado." );

    let mut buf = String::new();
    file.read_to_string(&mut buf).expect( "Erro ao ler arquivo" );

    buf.trim().to_string()
}

fn bubble_sort( arr: &mut Vec<i32> ) {

    let size = arr.len();

    for i in 0 .. size  {
        for j in (i+1 .. size).rev() { if arr[j-1] > arr[j] { arr.swap(j-1, j); } }
    }
}
