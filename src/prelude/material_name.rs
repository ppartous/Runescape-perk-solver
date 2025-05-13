use colored::*;
use itertools::Itertools;
use std::str::FromStr;
use strum::{Display, EnumIter, IntoEnumIterator};
use strum_macros::{EnumCount, EnumVariantNames, IntoStaticStr};

#[derive(
    Debug,
    Display,
    Default,
    PartialEq,
    Eq,
    Clone,
    Copy,
    PartialOrd,
    Ord,
    Hash,
    EnumCount,
    EnumIter,
    EnumVariantNames,
    IntoStaticStr,
)]
#[strum(ascii_case_insensitive, use_phf)]
pub enum MaterialName {
    #[strum(serialize = "Armadyl components")]
    ArmadylComponents,
    #[strum(serialize = "Ascended components")]
    AscendedComponents,
    #[strum(serialize = "Avernic components")]
    AvernicComponents,
    #[strum(serialize = "Bandos components")]
    BandosComponents,
    #[strum(serialize = "Base parts")]
    BaseParts,
    #[strum(serialize = "Blade parts")]
    BladeParts,
    #[strum(serialize = "Brassican components")]
    BrassicanComponents,
    #[strum(serialize = "Classic components")]
    ClassicComponents,
    #[strum(serialize = "Clear parts")]
    ClearParts,
    #[strum(serialize = "Clockwork components")]
    ClockworkComponents,
    #[strum(serialize = "Connector parts")]
    ConnectorParts,
    #[strum(serialize = "Corporeal components")]
    CorporealComponents,
    #[strum(serialize = "Cover parts")]
    CoverParts,
    #[strum(serialize = "Crafted parts")]
    CraftedParts,
    #[strum(serialize = "Crystal parts")]
    CrystalParts,
    #[strum(serialize = "Culinary components")]
    CulinaryComponents,
    #[strum(serialize = "Cywir components")]
    CywirComponents,
    #[strum(serialize = "Deflecting parts")]
    DeflectingParts,
    #[strum(serialize = "Delicate parts")]
    DelicateParts,
    #[strum(serialize = "Dextrous components")]
    DextrousComponents,
    #[strum(serialize = "Direct components")]
    DirectComponents,
    #[strum(serialize = "Dragonfire components")]
    DragonfireComponents,
    #[strum(serialize = "Enhancing components")]
    EnhancingComponents,
    #[strum(serialize = "Ethereal components")]
    EtherealComponents,
    #[strum(serialize = "Evasive components")]
    EvasiveComponents,
    #[strum(serialize = "Explosive components")]
    ExplosiveComponents,
    #[strum(serialize = "Faceted components")]
    FacetedComponents,
    #[strum(serialize = "Flexible parts")]
    FlexibleParts,
    #[strum(serialize = "Fortunate components")]
    FortunateComponents,
    #[strum(serialize = "Fungal components")]
    FungalComponents,
    #[strum(serialize = "Harnessed components")]
    HarnessedComponents,
    #[strum(serialize = "Head parts")]
    HeadParts,
    #[strum(serialize = "Healthy components")]
    HealthyComponents,
    #[strum(serialize = "Heavy components")]
    HeavyComponents,
    #[strum(serialize = "Historic components")]
    HistoricComponents,
    #[strum(serialize = "Ilujankan components")]
    IlujankanComponents,
    #[strum(serialize = "Imbued components")]
    ImbuedComponents,
    #[default]
    Junk,
    #[strum(serialize = "Knightly components")]
    KnightlyComponents,
    #[strum(serialize = "Light components")]
    LightComponents,
    #[strum(serialize = "Living components")]
    LivingComponents,
    #[strum(serialize = "Magic parts")]
    MagicParts,
    #[strum(serialize = "Manufactured components")]
    ManufacturedComponents,
    #[strum(serialize = "Metallic parts")]
    MetallicParts,
    #[strum(serialize = "Noxious components")]
    NoxiousComponents,
    #[strum(serialize = "Oceanic components")]
    OceanicComponents,
    #[strum(serialize = "Offcut components")]
    OffcutComponents,
    #[strum(serialize = "Organic parts")]
    OrganicParts,
    #[strum(serialize = "Padded parts")]
    PaddedParts,
    #[strum(serialize = "Pestiferous components")]
    PestiferousComponents,
    #[strum(serialize = "Pious components")]
    PiousComponents,
    #[strum(serialize = "Plated parts")]
    PlatedParts,
    #[strum(serialize = "Powerful components")]
    PowerfulComponents,
    #[strum(serialize = "Precious components")]
    PreciousComponents,
    #[strum(serialize = "Precise components")]
    PreciseComponents,
    #[strum(serialize = "Protective components")]
    ProtectiveComponents,
    #[strum(serialize = "Refined components")]
    RefinedComponents,
    #[strum(serialize = "Resilient components")]
    ResilientComponents,
    #[strum(serialize = "Rumbling components")]
    RumblingComponents,
    #[strum(serialize = "Saradomin components")]
    SaradominComponents,
    #[strum(serialize = "Seren components")]
    SerenComponents,
    #[strum(serialize = "Shadow components")]
    ShadowComponents,
    #[strum(serialize = "Sharp components")]
    SharpComponents,
    #[strum(serialize = "Shifting components")]
    ShiftingComponents,
    #[strum(serialize = "Silent components")]
    SilentComponents,
    #[strum(serialize = "Simple parts")]
    SimpleParts,
    #[strum(serialize = "Smooth parts")]
    SmoothParts,
    #[strum(serialize = "Spiked parts")]
    SpikedParts,
    #[strum(serialize = "Spiritual parts")]
    SpiritualParts,
    #[strum(serialize = "Stave parts")]
    StaveParts,
    #[strum(serialize = "Strong components")]
    StrongComponents,
    #[strum(serialize = "Stunning components")]
    StunningComponents,
    #[strum(serialize = "Subtle components")]
    SubtleComponents,
    #[strum(serialize = "Swift components")]
    SwiftComponents,
    #[strum(serialize = "Tensile parts")]
    TensileParts,
    #[strum(serialize = "Third-age components")]
    ThirdAgeComponents,
    #[strum(serialize = "Timeworn components")]
    TimewornComponents,
    #[strum(serialize = "Undead components")]
    UndeadComponents,
    #[strum(serialize = "Variable components")]
    VariableComponents,
    #[strum(serialize = "Vintage components")]
    VintageComponents,
    #[strum(serialize = "Zamorak components")]
    ZamorakComponents,
    #[strum(serialize = "Zaros components")]
    ZarosComponents,
}

impl MaterialName {
    pub fn vec_to_string(v: &[MaterialName]) -> String {
        let counts = v.iter().counts();
        v.iter()
            .unique()
            .map(|mat| {
                let count = *counts.get(mat).unwrap();
                if COMMON_MATERIALS.contains(mat) {
                    format!("{} × 5 {}", count, mat.to_string().cyan())
                } else {
                    format!("{} × {}", count, mat.to_string().cyan())
                }
            })
            .join(", ")
    }

    pub fn to_str(&self) -> &'static str {
        self.into()
    }
}

impl serde::Serialize for MaterialName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self)
    }
}

impl FromStr for MaterialName {
    type Err = &'static str;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        for mat in MaterialName::iter() {
            if mat
                .to_string()
                .to_lowercase()
                .contains(str.to_lowercase().as_str())
            {
                return Ok(mat);
            }
        }
        Err("Unknown material name")
    }
}

impl From<MaterialName> for usize {
    fn from(value: MaterialName) -> Self {
        value as usize
    }
}

pub static COMMON_MATERIALS: &[MaterialName] = &[
    MaterialName::BaseParts,
    MaterialName::BladeParts,
    MaterialName::ClearParts,
    MaterialName::ConnectorParts,
    MaterialName::CoverParts,
    MaterialName::CraftedParts,
    MaterialName::CrystalParts,
    MaterialName::DeflectingParts,
    MaterialName::DelicateParts,
    MaterialName::FlexibleParts,
    MaterialName::HeadParts,
    MaterialName::MagicParts,
    MaterialName::MetallicParts,
    MaterialName::OrganicParts,
    MaterialName::PaddedParts,
    MaterialName::PlatedParts,
    MaterialName::SimpleParts,
    MaterialName::SmoothParts,
    MaterialName::SpikedParts,
    MaterialName::SpiritualParts,
    MaterialName::StaveParts,
    MaterialName::TensileParts,
];

pub static UNCOMMON_MATERIALS: &[MaterialName] = &[
    MaterialName::DextrousComponents,
    MaterialName::DirectComponents,
    MaterialName::EnhancingComponents,
    MaterialName::EtherealComponents,
    MaterialName::EvasiveComponents,
    MaterialName::HealthyComponents,
    MaterialName::HeavyComponents,
    MaterialName::ImbuedComponents,
    MaterialName::LightComponents,
    MaterialName::LivingComponents,
    MaterialName::OffcutComponents,
    MaterialName::PiousComponents,
    MaterialName::PowerfulComponents,
    MaterialName::PreciousComponents,
    MaterialName::PreciseComponents,
    MaterialName::ProtectiveComponents,
    MaterialName::RefinedComponents,
    MaterialName::SharpComponents,
    MaterialName::StrongComponents,
    MaterialName::StunningComponents,
    MaterialName::SubtleComponents,
    MaterialName::SwiftComponents,
    MaterialName::VariableComponents,
];

pub static RARE_MATERIALS: &[MaterialName] = &[
    MaterialName::ArmadylComponents,
    MaterialName::AscendedComponents,
    MaterialName::AvernicComponents,
    MaterialName::BandosComponents,
    MaterialName::BrassicanComponents,
    MaterialName::ClassicComponents,
    MaterialName::ClockworkComponents,
    MaterialName::CorporealComponents,
    MaterialName::CulinaryComponents,
    MaterialName::CywirComponents,
    MaterialName::DragonfireComponents,
    MaterialName::ExplosiveComponents,
    MaterialName::FacetedComponents,
    MaterialName::FortunateComponents,
    MaterialName::FungalComponents,
    MaterialName::HarnessedComponents,
    MaterialName::HistoricComponents,
    MaterialName::IlujankanComponents,
    MaterialName::KnightlyComponents,
    MaterialName::ManufacturedComponents,
    MaterialName::NoxiousComponents,
    MaterialName::OceanicComponents,
    MaterialName::PestiferousComponents,
    MaterialName::ResilientComponents,
    MaterialName::RumblingComponents,
    MaterialName::SaradominComponents,
    MaterialName::SerenComponents,
    MaterialName::ShadowComponents,
    MaterialName::ShiftingComponents,
    MaterialName::SilentComponents,
    MaterialName::ThirdAgeComponents,
    MaterialName::TimewornComponents,
    MaterialName::UndeadComponents,
    MaterialName::VintageComponents,
    MaterialName::ZamorakComponents,
    MaterialName::ZarosComponents,
];
