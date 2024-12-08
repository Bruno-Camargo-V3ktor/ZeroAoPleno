/*
use rocket::{ launch };
use rocket::fs::FileServer;


#[ launch ]
fn start() -> _ {
    rocket::build()
        .mount("/public", FileServer::from("static_files"))
        // Montando um Roteador de usando um FileServer, que ira criar "rotas" para todos os arquivos com base em um diretorio
}
*/

// --------------------------------------------------------------

use std::path::{Path, PathBuf};
// Importando as as Structs Path e PathBuf
    // Ambas sao Struct para lida com diretorios de arquivos
    // A Path representa um diretorio imutavel, ja a PathBuf, representa
    // um diretorio mutavel

use rocket::{ launch, routes, get };

// Do modulo de FileSystem do rocket (fs)
    // Estamos pegando a Struct NamedFile, que e capaz de pegar arquivos em nosso servidor,
    // atravez de um Path e devolver como resposta
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;

// Utilizando um parametro do tipo <parametro..> (Multiplos Seguimentos / Multiple Segments) tudo que vie apos dele mesmo
    //Em teoria ja sendo outra rota como por exemplo: /page/public/text.txt
    // toda parte "/public/text.txt" sera capturado como um parametro
    // Por isso que depois de definido esse tipo de parametro nao pode ser colocado
    // mas nenhum na rota
#[ get( "/page/<path..>" ) ]
async fn get_page( path: PathBuf ) -> Result<NamedFile, NotFound<String>> {
    // A funcao precisar ser assincrona pos a varredura e leitura do arquivo por demora

    let base_path = Path::new( "static_files" );
    let file_path = base_path.join( path );

    NamedFile::open(file_path) // Usando o NamedFile para abrir um arquivo (funcao assincrona)
        .await.map_err( |_| NotFound("File not found".to_string()) ) // Esperando o resultado, e mapeando o error do mesmo
}

#[ launch ]
fn start() -> _ {
    rocket::build()
        .mount("/", routes![ get_page ]) // Montando um Roteador
}
