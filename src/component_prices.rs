use itertools::Itertools;
use serde_json::{self, Value};
use crate::{utils::print_warning, prelude::*};
use std::{collections::HashMap, str::FromStr, fs};
use regex::Regex;
use once_cell::sync::OnceCell;

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    ";",
    env!("CARGO_PKG_REPOSITORY")
);

static SHELL_PRICE: OnceCell<f64> = OnceCell::new();
static PRICES: OnceCell<HashMap<MaterialName, f64>> = OnceCell::new();

pub fn calc_gizmo_price(line: &ResultLine) -> f64 {
    let shell_price = SHELL_PRICE.get().unwrap();
    let prices = PRICES.get().unwrap();
    let price = shell_price + line.mat_combination.iter().fold(0.0, |acc, x| {
        acc + prices.get(x).unwrap()
    });

    price / line.prob_gizmo
}

pub fn load_component_prices(args: &Args) {
    let mut text = String::new();

    if std::path::Path::new("prices.txt").exists() {
        match fs::read_to_string("prices.txt") {
            Ok(file) => text = file,
            Err(err) => print_warning(format!("Failed to read prices.txt: {}", err).as_str())
        }
    } else {
        match lookup_on_wiki() {
            Ok(response) => text = extract_from_response(&response),
            Err(_) => print_warning("Failed to fetch prices")
        }
    }

    let mut prices = string_to_map(&text);
    for mat in MaterialName::iter() {
        prices.entry(mat).or_insert_with(|| {
            print_warning(format!("Price missing for '{}'", mat).as_str());
            0.0
        });
    }

    text = prices.iter().map(|(name, value)| format!("{}: {},", name, value)).sorted().join("\n");
    fs::write("prices.txt", text).unwrap_or_else(|err| {
        print_warning(format!("Failed to save prices.txt: {}", err).as_str());
    });

    SHELL_PRICE.set(calc_shell_price(args, &prices)).ok();
    PRICES.set(prices).ok();
}

fn calc_shell_price(args: &Args, prices: &HashMap<MaterialName, f64>) -> f64 {
    match args.ancient {
        true => match args.gizmo_type {
            GizmoType::Armour => 20.0 * prices.get(&MaterialName::DeflectingParts).unwrap() + 20.0 * prices.get(&MaterialName::HistoricComponents).unwrap() + 2.0 * prices.get(&MaterialName::ClassicComponents).unwrap() + 2.0 * prices.get(&MaterialName::ProtectiveComponents).unwrap(),
            GizmoType::Weapon => 20.0 * prices.get(&MaterialName::BladeParts).unwrap() + 20.0 * prices.get(&MaterialName::HistoricComponents).unwrap() + 2.0 * prices.get(&MaterialName::ClassicComponents).unwrap() + 2.0 * prices.get(&MaterialName::StrongComponents).unwrap(),
            GizmoType::Tool => 20.0 * prices.get(&MaterialName::HeadParts).unwrap() + 20.0 * prices.get(&MaterialName::HistoricComponents).unwrap() + 2.0 * prices.get(&MaterialName::ClassicComponents).unwrap() + 2.0 * prices.get(&MaterialName::PreciseComponents).unwrap(),
        },
        false => match args.gizmo_type {
            GizmoType::Armour => 10.0 * prices.get(&MaterialName::DeflectingParts).unwrap() + 5.0 * prices.get(&MaterialName::CraftedParts).unwrap() + 2.0 * prices.get(&MaterialName::ProtectiveComponents).unwrap(),
            GizmoType::Weapon => 10.0 * prices.get(&MaterialName::BladeParts).unwrap() + 5.0 * prices.get(&MaterialName::CraftedParts).unwrap() + 2.0 * prices.get(&MaterialName::StrongComponents).unwrap(),
            GizmoType::Tool => 10.0 * prices.get(&MaterialName::HeadParts).unwrap() + 5.0 * prices.get(&MaterialName::CraftedParts).unwrap() + 2.0 * prices.get(&MaterialName::PreciseComponents).unwrap(),
        },
    }
}

fn lookup_on_wiki() -> Result<String, reqwest::Error> {
    println!("Fetching component prices from Runescape.wiki...");
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    let body = client.get("https://runescape.wiki/api.php?action=parse&format=json&text=%7B%7B%23dpl%3Anamespace%3D%7Ccategory%3DMaterials%7Cnottitlematch%3DMaterials%7Cformat%3D%2C%25PAGE%25%3A%C2%B2%7Bmatcost%C2%A6%25PAGE%25%C2%A6raw%7D%C2%B2%5Cn%2C%2C%7D%7D&contentmodel=wikitext&formatversion=2")
        .send()?
        .text()?;

    Ok(body)
}

fn extract_from_response(response: &str) -> String {
    let json: Value = serde_json::from_str(response).unwrap_or_default();
    let text= &json["parse"]["text"];

    if let Value::String(text) = text {
        let re = Regex::new(r"^.+?<p>((.|\n)+?)</p>").unwrap();
        let content = re.captures(text);

        if let Some(content) = content {
            return content[1].to_string();
        } else {
            print_warning("Unexpected response from Runescape.wiki");
        }
    } else {
        print_warning("Unexpected response from Runescape.wiki");
    }

    String::new()
}

fn string_to_map(text: &str) -> HashMap<MaterialName, f64> {
    let mut prices = HashMap::new();
    let re = Regex::new(r"^([^:]+): ?([\d\.]+)").unwrap();
    let lines = text.split('\n');

    for line in lines {
        if let Some(captures) = re.captures(line) {
            if let Ok(mat) = MaterialName::from_str(&captures[1]) {
                if captures.len() == 3 {
                    let price: f64 = captures[2].parse().unwrap_or_else(|_| {
                        print_warning(format!("Failed to parse price for '{}'", mat).as_str());
                        0.0
                    });
                    prices.insert(mat, price);
                }
            }
        }
    }

    prices
}