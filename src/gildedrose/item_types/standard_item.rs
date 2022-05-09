use crate::Item;
pub trait StandardItem {
    fn update(&mut self);
}

impl StandardItem for Item {
    fn update(&mut self) {
        match self {
            _ if self.sell_in <= 0 && self.quality >= 2 => self.quality -= 2,
            _ if self.sell_in <= 0 && self.quality <= 0 => self.quality = 0,
            _ if self.quality > 0 => self.quality -= 1,
            _ => (),
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
    fn at_update_quality_decrements_by_one() {
        let mut item = Item {
            ..Default::default()
        };
        item.update();
        assert_eq!(item.quality, Item::default().quality - 1);
    }

    #[test]
    fn once_the_sell_in_date_has_passed_quality_degrades_twice_as_fast() {
        let mut item = Item {
            sell_in: 0,
            ..Default::default()
        };
        item.update();
        assert_eq!(item.quality, Item::default().quality - 2);
    }

    #[test]
    fn quality_of_an_item_is_never_negative() {
        let mut item = Item {
            quality: 0,
            ..Default::default()
        };
        item.update();
        assert!(item.quality >= 0);
    }
}
