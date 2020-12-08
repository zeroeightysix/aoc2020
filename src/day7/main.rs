use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;

fn main() {
    const INPUT: &str = include_str!("input");

    let vec = INPUT.lines().map(Bag::from).collect::<HashSet<Bag>>();

    println!("Answer 1: {}", vec.iter().filter(|b| {
        b.contains_indirect("shiny gold bag", &vec)
    }).count());

    println!("Answer 2: {}", (&vec).find_bag("shiny gold bag").unwrap().count_bags(&vec) - 1)
}

#[derive(Eq, PartialEq, Debug)]
struct Bag(String, HashMap<String, usize>);

impl Hash for Bag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl Borrow<String> for Bag {
    fn borrow(&self) -> &String {
        &self.0
    }
}

impl From<&str> for Bag {
    fn from(s: &str) -> Self {
        let mut split = s.split(" contain ");
        let name = split.next().unwrap().trim_end_matches("s"); // bags -> bag
        let contains = split.next().unwrap();
        let contains = match contains {
            "no other bags." => HashMap::new(),
            contains => contains.trim_end_matches(".").split(", ").map(|bag| {
                let i = bag.find(" ").unwrap();
                let count: usize = (&bag[0..i]).parse().unwrap();
                let name = &bag[(i + 1)..].trim_end_matches("s");

                (name.to_string(), count)
            }).collect::<HashMap<String, usize>>()
        };

        Bag(name.to_string(), contains)
    }
}

trait FindBag {
    fn find_bag(&self, name: &str) -> Option<&Bag>;
}

impl FindBag for &HashSet<Bag> {
    fn find_bag(&self, name: &str) -> Option<&Bag> {
        self.get(&name.to_string())
    }
}

impl Bag {
    fn contains(&self, other: &str) -> bool {
        self.1.contains_key(other)
    }

    fn contains_indirect(&self, other: &str, bags: &HashSet<Bag>) -> bool {
        self.contains(other) || self.1.keys().any(|k| {
            bags.find_bag(k).map_or_else(|| false, |b| b.contains_indirect(other, bags))
        })
    }

    fn count_bags(&self, bags: &HashSet<Bag>) -> usize {
        1 /*myself*/ + self.1.iter().map(|(bag_name, v)| {
            let bag = bags.find_bag(bag_name);
            (match bag {
                None => 1,
                Some(bag) => bag.count_bags(bags)
            }) * v
        }).sum::<usize>()
    }
}