use crate::{prelude::*, utils::print_warning};
use itertools::Itertools;
use regex::Regex;
use serde_json::{self, Value};
use std::{fs, str::FromStr, sync::RwLock};
use strum::EnumCount;

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

#[derive(PartialEq)]
pub enum PriceSource {
    Online,
    Local,
    Cache,
}

pub fn calc_gizmo_price(mat_combination: &[MaterialName], prob_gizmo: f64) -> f64 {
    let shell_price = *SHELL_PRICE.read().unwrap();
    let prices = PRICES.read().unwrap();
    if let Some(prices) = prices.as_ref() {
        let price = shell_price
            + mat_combination
                .iter()
                .fold(0.0, |acc, x| acc + prices.get(*x));

        price / prob_gizmo
    } else {
        0.0
    }
}

pub fn load_component_prices(
    local_price_file_path: &Option<String>,
    prefer_online: bool,
) -> Result<PriceSource, String> {
    let mut source = PriceSource::Cache;
    let mut prices = None;

    // Don't need to set prices again if the calc is invoked multiple times
    if PRICES.read().unwrap().is_some() {
        return Ok(source);
    }

    if prefer_online {
        prices = lookup_on_wiki().ok();
        source = PriceSource::Online;
    }

    if let Some(file_path) = local_price_file_path {
        if prices.is_none() && std::path::Path::new(file_path).exists() {
            prices = Some(load_from_local_file(file_path)?);
            source = PriceSource::Local;
        }
    }

    if prices.is_none() {
        prices = Some(lookup_on_wiki()?);
        source = PriceSource::Online;
    }

    if let Some(file_path) = local_price_file_path {
        if let Some(prices) = &prices {
            let text = prices
                .iter()
                .map(|(name, value)| format!("{}: {},", name, value))
                .sorted()
                .join("\n");

            fs::write(&file_path, text).unwrap_or_else(|err| {
                print_warning(format!("Failed to save {}: {}", file_path, err).as_str());
            });
        }
    }

    *PRICES.write().unwrap() = prices;
    Ok(source)
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

fn lookup_on_wiki() -> Result<PriceMap, &'static str> {
    fn _lookup() -> Result<String, reqwest::Error> {
        println!("Fetching component prices from Runescape.wiki...");
        let client = reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?;

        // use p._all_prices() from: https://runescape.wiki/w/Module:Component_costs
        let url = format!(
            "https://runescape.wiki/api.php?{}",
            form_urlencoded::Serializer::new(String::new())
                .append_pair("action", "parse")
                .append_pair("text", "{{#invoke:Component costs|_all_prices}}")
                .append_pair("contentmodel", "wikitext")
                .append_pair("prop", "text")
                .append_pair("disablelimitreport", "")
                .append_pair("wrapoutputclass", "")
                .append_pair("format", "json")
                .append_pair("formatversion", "2")
                .finish()
        );
        let body = client.get(url).send()?.text()?;

        Ok(body)
    }

    match _lookup() {
        Ok(response) => Ok(string_to_map(&extract_from_response(&response)?)),
        Err(_) => Err("Failed to fetch prices"),
    }
}

fn extract_from_response(response: &str) -> Result<String, &'static str> {
    let json: Value = serde_json::from_str(response).unwrap_or_default();
    let text = &json["parse"]["text"];

    if let Value::String(text) = text {
        // strip <p>..</p> or <div ...>...</div> wrappers from MediaWiki parse response
        let re = Regex::new(r"</?(p|div)[^>]*>").unwrap();
        Ok(re.replace_all(text, "").into_owned())
    } else {
        return Err("Unexpected response from Runescape.wiki");
    }
}

fn load_from_local_file(file_path: &str) -> Result<PriceMap, String> {
    let text = match fs::read_to_string(&file_path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to read {}: {}", file_path, err)),
    };

    Ok(string_to_map(&text))
}

fn string_to_map(text: &str) -> PriceMap {
    let mut prices = PriceMap::new();

    // match "(Component): (Price)" lines
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
