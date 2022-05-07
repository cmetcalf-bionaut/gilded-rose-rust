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

    impl Default for Item {
        fn default() -> Item {
            Item {
                name: "General Item".into(),
                sell_in: 5,
                quality: 32,
            }
        }
    }

    fn generate_one_item_sytem_from_item(item: Item) -> GildedRose {
        let items = vec![item];
        GildedRose::new(items)
    }

    #[test]
    fn at_update_sell_in_and_quality_reduced_for_general_items() {
        // Given a general item
        let mut rose = generate_one_item_sytem_from_item(Item::default());

        // when updated
        rose.update_quality();

        // Item.sell_in decrements by 1
        assert_eq!(Item::default().sell_in - 1, rose.items[0].sell_in);

        // Item.quality decrements by 1
        assert_eq!(Item::default().quality - 1, rose.items[0].quality);
    }

    #[test]
    fn if_sell_in_date_has_passed_then_quality_degrates_twice_as_fast() {
        // given a general item
        // if sell_by date has passed...
        let egg_salad_sandwich = Item {
            sell_in: 0,
            ..Item::default()
        };
        let mut rose = generate_one_item_sytem_from_item(egg_salad_sandwich);

        // when updated
        rose.update_quality();

        // then quality degrades twice as fast
        assert_eq!(Item::default().quality - 2, rose.items[0].quality);
    }

    #[test]
    fn the_quality_of_an_item_never_negative_following_update() {
        const ROCK_BOTT0M: i32 = 0;

        // Given a general item with no quality what-so-ever
        let crap_item = Item {
            quality: ROCK_BOTT0M,
            ..Item::default()
        };

        let mut rose = generate_one_item_sytem_from_item(crap_item);

        // when updated
        rose.update_quality();

        // at least quality stays positive! :D
        assert!(rose.items[0].quality >= ROCK_BOTT0M);
    }

    #[test]
    fn aged_bried_increases_quality_the_older_it_gets() {
        const CHEESE_FACTOR: i32 = 1;
        const CON_QUESO: &str = "Aged Brie";

        // Given a chunk of delicious Brie. Mmmm...
        // just warmed up in the oven, and covered in jalepeno jelly...
        //
        // omg yes.
        let senior_fromage = Item {
            name: CON_QUESO.into(),
            ..Item::default()
        };
        let mut rose = generate_one_item_sytem_from_item(senior_fromage);

        // when updated
        rose.update_quality();

        // THE CHEESE STANDS ALONE.
        assert_eq!(
            Item::default().quality + CHEESE_FACTOR,
            rose.items[0].quality
        );
        assert_eq!(CON_QUESO, rose.items[0].name);
    }

    #[test]
    fn quality_never_updates_above_50_if_initially_under_50() {
        // Given a high quality item which could only improve with age
        const TOP_NOTCH_STUFF: i32 = 50;
        const TO_THE_MOON: &str = "Aged Brie";

        let bitcoin = Item {
            name: TO_THE_MOON.into(),
            quality: TOP_NOTCH_STUFF,
            ..Item::default()
        };
        let mut rose = generate_one_item_sytem_from_item(bitcoin);

        // when updated
        rose.update_quality();

        // quality does not increase greater than 50
        assert!(rose.items[0].quality <= TOP_NOTCH_STUFF);
        assert_eq!(TO_THE_MOON, rose.items[0].name);
    }

    #[test]
    fn sulfuras_sell_by_and_quality_never_decrease() {
        // Given a hunk of Sulfurus, which is pretty good stuff.
        const SULFURAS: &str = "Sulfuras, Hand of Ragnaros";
        let sulfuras = Item {
            name: SULFURAS.into(),
            ..Item::default()
        };
        let mut rose = generate_one_item_sytem_from_item(sulfuras);

        // when updated
        rose.update_quality();

        // everything stays the same
        assert_eq!(SULFURAS, rose.items[0].name);
        assert_eq!(Item::default().sell_in, rose.items[0].sell_in);
        assert_eq!(Item::default().quality, rose.items[0].quality);
    }

    #[test]
    fn backstage_passes_quality_is_zero_if_sell_in_less_than_zero() {
        // Given a sweet backstage pass to BTS, probably.
        const KPOP_BOY_BAND_OMG: &str = "Backstage passes to a TAFKAL80ETC concert";
        const BUT_I_MISSED_IT_WAH: i32 = 0;

        let ticket = Item {
            name: KPOP_BOY_BAND_OMG.into(),
            sell_in: BUT_I_MISSED_IT_WAH,
            ..Item::default()
        };

        // But you went to a shady scalper after showing up an hour late.
        assert!(ticket.quality >= 0); //yup. seems legit.
        let mut rose = generate_one_item_sytem_from_item(ticket);

        // when updated
        rose.update_quality();

        assert_eq!(KPOP_BOY_BAND_OMG, rose.items[0].name);
        assert_eq!(0, rose.items[0].quality);
        assert!(rose.items[0].sell_in < BUT_I_MISSED_IT_WAH);
    }

    #[test]
    fn backstage_passes_quality_decreases_by_3_if_five_days_or_fewer_left_to_sell() {
        // Given a sweet backstage pass to BTS, probably.
        const KPOP_BOY_BAND_OMG: &str = "Backstage passes to a TAFKAL80ETC concert";
        const COMING_SOON: i32 = 5;
        const EXCITEMENT_FACTOR: i32 = 3;

        let ticket = Item {
            name: KPOP_BOY_BAND_OMG.into(),
            sell_in: COMING_SOON,
            ..Item::default()
        };

        // you went to a place in time!
        assert!(ticket.quality >= 0); //yup. seems legit.
        let mut rose = generate_one_item_sytem_from_item(ticket);

        // when updated
        rose.update_quality();

        // HOLY COW THESE ARE WORTH A BUNCH!
        assert_eq!(KPOP_BOY_BAND_OMG, rose.items[0].name);
        assert_eq!(
            Item::default().quality + EXCITEMENT_FACTOR,
            rose.items[0].quality
        );
        assert_eq!(COMING_SOON - 1, rose.items[0].sell_in);
    }

    #[test]
    fn backstage_passes_quality_decreases_by_2_if_6_days_left_to_sell() {
        // Given a sweet backstage pass to BTS, probably.
        const KPOP_BOY_BAND_OMG: &str = "Backstage passes to a TAFKAL80ETC concert";
        const COMING_SOON: i32 = 6;
        const EXCITEMENT_FACTOR: i32 = 2;

        let ticket = Item {
            name: KPOP_BOY_BAND_OMG.into(),
            sell_in: COMING_SOON,
            ..Item::default()
        };

        // you went to a place in time!
        assert!(ticket.quality >= 0); //yup. seems legit.
        let mut rose = generate_one_item_sytem_from_item(ticket);

        // when updated
        rose.update_quality();

        // HOLY COW THESE ARE WORTH A BUNCH!
        assert_eq!(KPOP_BOY_BAND_OMG, rose.items[0].name);
        assert_eq!(
            Item::default().quality + EXCITEMENT_FACTOR,
            rose.items[0].quality
        );
        assert_eq!(COMING_SOON - 1, rose.items[0].sell_in);
    }

    #[test]
    fn backstage_passes_quality_decreases_by_2_if_10_days_left_to_sell() {
        // Given a sweet backstage pass to BTS, probably.
        const KPOP_BOY_BAND_OMG: &str = "Backstage passes to a TAFKAL80ETC concert";
        const COMING_SOON: i32 = 10;
        const EXCITEMENT_FACTOR: i32 = 2;

        let ticket = Item {
            name: KPOP_BOY_BAND_OMG.into(),
            sell_in: COMING_SOON,
            ..Item::default()
        };

        // you went to a place in time!
        assert!(ticket.quality >= 0); //yup. seems legit.
        let mut rose = generate_one_item_sytem_from_item(ticket);

        // when updated
        rose.update_quality();

        // HOLY COW THESE ARE WORTH A BUNCH!
        assert_eq!(KPOP_BOY_BAND_OMG, rose.items[0].name);
        assert_eq!(
            Item::default().quality + EXCITEMENT_FACTOR,
            rose.items[0].quality
        );
        assert_eq!(COMING_SOON - 1, rose.items[0].sell_in);
    }

    #[test]
    fn backstage_passes_quality_decreases_by_1_if_11_days_left_to_sell() {
        // Given a sweet backstage pass to BTS, probably.
        const KPOP_BOY_BAND_OMG: &str = "Backstage passes to a TAFKAL80ETC concert";
        const COMING_SOON: i32 = 11;
        const EXCITEMENT_FACTOR: i32 = 1;

        let ticket = Item {
            name: KPOP_BOY_BAND_OMG.into(),
            sell_in: COMING_SOON,
            ..Item::default()
        };

        // you went to a place in time!
        assert!(ticket.quality >= 0); //yup. seems legit.
        let mut rose = generate_one_item_sytem_from_item(ticket);

        // when updated
        rose.update_quality();

        // HOLY COW THESE ARE WORTH A BUNCH!
        assert_eq!(KPOP_BOY_BAND_OMG, rose.items[0].name);
        assert_eq!(
            Item::default().quality + EXCITEMENT_FACTOR,
            rose.items[0].quality
        );
        assert_eq!(COMING_SOON - 1, rose.items[0].sell_in);
    }
}
