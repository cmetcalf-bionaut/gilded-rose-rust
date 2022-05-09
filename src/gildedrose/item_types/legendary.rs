use crate::Item;
pub trait LegendaryItem {
    fn update(&mut self);
}

impl LegendaryItem for Item {
    fn update(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legendary_items_never_decrease_quality_or_sell_in() {
        let mut item = Item::default();
        item.update();
        assert_eq!(item.quality, Item::default().quality);
        assert_eq!(item.sell_in, Item::default().sell_in);
    }
}
