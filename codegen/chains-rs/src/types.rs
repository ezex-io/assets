pub type AssetType = &'static str;

pub const COIN: AssetType = "coin";
pub const TOKEN: AssetType = "token";

#[derive(Debug, Clone)]
pub struct Link {
    pub name: String,
    pub url: String,
}
