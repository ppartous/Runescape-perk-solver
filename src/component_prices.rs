use itertools::Itertools;
use reqwest;
use serde_json::{self, Value};
use crate::{utils::print_warning, definitions::*};
use std::{collections::HashMap, str::FromStr, fs, sync::Mutex};
use regex::Regex;
use lazy_static::lazy_static;

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    ";",
    env!("CARGO_PKG_REPOSITORY")
);

lazy_static!{
    static ref PRICES: HashMap<MaterialName, f64> = get_component_prices();
    static ref SHELL_PRICE: Mutex<Option<f64>> = Mutex::new(None);
}

fn get_shell_price(args: &Args) -> f64 {
    if let Some(price) = *SHELL_PRICE.lock().unwrap() {
        return price;
    }
    let price = match args.ancient {
        true => match args.gizmo_type {
            GizmoType::Armour => 20.0 * PRICES.get(&MaterialName::DeflectingParts).unwrap() + 20.0 * PRICES.get(&MaterialName::HistoricComponents).unwrap() + 2.0 * PRICES.get(&MaterialName::ClassicComponents).unwrap() + 2.0 * PRICES.get(&MaterialName::ProtectiveComponents).unwrap(),
            GizmoType::Weapon => 20.0 * PRICES.get(&MaterialName::BladeParts).unwrap() + 20.0 * PRICES.get(&MaterialName::HistoricComponents).unwrap() + 2.0 * PRICES.get(&MaterialName::ClassicComponents).unwrap() + 2.0 * PRICES.get(&MaterialName::StrongComponents).unwrap(),
            GizmoType::Tool => 20.0 * PRICES.get(&MaterialName::HeadParts).unwrap() + 20.0 * PRICES.get(&MaterialName::HistoricComponents).unwrap() + 2.0 * PRICES.get(&MaterialName::ClassicComponents).unwrap() + 2.0 * PRICES.get(&MaterialName::PreciseComponents).unwrap(),
        },
        false => match args.gizmo_type {
            GizmoType::Armour => 10.0 * PRICES.get(&MaterialName::DeflectingParts).unwrap() + 5.0 * PRICES.get(&MaterialName::CraftedParts).unwrap() + 2.0 * PRICES.get(&MaterialName::ProtectiveComponents).unwrap(),
            GizmoType::Weapon => 10.0 * PRICES.get(&MaterialName::BladeParts).unwrap() + 5.0 * PRICES.get(&MaterialName::CraftedParts).unwrap() + 2.0 * PRICES.get(&MaterialName::StrongComponents).unwrap(),
            GizmoType::Tool => 10.0 * PRICES.get(&MaterialName::HeadParts).unwrap() + 5.0 * PRICES.get(&MaterialName::CraftedParts).unwrap() + 2.0 * PRICES.get(&MaterialName::PreciseComponents).unwrap(),
        },
    };
    *SHELL_PRICE.lock().unwrap() = Some(price);;
    return price;
}

pub fn calc_gizmo_price(line: &ResultLine, args: &Args) -> f64 {
    let shell_price = get_shell_price(&args);

    let price = shell_price + line.mat_combination.iter().fold(0.0, |acc, x| {
        acc + PRICES.get(x).unwrap()
    });

    price / line.prob_gizmo
}

fn get_component_prices() -> HashMap<MaterialName, f64> {
    let mut text = String::new();

    if std::path::Path::new("prices.txt").exists() {
        match fs::read_to_string("prices.txt") {
            Ok(file) => text = file,
            Err(err) => print_warning(format!("Failed to read prices.txt: {}", err).as_str())
        }
    } else {
        match lookup_on_wiki() {
            Ok(response) => text = extract_from_response(&response),
            Err(_) => print_warning(format!("Failed to fetch prices").as_str())
        }
    }

    let mut prices = string_to_map(&text);
    for mat in MaterialName::iter() {
        if !prices.contains_key(&mat) {
            print_warning(format!("Price missing for '{}'", mat).as_str());
            prices.insert(mat, 0.0);
        }
    }

    text = prices.iter().map(|(name, value)| format!("{}: {},", name, value)).join("\n");
    fs::write("prices.txt", text).unwrap_or_else(|err| {
        print_warning(format!("Failed to save prices.txt: {}", err).as_str());
    });

    return prices;
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

fn extract_from_response(response: &String) -> String {
    let json: Value = serde_json::from_str(response.as_str()).unwrap_or_default();
    let text= &json["parse"]["text"];

    if let Value::String(text) = text {
        let re = Regex::new(r"^.+?<p>((.|\n)+?)</p>").unwrap();
        let content = re.captures(&text);

        if let Some(content) = content {
            return content[1].to_string();
        } else {
            print_warning("Unexpected response from Runescape.wiki");
        }
    } else {
        print_warning("Unexpected response from Runescape.wiki");
    }

    return String::new()
}

fn string_to_map(text: &String) -> HashMap<MaterialName, f64> {
    let mut prices = HashMap::new();
    let re = Regex::new(r"^([^:]+): ?([\d\.]+)").unwrap();
    let lines = text.split("\n");

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