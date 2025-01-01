pub fn maior_str<'a>(strings: &'a Vec<String>) -> Option<&'a String> {
    let mut maior: Option<&String> = None;

    for s in strings {
        if let Some(m) = maior {
            maior = if m.len() < s.len() { Some(s) } else { maior }
        } else {
            maior = Some(s)
        }
    }

    maior
}
