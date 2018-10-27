use std::io::Error;
use std::io::ErrorKind;
use std::str::Lines;
use types::item::Item;
use types::item::ItemRarity;

type Rest = String;

fn parse_tooltip(content: String) -> Result<Item, Error> {
    let (rarity, rest) = parse_rarity(content)?;

    match rarity {
        ItemRarity::Currency => parse_currency(rest),
        ItemRarity::DivinationCard => unimplemented!(),
        ItemRarity::Magical | ItemRarity::Normal | ItemRarity::Rare | ItemRarity::Unique => {
            unimplemented!()
        }
    }
}

fn parse_currency(rest: String) -> Result<Item, Error> {
    let (name, name_rest) = parse_name(rest)?;
    let first_divider = parse_divider(name_rest)?;
    let (stack_size, stack_rest) = parse_stack_size(first_divider)?;
    let second_div = parse_divider(stack_rest)?;
    let (affixes, affixes_rest) = parse_affixes(second_div)?;
    let third_div = parse_divider(affixes_rest)?;
    let desc = parse_description(third_div)?;

    Ok(Item {
        rarity: ItemRarity::Currency,
        name: name,
        stack_size: stack_size,
        affixes: affixes,
        description: desc,
        item_level: 0,
        requirements: None,
        sockets: None,
    })
}

// Parser Combinators.

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
    let mut lines = item.lines();
    let name = match lines.next() {
        Some(x) => x.to_string(),
        None => return Err(generate_error(format!("Can't parse name: Empty string."))),
    };
    let rest: String = gather(lines);

    Ok((name, rest))
}

fn parse_stack_size(item: String) -> Result<((u32, u32), Rest), Error> {
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

            Ok(((current, max), rest))
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
        None => return String::new()
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

            let ((current, max), _rest) = res.unwrap();

            assert_eq!(current, 10);
            assert_eq!(max, 20);
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

        println!("{:?}", result);
        assert!(result.is_ok());

        let item = result.unwrap();

        assert_eq!(item.name, "Chaos Orb".to_string());
        assert_eq!(item.rarity, ItemRarity::Currency);
    }

    #[test]
    fn should_parse_essences() {
        let essence = include_str!("../resources/essence-of-spite").to_string();
        let result = parse_tooltip(essence);

        assert!(result.is_ok());

        let item = result.unwrap();

        assert_eq!(item.name, "Shrieking Essence of Spite".to_string());
        assert_eq!(item.rarity, ItemRarity::Currency);
    }

    #[test]
    fn should_parse_maps() {
        let cage = include_str!("../resources/shaped-cage").to_string();
        let result = parse_tooltip(cage);

        assert!(result.is_ok());

        let item = result.unwrap();

        assert_eq!(item.name, "Shaped Cage Map".to_string());
        assert_eq!(item.rarity, ItemRarity::Normal);
        assert_eq!(item.item_level, 75);
        assert_eq!(item.affixes.len(), 1);
        assert_eq!(item.affixes[0], "Map Tier: 8 (augmented)".to_string())
    }

    #[test]
    fn should_parse_divination_cards() {
        let card = include_str!("../resources/heterochromia-card").to_string();
        let result = parse_tooltip(card);

        assert!(result.is_ok());

        let item = result.unwrap();

        assert_eq!(item.name, "Heterochromia".to_string());
        assert_eq!(item.rarity, ItemRarity::DivinationCard);
        assert_eq!(item.affixes.len(), 1);
        assert_eq!(item.affixes[0], "Two-Stone Ring".to_string());
        assert_eq!(item.stack_size, (1, 2));
    }

    #[test]
    fn should_parse_uniques() {
        let inpulsas = include_str!("../resources/inpulsas-broken-heart").to_string();
        let result = parse_tooltip(inpulsas);

        assert!(result.is_ok());

        let item = result.unwrap();

        assert_eq!(item.name, "Inpulsa's Broken Heart".to_string());
        assert_eq!(item.rarity, ItemRarity::Unique);
        assert_eq!(item.requirements.unwrap().level, 68);
        // Inaccurate test, but no matter for this purpose.
        assert_eq!(item.affixes.len(), 7);
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
