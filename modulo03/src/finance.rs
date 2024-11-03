
// Structs
pub struct Asset {
    name: String,
    price: f64,
    category: String,
}

pub struct Portfolio {
    assets: Vec<Asset>,
}

// Impls
impl Asset {
    pub fn new(name: &str, price: f64, category: &str) -> Self {
        Self { name: name.to_string(), price, category: category.to_string() }
    }
}

impl Portfolio {
    pub fn new() -> Self { Self { assets: Vec::new() } }
    pub fn add_asset(&mut self, asset: Asset) { self.assets.push(asset); }
    pub fn total_value(&self) -> f64 { self.assets.iter().map( |asset| asset.price ).sum() }
}