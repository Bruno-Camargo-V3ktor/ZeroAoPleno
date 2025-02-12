use media_type::{Media, Movie, Serie};

#[test]
fn test_playing_in_movie() {
    let movie: Box<dyn Media> = Box::new(Movie::new("The Batman", 120));

    assert_eq!(
        movie.reproduce(),
        String::from("Playing the movie 'The Batman' which is 120 minutes long")
    )
}

#[test]
fn test_playing_in_serie() {
    let serie: Box<dyn Media> = Box::new(Serie::new("The Office", 8));

    assert_eq!(
        serie.reproduce(),
        String::from("Playing the series 'The Office' that has 8 seasons")
    )
}
