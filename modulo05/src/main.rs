use std::{fs::File, io::Read};


fn main() {

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
