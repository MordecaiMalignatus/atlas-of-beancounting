use types::item::Item;

fn parse_tooltip(content: String) -> Result<Item, Error> {
    Err(Error::new(ErrorKind::Other, "Not implemented yet :("))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_currencies() {
        let chaos_orb = include_str!("../resources/chaos-orb").to_string();
        let result = parse_tooltip(chaos_orb);

        assert!(result.is_ok());

        let item = result.unwrap();

        assert!(item.name == "Chaos Orb".to_string());
        assert!(item.rarity == ItemRarity::Currency);
    }

    #[test]
    fn should_parse_essences() {
        let essence = include_str!("../resources/essence-of-spite").to_string();
        let result = parse_tooltip(essence);

        assert!(result.is_ok());

        let item = result.unwrap();

        assert!(item.name == "Essence Of Spite".to_string());
        assert!(item.rarity == ItemRarity::Currency);
    }

    #[test]
    fn should_parse_maps() {
        let cage = include_str!("../resources/shaped-cage").to_string();
        let result = parse_tooltip(cage);

        assert!(result.is_ok());

        let item = result.unwrap();

        assert!(item.name == "Shaped Cage Map".to_string());
        assert!(item.rarity == ItemRarity::Normal);
        assert!(item.item_level == 75);
        assert!(item.affixes.len() == 1);
        assert!(item.affixes[0] == "Map Tier: 8 (augmented)".to_string())
    }

    #[test]
    fn should_parse_divination_cards() {
        let card = include_str!("../resources/heterochromia-card").to_string();
        let result = parse_tooltip(card);

        assert!(result.is_ok());

        let item = result.unwrap();

        assert!(item.name == "Heterochromia".to_string());
        assert!(item.rarity == ItemRarity::DivinationCard);
        assert!(item.affixes.len() == 1);
        assert!(item.affixes[0] == "Two-Stone Ring".to_string());
        assert!(item.stack_size == (1, 2));
    }

    #[test]
    fn should_parse_uniques() {
        let inpulsas = include_str!("../resources/inpulsas-broken-heart").to_string();
        let result = parse_tooltip(inpulsas);

        assert!(result.is_ok());

        let item = result.unwrap();

        assert!(item.name == "Inpulsa's Broken Heart".to_string());
        assert!(item.rarity == ItemRarity::Unique);
        assert!(item.requirements.unwrap().level == 68);
        // Inaccurate test, but no matter for this purpose.
        assert!(item.affixes.len() == 7);
    }
}
