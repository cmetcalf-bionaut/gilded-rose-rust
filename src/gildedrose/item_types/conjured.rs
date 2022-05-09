use crate::Item;
pub trait Conjured {
    fn update(&mut self);
}

impl Conjured for Item {
    fn update(&mut self) {
        if self.quality > 0 {
            self.quality -= 2;
        }
        self.sell_in -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_update_sell_in_decrements_by_one() {
        let mut item = Item {
            ..Default::default()
        };
        item.update();
        assert_eq!(item.sell_in, Item::default().sell_in - 1);
    }

    #[test]
    fn conjured_items_decrease_in_quality_twice_as_fast_as_normal_items() {
        let mut item = Item::default();
        item.update();
        assert_eq!(item.quality, Item::default().quality - 2);
    }
}
