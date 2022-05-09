use crate::Item;
pub trait AgedBrie {
    fn update(&mut self);
}

impl AgedBrie for Item {
    fn update(&mut self) {
        if self.quality < 50 {
            self.quality += 1;
        }
        self.sell_in -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aged_brie_quality_increases_with_age() {
        let mut item = Item::default();
        item.update();
        assert_eq!(item.quality, Item::default().quality + 1);
    }

    #[test]
    fn quality_never_increases_over_50() {
        let mut item = Item {
            quality: 50,
            ..Default::default()
        };

        item.update();
        assert!(item.quality <= 50);
    }
}
