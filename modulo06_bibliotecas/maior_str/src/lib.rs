pub struct Par<'a, T> {
    primeiro: &'a T,
    segundo: &'a T,
}

impl<'a, T> Par<'a, T> {
    pub fn novo(primeiro: &'a T, segundo: &'a T) -> Self {
        Self { primeiro, segundo }
    }

    pub fn primeiro(&self) -> &'a T {
        self.primeiro
    }

    pub fn segundo(&self) -> &'a T {
        self.segundo
    }
}

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
