use anyhow::Result;
use fxhash::{FxHashMap, FxHashSet};
use rayon::prelude::*;

type Bag = u16;

#[derive(Debug, Default)]
pub struct Bags {
    bag_id_counter: u16,
    bag_ids: FxHashMap<String, Bag>,
    containments: FxHashMap<Bag, FxHashSet<(Bag, u8)>>,
}

impl Bags {
    pub fn get_bag(&self, color: &str) -> Option<Bag> {
        self.bag_ids.get(color).map(|bag| *bag)
    }

    pub fn get_color(&self, bag: Bag) -> Option<String> {
        self.bag_ids.iter().find(|(_, id)| **id == bag).map(|(color, _)| color.to_owned())
    }

    pub fn get_or_create_bag(&mut self, color: &str) -> Bag {
        if let Some(bag) = self.get_bag(color) {
            return bag;
        }
        let bag = self.bag_id_counter;
        self.bag_id_counter += 1;
        self.bag_ids.insert(color.to_owned(), bag);
        return bag;
    }

    pub fn get_bag_containments(&self, bag: Bag) -> Option<&FxHashSet<(Bag, u8)>> {
        self.containments.get(&bag)
    }

    pub fn add_bag_containment(&mut self, bag: Bag, another_bag: Bag, amount: u8) -> Result<()> {
        let bag_containments = self.containments.entry(bag).or_default();
        if bag_containments.iter().any(|(bag, amount)| *bag == another_bag) {
            bail!("This bag was already added!")
        }
        bag_containments.insert((another_bag, amount));
        Ok(())
    }

    pub fn bags(&self) -> impl Iterator<Item = &Bag> {
        self.bag_ids.iter().map(|(color, id)| id)
    }

    pub fn contains_bag_recursively(&self, bag: Bag, searched_bag: Bag) -> bool {
        if let Some(containments) = self.get_bag_containments(bag) {
            for (contained, _) in containments {
                if *contained == searched_bag || self.contains_bag_recursively(*contained, searched_bag) {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn get_bag_containments_recursivly(&self, bag: Bag) -> Vec<(Bag, usize)> {
        let mut vec: Vec<(Bag, usize)> = vec![];
        if let Some(containments) = self.get_bag_containments(bag) {
            for (contained, amount) in containments {
                vec.push((*contained, *amount as usize));
                for (sub_contained_bag, sub_contained_amount) in self.get_bag_containments_recursivly(*contained) {
                    vec.push((sub_contained_bag, *amount as usize * sub_contained_amount as usize));
                }
            }
        }

        return vec;
    }
}

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Result<Bags> {
    let mut bags = Bags::default();
    // Find all bags
    for line in input.split('\n') {
        let words: Vec<&str> = line.split(' ').collect();
        let color = format!("{} {}", words[0], words[1]);
        ensure!(words[2] == "bags", "Colors are expected to be always 2 words");
        let bag = bags.get_or_create_bag(&color);
        // Add bags

        for contained_bag in line[line.find("contain ").ok_or(anyhow!("Invalid line"))? + "contain ".len()..line.len() - 1].split(", ") {
            if contained_bag == "no other bags" {
                continue;
            }

            let another_bag_split: Vec<&str> = contained_bag.split(' ').collect();
            ensure!(another_bag_split.len() == 4, "Expecting 4 words: NUM COLOR COLOR bag(s)");
            let another_color = format!("{} {}", another_bag_split[1], another_bag_split[2]);
            let another_amount = another_bag_split[0].parse()?;
            let another_bag = bags.get_or_create_bag(&another_color);

            bags.add_bag_containment(bag, another_bag, another_amount)?;
        }
    }

    Ok(bags)
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Bags) -> Result<usize> {
    //input.bags().for_each(|bag| println!("Bag: {:?}", input.get_color(*bag)));
    let searched_bag = input.get_bag("shiny gold").ok_or(anyhow!("Shiny gold bag not found"))?;
    Ok(input
        .bags()
        .par_bridge()
        .filter(|bag| input.contains_bag_recursively(**bag, searched_bag))
        .count())
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Bags) -> Result<usize> {
    let oh_shiny = input.get_bag("shiny gold").ok_or(anyhow!("Shiny gold bag not found"))?;
    Ok(input.get_bag_containments_recursivly(oh_shiny).iter().map(|(_, amount)| *amount).sum())
}
