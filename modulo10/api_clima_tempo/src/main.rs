use std::collections::HashMap;

use rocket::{ State, launch, routes, get };
use rocket::response::content;

// O modulo contente possui varios Struct e metodos para utilizamos na hora de definimos
    // o conteudo de um respota de um endpoint, como por exemplo a strcut RawHtml<T>
    // onde definimos que a resposta vai ser um HTML, ele e de tipo Generico T
    // pos podemos escolhe como vamos escrever o conteudo dele, mas normalmente eh com Strings

// O State e um 'container de referencia' que podemos usar para guarda referencias de valores
    // que queremos acessar posteriomente, normalmente em nossos endpoints
    // onde definimos um parametro que ira recener o State



//#[ get("/") ]
//fn teste() -> String { "Teste".to_string() }


// AUXILIAR...

struct Local {
    temperatura: f64,
    cidade: String
}

fn obter_temperatura(local: &Local) -> String {
    local.temperatura.to_string()
}


// ENDPOINTS...


//#[ get("/temperatura/<cidade>") ]
// fn temperatura( cidade: String, local_state: &State<Local> ) -> content::RawHtml<String> {
//    let local = local_state.inner();
//    if local.cidade == cidade {
//        content::RawHtml(
//            format!("A temperatura da ciade: {} eh {} C", local.cidade, obter_temperatura(&local) )
//        )
//    }
//    else {
//        content::RawHtml( format!( "Cidade Desconhecida" ) )
//    }
//}

#[ get( "/temperatura/<cidade>" ) ]
fn temperatura(cidade: String, citys_state: &State< HashMap<String, Local> >) -> content::RawHtml<String> {
    let citys = citys_state.inner();

    match citys.get( &cidade ) {

        Some(l) => {
            content::RawHtml(
                format!("<p>A temperatura da ciade: {} eh {} C</p>", l.cidade, obter_temperatura(&l) )
            )
        }

        None => {  content::RawHtml( format!( "<p>Cidade Desconhecida</p>" ) ) }
    }

}

// LAUNCH...

#[ launch ]
fn start() -> _ {

    let mut citys_state = HashMap::<String, Local>::new();
    citys_state.insert("Taboao Da Serra".to_string(), Local{ temperatura: 20.0, cidade: "Taboao Da Serra".to_string() } );
    citys_state.insert("Embu Das Artes".to_string(), Local{ temperatura: 27.0, cidade: "Embu Das Artes".to_string() } );
    citys_state.insert("Rio De Janeiro".to_string(), Local{ temperatura: 35.0, cidade: "Rio De Janeiro".to_string() } );


    rocket::build()
        .mount("/", routes![temperatura])
        .manage( citys_state )
        //.manage( Local{ temperatura: 20.0, cidade: "Taboao Da Serra".to_string() }  )

        // Cria um State dentro do Build/Contexto da aplicao com o valor passado
}
