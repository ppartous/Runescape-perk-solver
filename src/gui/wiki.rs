use dioxus::prelude::*;
use itertools::Itertools;
use perk_solver::prelude::*;

fn wiki_image_link(name: &str) -> String {
    format!(
        "https://runescape.wiki/images/{}.png?00000",
        name.to_string().replace(" ", "_")
    )
}

pub fn WikiImage<'a>(cx: Scope<'a>, name: &str) -> Element<'a> {
    cx.render(rsx!(div {
        class: "inline-mat-image",
        img {
            src: "{wiki_image_link(name)}",
            title: "{name}",
        }
    }))
}

pub fn make_wiki_calc_link(
    comb: &[MaterialName],
    ancient: bool,
    gizmo_type: GizmoType,
    level: u8,
) -> String {
    let ancient = if ancient { "&a=1" } else { "" };
    let gizmo_type = match gizmo_type {
        GizmoType::Weapon => "g=1",
        GizmoType::Armour => "g=2",
        GizmoType::Tool => "g=3",
    };

    let mut comb = perk_solver::result::gizmo_combination_sort(comb)
        .into_iter()
        .map(mat_link_id)
        .collect_vec();
    comb.resize(9, "0");
    comb.swap(0, 1); // Adjust order to match the wiki
    comb.swap(1, 2);

    format!(
        "https://runescape.wiki/w/Calculator:Perks#{gizmo_type}&l={level}{ancient}&m={}",
        comb.into_iter().join(",")
    )
}

fn mat_link_id(mat: MaterialName) -> &'static str {
    match mat {
        MaterialName::BaseParts => "1",
        MaterialName::BladeParts => "2",
        MaterialName::ClearParts => "3",
        MaterialName::ConnectorParts => "4",
        MaterialName::CoverParts => "5",
        MaterialName::CraftedParts => "6",
        MaterialName::CrystalParts => "7",
        MaterialName::DeflectingParts => "8",
        MaterialName::DelicateParts => "9",
        MaterialName::FlexibleParts => "10",
        MaterialName::HeadParts => "11",
        MaterialName::MagicParts => "12",
        MaterialName::MetallicParts => "13",
        MaterialName::OrganicParts => "14",
        MaterialName::PaddedParts => "15",
        MaterialName::PlatedParts => "16",
        MaterialName::SimpleParts => "17",
        MaterialName::SmoothParts => "18",
        MaterialName::SpikedParts => "19",
        MaterialName::SpiritualParts => "20",
        MaterialName::StaveParts => "21",
        MaterialName::TensileParts => "22",
        MaterialName::DextrousComponents => "23",
        MaterialName::DirectComponents => "24",
        MaterialName::EnhancingComponents => "25",
        MaterialName::EtherealComponents => "26",
        MaterialName::EvasiveComponents => "27",
        MaterialName::HealthyComponents => "28",
        MaterialName::HeavyComponents => "29",
        MaterialName::ImbuedComponents => "30",
        MaterialName::LightComponents => "31",
        MaterialName::LivingComponents => "32",
        MaterialName::PiousComponents => "33",
        MaterialName::PowerfulComponents => "34",
        MaterialName::PreciousComponents => "35",
        MaterialName::PreciseComponents => "36",
        MaterialName::ProtectiveComponents => "37",
        MaterialName::RefinedComponents => "38",
        MaterialName::SharpComponents => "39",
        MaterialName::StrongComponents => "40",
        MaterialName::StunningComponents => "41",
        MaterialName::SubtleComponents => "42",
        MaterialName::SwiftComponents => "43",
        MaterialName::VariableComponents => "44",
        MaterialName::ArmadylComponents => "45",
        MaterialName::AscendedComponents => "46",
        MaterialName::AvernicComponents => "47",
        MaterialName::BandosComponents => "48",
        MaterialName::BrassicanComponents => "49",
        MaterialName::ClockworkComponents => "50",
        MaterialName::CorporealComponents => "51",
        MaterialName::CulinaryComponents => "52",
        MaterialName::CywirComponents => "53",
        MaterialName::DragonfireComponents => "54",
        MaterialName::ExplosiveComponents => "55",
        MaterialName::FacetedComponents => "56",
        MaterialName::FortunateComponents => "57",
        MaterialName::FungalComponents => "58",
        MaterialName::HarnessedComponents => "59",
        MaterialName::IlujankanComponents => "60",
        MaterialName::KnightlyComponents => "61",
        MaterialName::NoxiousComponents => "62",
        MaterialName::OceanicComponents => "63",
        MaterialName::PestiferousComponents => "64",
        MaterialName::ResilientComponents => "65",
        MaterialName::RumblingComponents => "66",
        MaterialName::SaradominComponents => "67",
        MaterialName::SerenComponents => "68",
        MaterialName::ShadowComponents => "69",
        MaterialName::ShiftingComponents => "70",
        MaterialName::SilentComponents => "71",
        MaterialName::ThirdAgeComponents => "72",
        MaterialName::UndeadComponents => "73",
        MaterialName::ZamorakComponents => "74",
        MaterialName::ZarosComponents => "75",
        MaterialName::ClassicComponents => "76",
        MaterialName::HistoricComponents => "77",
        MaterialName::TimewornComponents => "78",
        MaterialName::VintageComponents => "79",
        MaterialName::OffcutComponents => "80",
        MaterialName::ManufacturedComponents => "81",
        _ => "0",
    }
}
