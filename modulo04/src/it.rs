pub struct Router { ip: String, }

pub trait Network {
    fn ping(&self, host: &str) -> bool;
    fn traceroute(&self, host: &str) -> Vec<String>;
    fn nslookup(&self, host: &str) -> String;
}

impl Router {
    pub fn new(ip: &str) -> Router { Router { ip: ip.to_string() } }
}

impl Network for Router {
    fn ping(&self, host: &str) -> bool {
        // Simulando uma verificação de ping
        println!("Verificando conexão com {} através do IP {}", host, self.ip);
        true
    }


    fn traceroute(&self, host: &str) -> Vec<String> {
        // Simulando um trace de rota
        println!("Executando trace de rota para {} a partir do IP {}", host, self.ip);
        vec!["192.168.1.1".to_string(), "192.168.1.2".to_string(), "192.168.1.3".to_string()]
    }


    fn nslookup(&self, host: &str) -> String {
        // Simulando uma consulta de DNS
        println!("Executando consulta de DNS para {} a partir do IP {}", host, self.ip);
        "192.168.1.100".to_string()
    }
}

