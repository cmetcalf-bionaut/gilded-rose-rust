use crate::Item;

pub trait ConcertTickets {
    const NAME: &'static str = "Backstage passes to a TAFKAL80ETC concert";
    fn update(&mut self);
}

impl ConcertTickets for Item {
    fn update(&mut self) {
        match self {
            _ if (6..11).contains(&self.sell_in) => self.quality += 2,
            _ if (1..6).contains(&self.sell_in) => self.quality += 3,
            _ if self.sell_in <= 0 => self.quality = 0,
            _ => self.quality += 1,
        }
        (self.quality >= 50).then(|| self.quality = 50);
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
    fn quality_zero_if_sell_in_zero_or_fewer() {
        let mut item = Item {
            sell_in: 0,
            ..Default::default()
        };

        assert!(item.quality > 0);
        item.update();
        assert_eq!(item.quality, 0);
    }

    #[test]
    fn quality_increases_by_3_when_5_days_remain() {
        let mut item = Item {
            sell_in: 5,
            ..Default::default()
        };

        item.update();
        assert_eq!(item.quality, Item::default().quality + 3);
    }

    #[test]
    fn quality_increases_by_3_when_1_days_remain() {
        let mut item = Item {
            sell_in: 1,
            ..Default::default()
        };

        item.update();
        assert_eq!(item.quality, Item::default().quality + 3);
    }

    #[test]
    fn quality_increase_by_2_when_6_days_remain() {
        let mut item = Item {
            sell_in: 6,
            ..Default::default()
        };

        item.update();
        assert_eq!(item.quality, Item::default().quality + 2);
    }

    #[test]
    fn quality_increase_by_2_when_10_days_remain() {
        let mut item = Item {
            sell_in: 10,
            ..Default::default()
        };

        item.update();
        assert_eq!(item.quality, Item::default().quality + 2);
    }

    #[test]
    fn quality_increase_by_1_when_over_ten_days_remain() {
        let mut item = Item {
            sell_in: 11,
            ..Default::default()
        };

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
