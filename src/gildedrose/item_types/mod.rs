use crate::Item;

pub mod standard_item;
pub use standard_item::StandardItem;

pub trait AgedBrie {
    fn update(&mut self);
}

pub trait LegendaryItem {
    fn update(&mut self);
}

pub trait Conjured {
    fn update(&mut self);
}

pub trait ConcertTickets {
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

impl LegendaryItem for Item {
    fn update(&mut self) {}
}

impl ConcertTickets for Item {
    fn update(&mut self) {
        match self {
            _ if (6..11).contains(&self.sell_in) => self.quality += 2,
            _ if (1..6).contains(&self.sell_in) => self.quality += 3,
            _ if self.sell_in <= 0 => self.quality = 0,
            _ => self.quality += 1,
        }

        if self.quality >= 50 {
            self.quality = 50;
        }
        self.sell_in -= 1;
    }
}

impl Conjured for Item {
    fn update(&mut self) {
        if self.quality > 0 {
            self.quality -= 2;
        }
        self.sell_in -= 1;
    }
}
