use std::collections::HashMap;

// Structs
pub struct EstatisticasDescritivas
{
    dados: Vec<i32>
}

// Impls
impl EstatisticasDescritivas {
    pub fn new( dados: Vec<i32> ) -> Self { Self { dados } }

    pub fn media(&self) -> f64 {
        if self.dados.is_empty() { return 0.0; }

        let sum = self.dados.iter().map( |v| *v as f64 ).sum::<f64>();
        sum / ( self.dados.len() as f64 )
    }

    pub fn mediana(&self) -> f64 {
        if self.dados.is_empty() { return 0.0; }
        let len = self.dados.len();

        if len == 0 { return 0.0 }
        if len == 1 { return self.dados[0] as f64 }

        let mut new_vec = self.dados.iter().map( |v| *v as f64 ).collect::<Vec<f64>>();
        new_vec.sort_by( |a, b| a.partial_cmp(b).unwrap() );

        let &left = new_vec.get( (len / 2) - 1 ).unwrap();
        let &right = new_vec.get( len / 2 ).unwrap();

        if len % 2 == 0 { ( left + right ) / 2.0 }
        else { right }
    }

    pub fn moda(&self) -> Vec< i32 > {
        if self.dados.is_empty() { return Vec::new(); }

        let mut modas:Vec<i32> = Vec::new();
        let mut nums = HashMap::<i32, u16>::new(  );

        for n in self.dados.clone() {
            let num = nums.entry( n ).or_insert(0);
            *num += 1;
        }

        let &maior = nums.values().max().unwrap();
        nums.iter().for_each( |(&key, &value)| { if value == maior { modas.push(key); } } );

        modas
    }

}

// Functions
pub fn executar_estatisticas_descritivas( dados: Vec< i32 > ) -> EstatisticasDescritivas {

    let estatisticas = EstatisticasDescritivas::new( dados );

    println!( "Media: {:.2}", estatisticas.media());
    println!( "Mediana: {}", estatisticas.mediana());
    println!( "Modas: {:?}", estatisticas.moda());

    estatisticas
}