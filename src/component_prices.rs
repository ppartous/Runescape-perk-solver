use crate::{prelude::*, utils::print_warning};
use itertools::Itertools;
use regex::Regex;
use serde_json::{self, Value};
use std::{fs, str::FromStr, sync::RwLock};
use strum::{EnumCount, IntoEnumIterator};

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    ";",
    env!("CARGO_PKG_REPOSITORY")
);

type PriceMap = StackMap<MaterialName, f64, { MaterialName::COUNT }>;

pub static PRICES: RwLock<Option<PriceMap>> = RwLock::new(None);
static SHELL_PRICE: RwLock<f64> = RwLock::new(0.0);

pub fn calc_gizmo_price(mat_combination: &[MaterialName], prob_gizmo: f64) -> f64 {
    let shell_price = *SHELL_PRICE.read().unwrap();
    let prices = PRICES.read().unwrap();
    let prices = prices.as_ref().unwrap();
    let price = shell_price
        + mat_combination
            .iter()
            .fold(0.0, |acc, x| acc + prices.get(*x));

    price / prob_gizmo
}

pub fn load_component_prices(price_file: &str) -> Result<(), String> {
    // Don't need to set prices again if the calc is invoked multiple times
    if PRICES.read().unwrap().is_some() {
        return Ok(());
    }

    let mut text;

    if price_file != "false" && std::path::Path::new(&price_file).exists() {
        match fs::read_to_string(&price_file) {
            Ok(file) => text = file,
            Err(err) => return Err(format!("Failed to read {}: {}", price_file, err)),
        }
    } else {
        if let Ok(response) = lookup_on_wiki() {
            text = extract_from_response(&response)?;
        } else {
            return Err("Failed to fetch prices".to_string());
        }
    }

    let prices = string_to_map(&text);
    for mat in MaterialName::iter() {
        if *prices.get(mat) == 0.0 {
            print_warning(format!("Price missing for '{}'", mat).as_str());
        };
    }

    if price_file != "false" {
        text = prices
            .iter()
            .map(|(name, value)| format!("{}: {},", name, value))
            .sorted()
            .join("\n");
        fs::write(&price_file, text).unwrap_or_else(|err| {
            print_warning(format!("Failed to save {}: {}", price_file, err).as_str());
        });
    }

    *PRICES.write().unwrap() = Some(prices);
    Ok(())
}

pub fn get_shell_price(gizmo_type: GizmoType, ancient: bool) -> f64 {
    let prices = PRICES.read().unwrap();
    let prices = prices.as_ref().unwrap();
    match ancient {
        true => match gizmo_type {
            GizmoType::Armour => {
                20.0 * prices.get(MaterialName::DeflectingParts)
                    + 20.0 * prices.get(MaterialName::HistoricComponents)
                    + 2.0 * prices.get(MaterialName::ClassicComponents)
                    + 2.0 * prices.get(MaterialName::ProtectiveComponents)
            }
            GizmoType::Weapon => {
                20.0 * prices.get(MaterialName::BladeParts)
                    + 20.0 * prices.get(MaterialName::HistoricComponents)
                    + 2.0 * prices.get(MaterialName::ClassicComponents)
                    + 2.0 * prices.get(MaterialName::StrongComponents)
            }
            GizmoType::Tool => {
                20.0 * prices.get(MaterialName::HeadParts)
                    + 20.0 * prices.get(MaterialName::HistoricComponents)
                    + 2.0 * prices.get(MaterialName::ClassicComponents)
                    + 2.0 * prices.get(MaterialName::PreciseComponents)
            }
        },
        false => match gizmo_type {
            GizmoType::Armour => {
                10.0 * prices.get(MaterialName::DeflectingParts)
                    + 5.0 * prices.get(MaterialName::CraftedParts)
                    + 2.0 * prices.get(MaterialName::ProtectiveComponents)
            }
            GizmoType::Weapon => {
                10.0 * prices.get(MaterialName::BladeParts)
                    + 5.0 * prices.get(MaterialName::CraftedParts)
                    + 2.0 * prices.get(MaterialName::StrongComponents)
            }
            GizmoType::Tool => {
                10.0 * prices.get(MaterialName::HeadParts)
                    + 5.0 * prices.get(MaterialName::CraftedParts)
                    + 2.0 * prices.get(MaterialName::PreciseComponents)
            }
        },
    }
}

pub fn set_shell_price(gizmo_type: GizmoType, ancient: bool) {
    *SHELL_PRICE.write().unwrap() = get_shell_price(gizmo_type, ancient);
}

fn lookup_on_wiki() -> Result<String, reqwest::Error> {
    println!("Fetching component prices from Runescape.wiki...");
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    let url = format!("https://runescape.wiki/api.php?action=scribunto-console&format=json&title=sandbox&clear=true&content=&question={}",
        include_str!("./component_prices.lua")
    );
    let body = client.get(url).send()?.text()?;

    Ok(body)
}

fn extract_from_response(response: &str) -> Result<String, String> {
    let json: Value = serde_json::from_str(response).unwrap_or_default();
    let text = &json["print"];

    if let Value::String(text) = text {
        Ok(text.clone())
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
