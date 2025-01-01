use maior_str::maior_str;

#[test]
fn test_vetor_empty() {
    let vetor_vazio: Vec<String> = Vec::new();
    assert_eq!(maior_str(&vetor_vazio), None);
}

#[test]
fn test_maior_str() {
    let vetor_vazio = vec![
        "Bruno".to_string(),
        "Camargo".to_string(),
        "Ferreira".to_string(),
    ];
    assert_eq!(maior_str(&vetor_vazio), Some(&vetor_vazio[2]));
}
