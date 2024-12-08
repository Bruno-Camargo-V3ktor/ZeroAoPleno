use std::any::Any;

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

    pub fn minimum_lines( mut precos_acoes: Vec< Vec<i32> > ) -> i32 {
        let size = precos_acoes.len();
        if size<= 2 { return i32::max(0, ( size - 1 ) as i32 ) }


        precos_acoes.sort_unstable();
        let mut res = 1;

        for i in precos_acoes.windows(3) {

            // Calculor do Coeficiente Angular
            let esquerdo = ( i[1][1] - i[0][1] ) * ( i[2][0] - i[1][0] );
            let direito = ( i[2][1] - i[1][1] ) * ( i[1][0] - i[0][0] );

            if esquerdo != direito {
                res += 1;
            }

        }

        res
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
