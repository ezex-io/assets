use crate::asset::Asset;
use crate::types::Link;

pub trait Blockchain {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn website(&self) -> &str;
    fn explorer(&self) -> &str;
    fn links(&self) -> Vec<Link>;
    fn assets(&self) -> Vec<Box<dyn Asset>>;
    fn asset(&self, id: &str) -> Option<Box<dyn Asset>>;
    fn logo(&self) -> &str;
}
