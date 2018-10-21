use types::item::Item;

fn parse_tooltip(content: String) -> Option<Item> {
    None
}

#[cfg(test)]
mod test {
    use super::*;
    use types::item::ItemRarity;

    #[test]
    fn should_parse_currencies() {
        let chaos_orb = include_str!("../resources/chaos-orb").to_string();
        let result = parse_tooltip(chaos_orb);

        assert!(result.is_some());

        let item = result.unwrap();

        assert!(item.name == "Chaos Orb".to_string());
        assert!(item.rarity == ItemRarity::Currency);
    }
}
