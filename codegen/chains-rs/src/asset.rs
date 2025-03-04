use crate::types::AssetType;

pub trait Asset {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn symbol(&self) -> &str;
    fn decimals(&self) -> u32;
    fn asset_type(&self) -> AssetType;
    fn asset_kind(&self) -> &str;
    fn bip44_coin_type(&self) -> isize;
    fn website(&self) -> &str;
    fn explorer(&self) -> &str;
    fn icon(&self) -> &str;
}
