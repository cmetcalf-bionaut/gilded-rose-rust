use crate::Item;

pub trait AgedBrie {
    const NAME: &'static str = "Aged Brie";
    fn update(&mut self);
}

impl AgedBrie for Item {
    fn update(&mut self) {
        (self.quality < 50).then(|| self.quality += 1);
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
