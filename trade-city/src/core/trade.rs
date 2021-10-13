pub trait Tradeable {
    fn name(&self) -> String;
    fn asset_type(&self) -> String;
    fn trade_unit(&self) -> f32;
}
