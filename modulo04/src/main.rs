mod it;
mod viagens;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use modulo04::executar_estatisticas_descritivas;
use it::Router;
use it::Network;
use viagens::{add_passageiro, add_voo, exibir_passageiros, exibir_voos, Passageiro, Voo};


struct User {
    name: String,
    age: u8,
    friends: Vec< String >,
}

// -> Batalha Narval
const BOARD_SIZE: usize = 10;

struct Ship {
    x: usize,
    y: usize,
    length: usize,
    direction: Direction
}

enum Direction {
    Horizontal,
    Vertical
}

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


    // -- Departamentos
    println!();

    departamento();

    // --------------------------------


    // -- Tarefa: Rede Socias Simples

    println!();

    let mut users: HashMap<String, User> = HashMap::new();

    loop {

        println!( "\n -- REDE SOCIAL MENU -- " );
        println!( "1 -> Adicionar Usuário");
        println!( "2 -> Adicionar Amigo");
        println!( "3 -> Ver Lista De Amigos");
        println!( "4 -> Sair");

        let mut op = String::new();
        stdin().read_line( &mut op ).expect( "Failed to read line!" );
        let choose = op.trim().parse().unwrap();

        match choose {
            1 => {
                println!("\nNome do novo Usuario: ");
                let mut name = String::new();
                stdin().read_line( &mut name ).expect( "Failed to read line!" );

                println!("\nIdade do novo Usuario: ");
                let mut age = String::new();
                stdin().read_line( &mut age ).expect( "Failed to read line!" );
                let age: u8 = age.trim().parse().unwrap();


                let user = User {  name: name.clone() , age, friends: Vec::new() };
                users.insert( name, user );
            },

            2 => {

                println!("\nNome do Usuario: ");
                let mut user_name = String::new();
                stdin().read_line( &mut user_name ).expect( "Failed to read line!" );

                println!("\nNome do Amigo");
                let mut friend_name = String::new();
                stdin().read_line( &mut friend_name ).expect( "Failed to read line!" );


                let friend_option = users.remove( &friend_name );
                let user_option = users.get_mut( &user_name );

                match ( user_option, &friend_option ) {
                    ( Some(user), Some(friend) ) => {
                        user.friends.push( friend.name.clone() );
                    }

                    _ => { println!( "Usuario ou Amigo nao encontrado!!!" ); }
                }

                match friend_option {
                    Some(user) => { users.insert( user.name.clone(), user ); },
                    _ => {}
                }

            },

            3 => {
                println!("\nNome do Usuario: ");
                let mut user_name = String::new();
                stdin().read_line( &mut user_name ).expect( "Failed to read line!" );

                match users.get_mut( &user_name ) {
                    Some(user) => {
                        for (pos, friend) in user.friends.iter().enumerate() {
                           println!( "{} - {}", pos, friend );
                        }
                    }
                    None => { println!( "Usuario nao encontrado!!!" ) }
                }

            },

            4 => { break; },

            _ => { break; }
        }

    }

    // --------------------------------


    // -- Tarefa: Implementação de uma trait de rede com Rust
    println!();

    let router = Router::new( "192.133.3.1" );

    let is_alive = router.ping( "google.com" );
    let trace = router.traceroute("google.com" );
    let ip = router.nslookup( "google.com" );

    println!("O host está online: {}", is_alive);
    println!("Trace de rota: {:?}", trace);
    println!("Endereço IP: {}", ip);

    // --------------------------------


    // -- Tarefa: Companhia De Viagens
    println!();

    let mut passageiros: Vec<Passageiro> = Vec::new();
    let mut voos: Vec<Voo> = Vec::new();

    // Adicione alguns passageiros
    add_passageiro(&mut passageiros, "João", "123ABC", 18);
    add_passageiro(&mut passageiros, "Maria", "456DEF", 21);
    add_passageiro(&mut passageiros, "Pedro", "789GHI", 25);

    // Adicione alguns vôos
    add_voo(&mut voos, "Voo101", "São Paulo", "Rio de Janeiro", "30/06/2020", "09:00");
    add_voo(&mut voos, "Voo102", "Rio de Janeiro", "São Paulo", "01/07/2020", "09:00");

    // Exiba os passageiros
    exibir_passageiros(&passageiros);

    // Exiba os vôos
    exibir_voos(&voos);

    // --------------------------------


    // -- Tarefa: Caça-Palavra
    println!();

    let board = [
        ['A', 'B', 'E', 'E'],
        ['S', 'F', 'C', 'S'],
        ['A', 'B', 'C', 'D']
    ];
    let word = "SEE";


    let board = [
        ['I', 'H', 'G', 'F'],
        ['J', 'K', 'F', 'E'],
        ['A', 'B', 'C', 'D']
    ];
    let word = "ABCDEFGHIJK";

    let contain = search_word_in_board( &word, &board );
    println!( "A palavra {}, contem no tabuleiro? {}", word, contain );


    // --------------------------------


    // -- Tarefa: Numero de Rainhas
    println!();

    println!( "{:?}", n_queens( 2 ) );

    // --------------------------------

    // -- Tarefa: Batalha Narval
    println!();

    let mut board = [ ['_'; BOARD_SIZE]; BOARD_SIZE ];
    let ships = vec![
        Ship{ x: 2, y: 4, length: 2, direction: Direction::Horizontal },
        Ship{ x: 4, y: 5, length: 3, direction: Direction::Vertical },
        Ship{ x: 8, y: 8, length: 1, direction: Direction::Horizontal }
    ];

    for ship in ships.iter() {
        for i in 0 .. ship.length {

            let(x, y) = match ship.direction {
                Direction::Horizontal => { (ship.x + i, ship.y) }
                Direction::Vertical => { (ship.x, ship.y + i) }
            };

            board[x][y] = 'S';

        }
    }

    println!( "Bem-Vindo a Batalha Naval!" );
    println!( "Acerte todos os navios antes que acabe os seus tiros" );

    let mut shorts = BOARD_SIZE*BOARD_SIZE;
    let mut ships_left = ships.len();

    while shorts > 0 && ships_left > 0 {
        println!( "---------------------------------------------------" );
        for l in board.iter() { println!( "{:?}", l ); }
        println!( "---------------------------------------------------" );

        println!( "Voce tem {shorts} tiros restantes" );
        println!( "Digite as cordenas (Linha Coluna) para atirar..." );

        let mut buffer = String::new();
        stdin().read_line( &mut buffer ).expect( "Error ao ler entrada" );

        let guess: Vec<usize> = buffer.split_whitespace().map( |s| s.parse::<usize>().unwrap() ).collect();

        let (x, y) = (guess[0] - 1, guess[1] - 1);
        if x >= BOARD_SIZE || y >= BOARD_SIZE {
            println!( "Coordenas fora do tabuleiro, tente novamente..." );
            continue
        }

        if board[x][y] == 'S' {
            println!( "Voce acertou um navio!" );
            board[x][y] = 'X';
            ships_left -= 1;
        }
        else {
            println!( "Voce Errou" );
            board[x][y] = 'O';
        }
        shorts -= 1;
    }

    if ships_left == 0 { println!( "Voce ganhou no jogo :)" ) }
    else { println!( "Voce perdeu no jogo :(" ) }

    // --------------------------------

}

fn n_queens(size: usize) -> Vec< Vec<String> > {
    let mut result: Vec< Vec<String> > = Vec::new();
    let mut board = vec![ vec!['.'; size]; size ];

    backtrack_board(&mut result, &mut board, size, 0);

    result
}

fn backtrack_board( result: &mut Vec< Vec< String > >, board: &mut Vec< Vec<char> >, size: usize, line: usize ) {

    if line == size || (size == 2 && line == size-1) {
        result.push(
            board.iter().map( |l| l.iter().collect::<String>() )
                .collect()
        );
        return;
    }

    for col in 0 .. size {
       if is_safe( board, line, col) {
           board[line][col] = 'Q';
           backtrack_board(result, board, size, line + 1);

           board[line][col] = '.';
       }
    }

}

fn is_safe( tabuleiro: &Vec<Vec<char>>, linha: usize, coluna: usize ) -> bool {

    let size = tabuleiro.len() as i32;

    // Verifica se ha rainhas na mesma coluna
    for i in 0 .. linha {
        if tabuleiro[i][coluna] == 'Q' { return false }
    }

    // Verifica se ha rainhas na diagonal esquerda (cima)
    let mut i = linha as i32 -1;
    let mut j = coluna as i32 - 1;
    while i >= 0 && j >= 0 {
        if tabuleiro[i as usize][j as usize] == 'Q' { return false }
        i -= 1;
        j -= 1;
    }

    // Verifica se ha rainhas na diagonal direita (cima)
    let mut i = linha as i32 -1;
    let mut j = coluna as i32 + 1;
    while i >= 0 && j <= size-1 {
        if tabuleiro[i as usize][j as usize] == 'Q' { return false }
        i -= 1;
        j += 1;
    }

    true
}

fn search_word_in_board( word: &str, board: &[ [char; 4]; 3 ] ) -> bool {

    let l = board.len();
    let c = board[0].len();

    for i in 0..l {
        for j in 0..c {

            let current_char = board[i][j];
            let mut chars = word.chars().collect::<VecDeque<char>>();

            if current_char == chars.pop_front().unwrap() {

                let mut historic: HashSet< (usize, usize) > = HashSet::new();
                let mut check: Vec< (usize, usize) > = Vec::new();

                historic.insert( (i, j) );
                check.push( (i, j) );

                while !check.is_empty() {

                    let mut results: Vec< Option<_> > = Vec::new();
                    let target_char = {
                        match chars.pop_front() {
                            Some(c) => c,
                            None => return true
                        }
                    };

                    for (x, y) in check.iter() {
                        results.push( search_nearby_board( target_char, board, (*x as i32, *y as i32) ) );
                    }

                    check.clear();
                    for r in results {
                        match r {
                            Some( positions ) => {
                                for (x, y) in positions {
                                    if historic.contains( &(x, y) ) { continue; }

                                    check.push( (x, y) );
                                    historic.insert( (i, y) );
                                }
                            }
                            None => {}
                        }
                    }

                }

            }

        }
    }

    false
}

fn search_nearby_board( char: char, board: &[ [char; 4]; 3 ], pos: (i32, i32) ) -> Option< Vec< (usize, usize) > > {


    let mut positions = Vec::new();

    let mut add = |linha: i32, coluna: i32| {
        let l = linha as usize;
        let c = coluna as usize;

        if !(l < 0 || l >= board.len()) && !(c < 0 || c >= board[0].len()) {
            if board[l][c] == char { positions.push((l, c)); }
        }
    };

    { // Direita
        let linha = pos.0;
        let coluna = pos.1 + 1;
        add(linha, coluna);
    }

    { // Esquerda
        let linha = pos.0;
        let coluna = pos.1 - 1;
        add(linha, coluna);
    }

    { // Cima
        let linha = pos.0 - 1;
        let coluna = pos.1;
        add(linha, coluna);
    }

    { // Baixo
        let linha = pos.0 + 1;
        let coluna = pos.1;
        add(linha, coluna);
    }

    // Diagonais
    { // Direita + Baixo
        let linha = pos.0 + 1;
        let coluna = pos.1 + 1;
        add(linha, coluna);
    }

    { // Direita + Cima
        let linha = pos.0 - 1;
        let coluna = pos.1 + 1;
        add(linha, coluna);
    }

    { // Esquerda + Baixo
        let linha = pos.0 + 1;
        let coluna = pos.1 - 1;
        add(linha, coluna);
    }

    { // Esquerda + Cima
        let linha = pos.0 - 1;
        let coluna = pos.1 - 1;
        add(linha, coluna);
    }

    if positions.is_empty() { None }
    else { Some( positions ) }
}

fn departamento() {

    let mut departamentos: HashMap< String, Vec<String> > = HashMap::new();


    loop {

        println!( "Digite o comando do tipo: Adicione <Pessoa> para <Departamento>" );
        let mut comando = String::new();

        stdin().read_line( &mut comando ).expect("Erro ao ler entrada");
        let comando = comando.trim();

        let mut token_comando = comando.split_whitespace();

        let pessoa = match token_comando.nth(1) {
            Some( pessoa ) => pessoa,
            None => { println!("O Comando digitado não é valido"); break; }
        };

        let departamento =  match token_comando.last() {
            Some( departamento ) => departamento,
            None => { println!("O Comando digitado não é valido"); break; }
        };

        let empregados = departamentos.entry( departamento.to_string() ).or_insert( Vec::new() );
        empregados.push( pessoa.to_string() );

        println!( "O {} foi adicionado ao departamento {}", pessoa, departamento );
    }
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
