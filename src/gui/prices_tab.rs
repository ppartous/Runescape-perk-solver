use crate::wiki;
use dioxus::prelude::*;
use perk_solver::prelude::*;

pub fn PricesTab<'a>(
    cx: Scope<'a>,
    prices_status: &'a UseState<Option<Result<(), String>>>,
) -> Element<'a> {
    let shell_price_update_check = use_state(cx, || ());

    cx.render(rsx!(
        if let Some(status) = prices_status.get() {
            rsx!(
                button {
                    r#type: "button",
                    class: "btn btn-primary",
                    float: "right",
                    onclick: move |_| {
                        *perk_solver::component_prices::PRICES.write().unwrap() = None;
                        prices_status.set(None);
                    },
                    "Reload"
                }
                if let Err(err) = status {
                    rsx!("{err}")
                } else {
                    rsx!(
                        h3 { "Common materials" }
                        div {
                            class: "prices-container",
                            table {
                                for mat in COMMON_MATERIALS.iter() {
                                    PriceTabElement(cx, *mat, &shell_price_update_check)
                                }
                            }
                        }
                        h3 { "Uncommon materials" }
                        div {
                            class: "prices-container",
                            table {
                                for mat in UNCOMMON_MATERIALS.iter() {
                                    PriceTabElement(cx, *mat, &shell_price_update_check)
                                }
                            }
                        }
                        h3 { "Rare materials" }
                        div {
                            class: "prices-container",
                            table {
                                for mat in RARE_MATERIALS.iter() {
                                    PriceTabElement(cx, *mat, &shell_price_update_check)
                                }
                            }
                        }
                        h3 { "Gizmo shells" }
                        div { "Prices are calculated based on the material prices above." }
                        table {
                            class: "wikitable align-left-1",
                            tr {
                                th { "Shell" }
                                th { "Price" }
                                th { "Source materials" }
                            }
                            PriceTabShellElement(cx, GizmoType::Weapon, false,
                                &[(10, MaterialName::BladeParts), (5, MaterialName::CraftedParts), (2, MaterialName::StrongComponents)])
                            PriceTabShellElement(cx, GizmoType::Armour, false,
                                &[(10, MaterialName::DeflectingParts), (5, MaterialName::CraftedParts), (2, MaterialName::ProtectiveComponents)])
                            PriceTabShellElement(cx, GizmoType::Tool, false,
                                &[(10, MaterialName::HeadParts), (5, MaterialName::CraftedParts), (2, MaterialName::PreciseComponents)])
                            PriceTabShellElement(cx, GizmoType::Weapon, true,
                                &[(20, MaterialName::BladeParts), (20, MaterialName::HistoricComponents), (2, MaterialName::ClassicComponents), (2, MaterialName::StrongComponents)])
                            PriceTabShellElement(cx, GizmoType::Armour, true,
                                &[(20, MaterialName::DeflectingParts), (20, MaterialName::HistoricComponents), (2, MaterialName::ClassicComponents), (2, MaterialName::ProtectiveComponents)])
                            PriceTabShellElement(cx, GizmoType::Tool, true,
                                &[(20, MaterialName::HeadParts), (20, MaterialName::HistoricComponents), (2, MaterialName::ClassicComponents), (2, MaterialName::PreciseComponents)])
                        }
                    )
                }
            )
        } else {
            rsx!("Fetching component prices from Runescape.wiki...")
        }
    ))
}

fn PriceTabShellElement<'a>(
    cx: Scope<'a>,
    shell: GizmoType,
    ancient: bool,
    comb: &[(u8, MaterialName)],
) -> Element<'a> {
    let name = if ancient {
        format!("Ancient {} gizmo shell", shell.to_string().to_lowercase())
    } else {
        format!("{} gizmo shell", shell.to_string())
    };

    cx.render(rsx!(
        tr {
            td {
                wiki::WikiImage(cx, name.as_str())
                "{name}"
            }
            td { format!("{:.0}", perk_solver::component_prices::get_shell_price(shell, ancient)) }
            td {
                for (i, (n, mat)) in comb.iter().enumerate() {
                    if i > 0 {
                        rsx!(", ")
                    }
                    "{n} × "
                    wiki::WikiImage(cx, mat.to_str())
                }
            }
        }
    ))
}

fn PriceTabElement<'a>(
    cx: Scope<'a>,
    mat: MaterialName,
    update_check: &'a UseState<()>,
) -> Element<'a> {
    cx.render(rsx!(
        tr {
            td {
                wiki::WikiImage(cx, mat.to_str()),
                "{mat}:"
            }
            td {
                input {
                    r#type: "number",
                    min: "0",
                    value: *perk_solver::component_prices::PRICES.read().unwrap().as_ref().unwrap().get(mat),
                    oninput: move |ev| {
                        if let Some(prices) = perk_solver::component_prices::PRICES.write().unwrap().as_mut() {
                            *prices.get_mut(mat) = ev.value.parse().unwrap_or(0.0);
                            update_check.needs_update();
                        }
                    }
                }
            }
        }
    ))
}
