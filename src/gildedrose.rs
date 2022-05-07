use std::fmt::{self, Display};
pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            if item.name != "Aged Brie" && item.name != "Backstage passes to a TAFKAL80ETC concert"
            {
                if item.quality > 0 {
                    if item.name != "Sulfuras, Hand of Ragnaros" {
                        item.quality = item.quality - 1;
                    }
                }
            } else {
                if item.quality < 50 {
                    item.quality = item.quality + 1;

                    if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                        if item.sell_in < 11 {
                            if item.quality < 50 {
                                item.quality = item.quality + 1;
                            }
                        }

                        if item.sell_in < 6 {
                            if item.quality < 50 {
                                item.quality = item.quality + 1;
                            }
                        }
                    }
                }
            }

            if item.name != "Sulfuras, Hand of Ragnaros" {
                item.sell_in = item.sell_in - 1;
            }

            if item.sell_in < 0 {
                if item.name != "Aged Brie" {
                    if item.name != "Backstage passes to a TAFKAL80ETC concert" {
                        if item.quality > 0 {
                            if item.name != "Sulfuras, Hand of Ragnaros" {
                                item.quality = item.quality - 1;
                            }
                        }
                    } else {
                        item.quality = item.quality - item.quality;
                    }
                } else {
                    if item.quality < 50 {
                        item.quality = item.quality + 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};

    #[test]
    pub fn at_update_sell_in_and_quality_reduced_for_general_items() {
        // Given a general item
        let name = "General Item";
        let sell_in = 23;
        let quality = 13;

        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when updated
        rose.update_quality();

        // Item.sell_in decrements by 1
        assert_eq!(sell_in - 1, rose.items[0].sell_in);

        // Item.quality decrements by 1
        assert_eq!(quality - 1, rose.items[0].quality);
    }

    #[test]
    pub fn if_sell_by_date_has_passed_then_quality_degrates_twice_as_fast() {
        // given a general item
        let name = "General Item";
        let quality = 13;

        // if sell_by date has passed...
        let sell_in = 0;

        // when updated
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        // then quality degrades twice as fast
        assert_eq!(quality - 2, rose.items[0].quality);
    }

    #[test]
    pub fn the_quality_of_an_item_never_negative_following_update() {
        // Given a general item
        let name = "General Item";
        let sell_in = 0;
        let quality = 0;

        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when updated
        rose.update_quality();

        // quality is positive! :D
        assert!(quality >= 0);
    }


}
