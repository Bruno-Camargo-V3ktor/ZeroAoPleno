use std::{any::Any, collections::HashMap, collections::HashSet, fs::File, path::Path};

use std::io::prelude::*;
use std::io::{self, Read, Write, BufReader, BufWriter};

// Structs

pub struct Solution {}

#[ derive(Debug, PartialEq) ]
pub struct Passanger {
    start: i32,
    end: i32,
    earn: f64
}


// Implements

impl Passanger {

    pub fn new(start: i32, end: i32, tip: i32) -> Self {
        Self { start, end, earn: (end - start + tip) as f64  }
    }
}

impl Eq for Passanger {}

impl  PartialOrd for Passanger {

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.end.partial_cmp( &other.end )
    }
}

impl Ord for Passanger {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.end.cmp( &other.end )
    }
}

impl Solution {

    pub fn two_sums( vec: &Vec<i32>, target: i32 ) -> Vec<i32> {

        /* for (i, &v1) in vec.iter().enumerate() {

            for (j, &v2) in vec.iter().skip(i + 1).enumerate() {
                let j = j + i + 1;
                if v1 + v2 == target { return vec![i as i32, j as i32] }
            }
        } */

        use std::collections::HashMap;

        let mut map: HashMap<i32, i32> = HashMap::new();

        for i in 0 .. vec.len() {

            let v = vec[i];
            match map.get( &(target - v) ) {
                Some( &n ) => { return vec![ n, i as i32 ] }
                None => { map.entry( v ).or_insert( i as i32 ); }
            }

        }

        vec![]
    }

    pub fn linear_search( vec: &Vec<i32>, target: i32 ) -> i32 {

        for i in 0 .. vec.len() {
            if vec[i] == target { return i as i32; }
        }

        -1
    }

    pub fn binary_search( vec: &Vec<i32>, target: i32 ) -> Option<usize> {

        let mut head = 0;
        let mut tail = vec.len() - 1;

        while head <= tail {
            let mid = (head + tail) / 2;

            if vec[mid] == target { return Some(mid); }
            else if vec[mid] < target { head = mid + 1; }
            else { tail = mid - 1; }
        }

        None
    }

    pub fn max_coins(piles: &mut Vec<i32>) -> i32 {

        piles.sort_unstable();
        let numbers = piles.len();

        piles.iter()
            .rev()
            .take( (numbers*2) / 3  )
            .skip(1)
            .step_by(2)
            .sum::<i32>()
    }

    pub fn tratar_entrada( valor: &dyn Any ) {

        if let Some(val) = valor.downcast_ref::<i32>() {
            println!("É um i32: {}", val * 2);
        }

        else if let Some(val) = valor.downcast_ref::<String>() {
            println!("É uma String: {}", val.len());
        }

        else if let Some(val) = valor.downcast_ref::< Vec<i32> >() {
            println!("É um Vetor de inteiros: {}", val.len());
        }

        else {
            println!("Tipo de dado nao suportado");
        }
    }

    pub fn dinheiro_minimo(transacoes: Vec< Vec<i32> > ) -> i64 {
        let mut gastos_liquidos = 0;

        for t in &transacoes {
            gastos_liquidos += i32::max(0, t[0] - t[1]) as i64;
        }

        let mut dinheiro_minimo = gastos_liquidos;
        for t in &transacoes {
            dinheiro_minimo = i64::max(dinheiro_minimo, gastos_liquidos + ( i32::min(t[0], t[1]) ) as i64 );
        }

        dinheiro_minimo
    }

    pub fn minimum_lines( mut prices: Vec< Vec<i32> > ) -> i32 {
        let size = prices.len();
        if size<= 2 { return i32::max(0, ( size - 1 ) as i32 ) }


        prices.sort_unstable();
        let mut res = 1;

        for i in prices.windows(3) {

            // Calculor do Coeficiente Angular
            let left_eq = ( i[1][1] - i[0][1] ) * ( i[2][0] - i[1][0] );
            let right_eq = ( i[2][1] - i[1][1] ) * ( i[1][0] - i[0][0] );

            if left_eq != right_eq {
                res += 1;
            }

        }

        res
    }

    pub fn players_with_zero_or_one_loss( matches: Vec< Vec<i32> > ) -> Vec< Vec<i32> > {

        let mut losses = HashMap::<i32, i32>::new();
        let mut answer = vec![ vec![], vec![] ];

        matches.iter().for_each( |m| {
            losses.entry(m[0]).or_insert(0); //Vencedor

            let player = losses.entry( m[1] ).or_insert(0); // Perderdor
            *player += 1;
        } );

        let mut values: Vec<_> = losses.iter().map( |(k, v)| (k, v) ).collect();
        values.sort_unstable();

        for (k, v) in values {
            if *v <= 1 { answer[*v as usize].push( *k ); }
        }

        answer
    }

    pub fn rearrange_number( num: i32 ) -> i32 {

        let mut digits: Vec<char> = num.abs().to_string().chars().collect();
        let signal = num.signum();

        if signal >= 0 {
            digits.sort();
            if let Some(pos) = digits.iter().position( |x| *x != '0' ) {
                digits.swap(0, pos);
            }
        }

        else {
            digits.sort_by( |a, b| b.cmp(a) );
        }

        let new_number: String = digits.iter().collect();
        let res = ( new_number.parse::<i32>().unwrap() ) * signal;

        res
    }

    pub fn melhores_alunos( feedback_positivo: Vec<&str>, feedback_negativo: Vec<&str>, relatorio: Vec<&str>, id_aluno: Vec<i32>, k: i32  ) -> Vec<i32> {

        let conjunto_positivo: HashSet<&str> = feedback_positivo.iter().map( |s| *s ).collect();
        let conjunto_negativo: HashSet<&str> = feedback_negativo.iter().map( |s| *s ).collect();
        let mut pontuacoes = HashMap::<i32, i32>::new();

        for (i, &rel) in relatorio.iter().enumerate() {
            let aluno = pontuacoes.entry( id_aluno[i] ).or_insert(0);

            for palavra in rel.split_whitespace() {
                if conjunto_positivo.contains( palavra ) { *aluno += 3; }
                else if conjunto_negativo.contains( palavra ) { *aluno -= 1; }
            }
        }

        let mut alunos: Vec<(i32, i32)> = pontuacoes.into_iter().collect();
        alunos.sort_by( |a, b| {
            if a.1 == b.1 { a.0.cmp( &b.0 ) }
            else { b.1.cmp( &a.1 ) }
        } );

        alunos.into_iter().take(k as usize).map( |(id, _)| id ).collect()
    }

    pub fn ler_arquivo( filename: &str ) -> io::Result< Vec<i32> > {
        let path = Path::new( filename );
        let file = File::open( &path )?;

        let mut reader = BufReader::new( file );
        let numeros = reader
            .lines()
            .filter_map( |line| line.ok()?.parse().ok() )
            .collect::< Vec<i32> >();

        Ok( numeros )
    }

    pub fn salvar_arquivo( numeros: &Vec<i32>, filename: &str ) -> io::Result<()> {
        let path = Path::new( filename );
        let file = File::create( &path )?;

        let mut writer = BufWriter::new( file );
        for &numero in numeros { writeln!( writer, "{numero}" )? }

        Ok(())
    }

    pub fn bubble_sort( arr: &mut Vec<i32> ) {

        let size = arr.len();

        for i in 0 .. size  {
            for j in (i+1 .. size).rev() { if arr[j-1] > arr[j] { arr.swap(j-1, j); } }
        }
    }

}

// Functions

pub fn max_uber_earnings(n: i32, rides: Vec< Vec<i32> >) -> i64 {

    let mut passangers: Vec<Passanger> = rides
        .iter().map( | v | Passanger::new(v[0], v[1], v[2]) ).collect();

    passangers.sort_unstable();
    let mut dp = vec![0; passangers.len() + 1];

    for i in 0 .. passangers.len() {
        let j = passangers.partition_point( |p| p.end <= passangers[i].start );
        dp[i + 1] = dp[i].max( dp[j] + (passangers[i].earn as i32) );
    }

    *dp.last().unwrap() as i64
}
