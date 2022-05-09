use crate::Item;

const NAME: &'static str = "Aged Brie";

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

    pub fn set_brie(item: Item) -> Option<impl AgedBrie> {
        if item.name == NAME {
            Some(item)
        } else {
            None
        }
    }

    #[test]
    fn if_name_is_not_aged_brie_return_none() {
        let item = Item::default();
        assert!(set_brie(item).is_none());
    }

    #[test]
    fn if_name_is_aged_brie_return_some() {
        let item = Item {
            name: NAME.to_string(),
            ..Default::default()
        };
        assert!(set_brie(item).is_some());
    }

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
