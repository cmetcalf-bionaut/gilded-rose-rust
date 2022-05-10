use crate::Item;

mod aged_brie;
mod concert_tickets;
mod conjured;
mod legendary;
mod standard_item;

use aged_brie::AgedBrie;
use concert_tickets::ConcertTickets;
use conjured::Conjured;
use legendary::LegendaryItem;
use standard_item::StandardItem;

pub trait UpdateItem {
    fn update(&mut self) {}
}

impl UpdateItem for Item {
    fn update(&mut self) {
        match self.name.as_str() {
            <Self as AgedBrie>::NAME => AgedBrie::update(self),
            <Self as ConcertTickets>::NAME => ConcertTickets::update(self),
            <Self as LegendaryItem>::NAME => LegendaryItem::update(self),
            <Self as Conjured>::NAME => Conjured::update(self),
            _ => StandardItem::update(self),
        }
    }
}
