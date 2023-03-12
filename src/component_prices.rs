use itertools::Itertools;
use serde_json::{self, Value};
use crate::{utils::print_warning, prelude::*};
use std::{str::FromStr, fs};
use regex::Regex;
use once_cell::sync::OnceCell;
use strum::{EnumCount, IntoEnumIterator};

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    ";",
    env!("CARGO_PKG_REPOSITORY")
);

type PriceMap = StackMap<MaterialName, f64, {MaterialName::COUNT}>;

static SHELL_PRICE: OnceCell<f64> = OnceCell::new();
static PRICES: OnceCell<PriceMap> = OnceCell::new();

pub fn calc_gizmo_price(line: &ResultLine) -> f64 {
    let shell_price = SHELL_PRICE.get().unwrap();
    let prices = PRICES.get().unwrap();
    let price = shell_price + line.mat_combination.iter().fold(0.0, |acc, x| {
        acc + prices.get(*x)
    });

    price / line.prob_gizmo
}

pub fn load_component_prices(args: &Args) -> Result<(), String> {
    let mut text = String::new();

    if args.price_file != "false" && std::path::Path::new(&args.price_file).exists() {
        match fs::read_to_string(&args.price_file) {
            Ok(file) => text = file,
            Err(err) => return Err(format!("Failed to read {}: {}", args.price_file, err))
        }
    } else {
        if let Ok(response) = lookup_on_wiki() {
            text = extract_from_response(&response)?;
        } else if args.sort_type == SortType::Price {
            return Err("Failed to fetch prices".to_string());
        } else {
            print_warning("Failed to fetch prices")
        }
    }

    let prices = string_to_map(&text);
    for mat in MaterialName::iter() {
        if *prices.get(mat) == 0.0 {
            print_warning(format!("Price missing for '{}'", mat).as_str());
        };
    }

    if args.price_file != "false" {
        text = prices.iter().map(|(name, value)| format!("{}: {},", name, value)).sorted().join("\n");
        fs::write(&args.price_file, text).unwrap_or_else(|err| {
            print_warning(format!("Failed to save {}: {}", args.price_file, err).as_str());
        });
    }

    SHELL_PRICE.set(calc_shell_price(args, &prices)).ok();
    PRICES.set(prices).ok();
    Ok(())
}

fn calc_shell_price(args: &Args, prices: &PriceMap) -> f64 {
    match args.ancient {
        true => match args.gizmo_type {
            GizmoType::Armour => 20.0 * prices.get(MaterialName::DeflectingParts) + 20.0 * prices.get(MaterialName::HistoricComponents) + 2.0 * prices.get(MaterialName::ClassicComponents) + 2.0 * prices.get(MaterialName::ProtectiveComponents),
            GizmoType::Weapon => 20.0 * prices.get(MaterialName::BladeParts) + 20.0 * prices.get(MaterialName::HistoricComponents) + 2.0 * prices.get(MaterialName::ClassicComponents) + 2.0 * prices.get(MaterialName::StrongComponents),
            GizmoType::Tool => 20.0 * prices.get(MaterialName::HeadParts) + 20.0 * prices.get(MaterialName::HistoricComponents) + 2.0 * prices.get(MaterialName::ClassicComponents) + 2.0 * prices.get(MaterialName::PreciseComponents),
        },
        false => match args.gizmo_type {
            GizmoType::Armour => 10.0 * prices.get(MaterialName::DeflectingParts) + 5.0 * prices.get(MaterialName::CraftedParts) + 2.0 * prices.get(MaterialName::ProtectiveComponents),
            GizmoType::Weapon => 10.0 * prices.get(MaterialName::BladeParts) + 5.0 * prices.get(MaterialName::CraftedParts) + 2.0 * prices.get(MaterialName::StrongComponents),
            GizmoType::Tool => 10.0 * prices.get(MaterialName::HeadParts) + 5.0 * prices.get(MaterialName::CraftedParts) + 2.0 * prices.get(MaterialName::PreciseComponents),
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

fn extract_from_response(response: &str) -> Result<String, String> {
    let json: Value = serde_json::from_str(response).unwrap_or_default();
    let text= &json["parse"]["text"];

    if let Value::String(text) = text {
        let re = Regex::new(r"^.+?<p>((.|\n)+?)</p>").unwrap();
        let content = re.captures(text);

        if let Some(content) = content {
            return Ok(content[1].to_string());
        } else {
            return Err("Unexpected response from Runescape.wiki".to_string());
        }
    } else {
        return Err("Unexpected response from Runescape.wiki".to_string());
    }
}

fn string_to_map(text: &str) -> PriceMap {
    let mut prices = PriceMap::new();
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