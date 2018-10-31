use std::io::Error;
use std::io::ErrorKind;
use std::str::Lines;
use types::item::Currency;
use types::item::DivinationCard;
use types::item::Item;
use types::item::ItemRarity;
use types::item::KeyCapture;
use types::item::KeyCapture::{Capture, NoCapture};
use types::item::Map;
use types::item::Rest;
use types::item::StackSize;

fn parse_tooltip(content: String) -> Result<Item, Error> {
    let (rarity, rest) = parse_rarity(content)?;

    match rarity {
        ItemRarity::Currency => parse_currency(rest),
        ItemRarity::DivinationCard => parse_divination_cards(rest),
        ItemRarity::Magical | ItemRarity::Normal | ItemRarity::Rare | ItemRarity::Unique => {
            unimplemented!()
        }
    }
}

fn parse_map(name: Option<String>, kind: String, rarity: ItemRarity, rest: String) -> Result<Item, Error> {
    let rest = parse_divider(rest)?;
    let (tier, rest) = parse_tier(rest)?;
    let rest = parse_divider(rest)?;
    let (item_level, rest) = parse_item_level(rest)?;
    let rest = parse_divider(rest)?;
    let (affixes, rest) = parse_affixes(rest)?;
    let rest = parse_divider(rest)?;
    let _desc = parse_description(rest)?;

    Ok(Item::Map(Map {
        kind: String::new(),
        quality: 0,
        item_quantity: 0,
        item_rarity: 0,
        pack_size: 0,
        name: name,
        rarity: rarity,
        affixes: affixes,
        tier: tier,
        item_level: item_level,
    }))
}

fn parse_currency(rest: String) -> Result<Item, Error> {
    let (name, name_rest) = parse_name(rest)?;
    let first_divider = parse_divider(name_rest)?;
    let (stack_size, stack_rest) = parse_stack_size(first_divider)?;
    let second_div = parse_divider(stack_rest)?;
    let (affixes, affixes_rest) = parse_affixes(second_div)?;
    let third_div = parse_divider(affixes_rest)?;
    let desc = parse_description(third_div)?;

    Ok(Item::Currency(Currency {
        name: name,
        stack_size: stack_size,
        affixes: affixes,
        description: desc,
    }))
}

fn parse_divination_cards(item: String) -> Result<Item, Error> {
    let (name, rest) = parse_name(item)?;
    let rest = parse_divider(rest)?;
    let (stacks, rest) = parse_stack_size(rest)?;
    let rest = parse_divider(rest)?;
    let (mut affixes, rest) = parse_affixes(rest)?;
    let rest = parse_divider(rest)?;
    let description = parse_description(rest)?;

    Ok(Item::DivinationCard(DivinationCard {
        name: name,
        stack_size: stacks,
        reward: affixes.remove(0), // There's only a single thing divcards grant.
        description: description,
    }))
}

// Parsers.

fn capture_required_line(item: String) -> Result<(String, Rest), Error> {
    let mut lines = item.lines();
    let name = match lines.next() {
        Some(x) => x.to_string(),
        None => return Err(generate_error(format!("Can't capture line: Empty string."))),
    };
    let rest: String = gather(lines);

    Ok((name, rest))
}

fn capture_key_line(item: String, key: &str) -> Result<KeyCapture, Error> {
    let mut lines = item.lines();
    let first_line = match lines.next() {
        Some(x) => x.to_string(),
        None => {
            return Err(generate_error(format!(
                "Can't capture key {:?}, empty string.",
                key
            )))
        }
    };

    match first_line.starts_with(key) {
        true => {
            // Keys are specified in form of "Key: Value", so we can add 2 to drop colon and space.
            let value_part = first_line[(key.len() + 2)..].to_string();
            Ok(Capture(value_part, gather(lines)))
        }
        false => Ok(NoCapture(item.clone())),
    }
}

// Applications. Concrete attributes that will be parsed.

fn parse_item_level(item: String) -> Result<(u32, Rest), Error> {
    let cap = match capture_key_line(item, "Item Level") {
        Ok(capture) => capture,
        Err(e) => return Err(generate_error(format!("Can't parse Item Level: {}", e))),
    };
    match cap {
        Capture(ilvl, rest) => match ilvl.parse::<u32>() {
            Ok(x) => Ok((x, rest)),
            Err(_e) => Err(generate_error(format!(
                "Can't parse Item Level: Value '{:?}' can't be parsed",
                ilvl
            ))),
        },
        NoCapture(_rest) => Err(generate_error(format!(
            "Can't parse Item Level: Key not found in tool tip {:?}",
            item
        ))),
    }
}

fn parse_tier(item: String) -> Result<(u32, Rest), Error> {
    let res = capture_key_line(item, "Map Tier")?;
    match res {
        Capture(tier_string, rest) => {
            // Tier strings might include an "(augmented)", which would mess up parsing.
            // I also highly doubt tiers are going to reach higher than 99.
            let relevant_parts = &tier_string[..1];
            match relevant_parts.parse::<u32>() {
                Ok(tier) => Ok((tier, rest)),
                Err(e) => Err(generate_error(format!(
                    "Could not parse tier into number, error: {:?}",
                    e
                ))),
            }
        }
        NoCapture(_rest) => Err(generate_error(format!(
            "Could not parse map tier, key not found"
        ))),
    }
}

fn parse_affixes(item: String) -> Result<(Vec<String>, Rest), Error> {
    if item.is_empty() {
        return Err(generate_error(format!("Can't parse affixes: Empty string")));
    }

    let mut lines = item.lines();
    let mut affixes: Vec<String> = Vec::new();

    loop {
        let this_line = match lines.next() {
            Some(x) => x.to_string(),
            None => {
                return Err(generate_error(format!(
                    "Can't parse affixes: EOF while parsing"
                )))
            }
        };

        if this_line == "--------".to_string() {
            let mut rest = this_line;
            rest.push('\n');
            rest.push_str(&gather(lines));
            return Ok((affixes, rest));
        }

        affixes.push(this_line);
    }
}

fn parse_description(item: String) -> Result<String, Error> {
    match item.len() != 0 {
        true => Ok(item),
        false => Err(generate_error(format!(
            "Can't parse description: Empty String."
        ))),
    }
}

fn parse_rarity(item: String) -> Result<(ItemRarity, Rest), Error> {
    let mut item_lines = item.lines();
    let first_line = match item_lines.next() {
        Some(x) => x,
        None => {
            return Err(generate_error(
                "Empty string, can't parse rarity".to_string(),
            ))
        }
    };
    let rest: String = gather(item_lines);

    match first_line.starts_with("Rarity: ") {
        true => match &first_line[8..] {
            "Unique" => Ok((ItemRarity::Unique, rest)),
            "Currency" => Ok((ItemRarity::Currency, rest)),
            "Normal" => Ok((ItemRarity::Normal, rest)),
            "Magical" => Ok((ItemRarity::Magical, rest)),
            "Rare" => Ok((ItemRarity::Rare, rest)),
            "Divination Card" => Ok((ItemRarity::DivinationCard, rest)),
            r @ _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Rarity {} is not a valid rarity!", r),
            )),
        },

        false => Err(Error::new(
            ErrorKind::InvalidData,
            format!("No item rarity in first line, in this tooltip: \n {}", item),
        )),
    }
}

fn parse_divider(item: String) -> Result<Rest, Error> {
    let mut lines = item.lines();
    let relevant_line = match lines.next() {
        Some(x) => x,
        None => {
            return Err(generate_error(format!(
                "Can't parse divider: Empty string."
            )))
        }
    };
    let rest: String = gather(lines);

    match relevant_line {
        "--------" => Ok(rest),
        _ => Err(generate_error(format!(
            "No divider was found in line '{}'.",
            relevant_line
        ))),
    }
}

fn parse_name(item: String) -> Result<(String, Rest), Error> {
    capture_required_line(item)
}

fn parse_stack_size(item: String) -> Result<(StackSize, Rest), Error> {
    let mut lines = item.lines();
    let relevant_line = match lines.next() {
        Some(x) => x,
        None => {
            return Err(generate_error(
                "Empty string, can't parse stack size.".to_string(),
            ))
        }
    };
    let rest: String = gather(lines);

    match relevant_line.starts_with("Stack Size: ") {
        true => {
            let relevant_string = relevant_line[12..].to_string();
            let split: Vec<_> = relevant_string.split("/").collect();

            if split.len() != 2 {
                return Err(generate_error(format!(
                    "Malformed or no information found in line '{}', cannot parse stacksize.",
                    relevant_line
                )));
            }

            let current: u32 = match split[0].parse() {
                Ok(x) => x,
                Err(_e) => {
                    return Err(generate_error(format!(
                        "Can't parse '{}' into number for stack szie.",
                        split[0]
                    )))
                }
            };
            let max: u32 = match split[1].parse() {
                Ok(x) => x,
                Err(_e) => {
                    return Err(generate_error(format!(
                        "Can't parse '{}' into number for stack size.",
                        split[1]
                    )))
                }
            };

            Ok((
                StackSize {
                    current: current,
                    max: max,
                },
                rest,
            ))
        }
        false => Err(generate_error(format!(
            "Line '{}' does not hold a valid stack size.",
            relevant_line
        ))),
    }
}

fn generate_error(reason: String) -> Error {
    Error::new(ErrorKind::InvalidData, reason)
}

fn gather(mut t: Lines) -> String {
    let first_line = match t.next() {
        Some(x) => x.to_string(),
        None => return String::new(),
    };

    t.fold(first_line, |mut acc, line| {
        acc.push('\n');
        acc.push_str(line);
        acc
    })
}

#[cfg(test)]
mod test {
    use super::*;

    mod affix_text {
        use super::*;

        #[test]
        fn should_stop_at_divider() {
            let test_string = "Foo\nBar\n--------\nBaz".to_string();
            let res = parse_affixes(test_string);

            assert!(res.is_ok());

            let (affixes, rest) = res.unwrap();
            assert_eq!(rest, "--------\nBaz".to_string());
            assert_eq!(affixes.len(), 2);
            assert_eq!(affixes[0], "Foo".to_string());
            assert_eq!(affixes[1], "Bar".to_string());
        }
    }

    mod name_test {
        use super::*;

        #[test]
        fn should_parse_name_correctly() {
            let test_string = "Chaos Orb\nFoobar".to_string();
            let res = parse_name(test_string);

            assert!(res.is_ok());

            let (name, rest) = res.unwrap();
            assert_eq!(name, "Chaos Orb".to_string());
            assert_eq!(rest, "Foobar".to_string());
        }
    }

    mod divider_test {
        use super::*;

        #[test]
        fn should_parse_diviers() {
            let test_string = "--------".to_string();
            let res = parse_divider(test_string);

            assert!(res.is_ok());

            let uw = res.unwrap();

            assert_eq!(uw, "".to_string());
        }

        #[test]
        fn should_break_on_malformed_dividers() {
            let test_string = "------".to_string();
            let res = parse_divider(test_string);

            assert!(res.is_err());
        }
    }

    mod stack_size_test {
        use super::*;

        #[test]
        fn should_parse_simple_stacks() {
            let test_string = "Stack Size: 10/20\n".to_string();
            let res = parse_stack_size(test_string);

            assert!(res.is_ok());

            let (stacks, _rest) = res.unwrap();

            assert_eq!(stacks.current, 10);
            assert_eq!(stacks.max, 20);
        }

        #[test]
        fn should_break_on_malformed_stacks() {
            let test_string = "Stack Size: Foo/Bar".to_string();
            let res = parse_stack_size(test_string);

            assert!(res.is_err());

            let other_test_string = "Stack Size: 10/12/10".to_string();
            let res2 = parse_stack_size(other_test_string);

            assert!(res2.is_err());
        }

        #[test]
        fn should_break_on_stacks_without_slash() {
            let test_string = "Stack Size: 10".to_string();
            let res = parse_stack_size(test_string);

            assert!(res.is_err());
        }
    }

    mod rarity_test {
        use super::*;

        #[test]
        fn should_parse_unique_rarities() {
            let test_string = "Rarity: Unique\n".to_string();
            assert_eq!(
                parse_rarity(test_string).unwrap(),
                (ItemRarity::Unique, "".to_string())
            )
        }

        #[test]
        fn should_error_on_bad_rarities() {
            let test_string = "Rarity: Some Shit".to_string();
            assert!(parse_rarity(test_string).is_err());
        }

        #[test]
        fn should_correclty_handle_empty_string() {
            let test_string = "".to_string();
            assert!(parse_rarity(test_string).is_err());
        }
    }

    #[test]
    fn should_parse_currencies() {
        let chaos_orb = include_str!("../resources/chaos-orb").to_string();
        let result = parse_tooltip(chaos_orb);

        match result.unwrap() {
            Item::Currency(c) => {
                assert_eq!(c.name, "Chaos Orb".to_string());
                assert_eq!(c.affixes.len(), 1);
                assert_eq!(
                    c.stack_size,
                    StackSize {
                        current: 20,
                        max: 10,
                    }
                )
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn should_parse_essences() {
        let essence = include_str!("../resources/essence-of-spite").to_string();
        let result = parse_tooltip(essence);

        match result.unwrap() {
            Item::Currency(c) => {
                assert_eq!(c.affixes.len(), 4);
                assert_eq!(c.stack_size, StackSize { current: 1, max: 9 });
                assert_eq!(c.name, "Shrieking Essence of Spite".to_string());
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn should_parse_maps() {
        let cage = include_str!("../resources/shaped-cage").to_string();
        let result = parse_tooltip(cage);

        assert!(result.is_ok());

        match result.unwrap() {
            Item::Map(m) => {
                assert_eq!(m.name, Some("Shaped Cage Map".to_string()));
                assert_eq!(m.rarity, ItemRarity::Normal);
                assert_eq!(m.item_level, 75);
                assert_eq!(m.affixes.len(), 0);
                assert_eq!(m.tier, 8);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn should_parse_complex_maps() {
        let sugs = include_str!("../resources/shaped-underground-sea").to_string();
        match parse_tooltip(sugs) {
            Ok(Item::Map(map)) => {
                assert_eq!(map.tier, 11);
                assert_eq!(map.kind, "Shaped Underground Sea Map".to_string());
                assert_eq!(map.item_level, 79);
                assert_eq!(map.quality, 20);
                assert_eq!(map.item_quantity, 142);
                assert_eq!(map.item_rarity, 72);
                assert_eq!(map.pack_size, 46);
                assert_eq!(map.affixes.len(), 12);
            }
            Ok(_) => assert!(false),
            Err(some_err) => {
                println!("{:?}", some_err);
                assert!(false)
            }
        }
    }

    #[test]
    fn should_parse_divination_cards() {
        let card = include_str!("../resources/heterochromia-card").to_string();
        let result = parse_tooltip(card);

        assert!(result.is_ok());

        match result.unwrap() {
            Item::DivinationCard(c) => {
                assert_eq!(c.name, "Heterochromia".to_string());
                assert_eq!(c.reward, "Two-Stone Ring".to_string());
                assert_eq!(c.stack_size, StackSize { current: 1, max: 2 });
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn should_parse_uniques() {
        let inpulsas = include_str!("../resources/inpulsas-broken-heart").to_string();
        let result = parse_tooltip(inpulsas);

        assert!(result.is_ok());

        match result.unwrap() {
            Item::Gear(g) => {
                assert_eq!(g.name, "Inpulsa's Broken Heart".to_string());
                assert_eq!(g.rarity, ItemRarity::Unique);
                assert_eq!(g.requirements.level, 68);
                assert_eq!(g.item_level, 74);
                // Inaccurate test, but no matter for this purpose.
                assert_eq!(g.affixes.len(), 7);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn should_preserve_newlines() {
        let test_string = String::from("a\nb\nc\nd");
        let mut lines = test_string.lines();
        let _first_line = lines.next().unwrap();
        let rest = gather(lines);
        assert_eq!(rest, "b\nc\nd");
    }
}
