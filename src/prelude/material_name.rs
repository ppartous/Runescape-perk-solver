use colored::*;
use itertools::Itertools;
use std::str::FromStr;
use strum::{Display, EnumIter};
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
#[strum(serialize_all = "title_case", ascii_case_insensitive, use_phf)]
pub enum MaterialName {
    ArmadylComponents,
    AscendedComponents,
    AvernicComponents,
    BandosComponents,
    BaseParts,
    BladeParts,
    BrassicanComponents,
    ClassicComponents,
    ClearParts,
    ClockworkComponents,
    ConnectorParts,
    CorporealComponents,
    CoverParts,
    CraftedParts,
    CrystalParts,
    CulinaryComponents,
    CywirComponents,
    DeflectingParts,
    DelicateParts,
    DextrousComponents,
    DirectComponents,
    DragonfireComponents,
    EnhancingComponents,
    EtherealComponents,
    EvasiveComponents,
    ExplosiveComponents,
    FacetedComponents,
    FlexibleParts,
    FortunateComponents,
    FungalComponents,
    HarnessedComponents,
    HeadParts,
    HealthyComponents,
    HeavyComponents,
    HistoricComponents,
    IlujankanComponents,
    ImbuedComponents,
    #[default]
    Junk,
    KnightlyComponents,
    LightComponents,
    LivingComponents,
    MagicParts,
    MetallicParts,
    NoxiousComponents,
    OceanicComponents,
    OrganicParts,
    PaddedParts,
    PestiferousComponents,
    PiousComponents,
    PlatedParts,
    PowerfulComponents,
    PreciousComponents,
    PreciseComponents,
    ProtectiveComponents,
    RefinedComponents,
    ResilientComponents,
    RumblingComponents,
    SaradominComponents,
    SerenComponents,
    ShadowComponents,
    SharpComponents,
    ShiftingComponents,
    SilentComponents,
    SimpleParts,
    SmoothParts,
    SpikedParts,
    SpiritualParts,
    StaveParts,
    StrongComponents,
    StunningComponents,
    SubtleComponents,
    SwiftComponents,
    TensileParts,
    #[strum(serialize = "Third-age components")]
    ThirdAgeComponents,
    TimewornComponents,
    UndeadComponents,
    VariableComponents,
    VintageComponents,
    ZamorakComponents,
    ZarosComponents,
}

impl MaterialName {
    pub fn vec_to_string(v: &[MaterialName]) -> String {
        let counts = v.iter().counts();
        v.iter()
            .unique()
            .map(|x| {
                let count = *counts.get(x).unwrap();
                format!("{} × {}", count, x.to_string().cyan())
            })
            .join(", ")
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

    fn from_str(mat: &str) -> Result<Self, Self::Err> {
        fn find(mat: &str) -> Result<MaterialName, &'static str> {
            match mat.to_lowercase().as_str() {
                "armadyl components" => Ok(MaterialName::ArmadylComponents),
                "ascended components" => Ok(MaterialName::AscendedComponents),
                "avernic components" => Ok(MaterialName::AvernicComponents),
                "bandos components" => Ok(MaterialName::BandosComponents),
                "base parts" => Ok(MaterialName::BaseParts),
                "blade parts" => Ok(MaterialName::BladeParts),
                "brassican components" => Ok(MaterialName::BrassicanComponents),
                "classic components" => Ok(MaterialName::ClassicComponents),
                "clear parts" => Ok(MaterialName::ClearParts),
                "clockwork components" => Ok(MaterialName::ClockworkComponents),
                "connector parts" => Ok(MaterialName::ConnectorParts),
                "corporeal components" => Ok(MaterialName::CorporealComponents),
                "cover parts" => Ok(MaterialName::CoverParts),
                "crafted parts" => Ok(MaterialName::CraftedParts),
                "crystal parts" => Ok(MaterialName::CrystalParts),
                "culinary components" => Ok(MaterialName::CulinaryComponents),
                "cywir components" => Ok(MaterialName::CywirComponents),
                "deflecting parts" => Ok(MaterialName::DeflectingParts),
                "delicate parts" => Ok(MaterialName::DelicateParts),
                "dextrous components" => Ok(MaterialName::DextrousComponents),
                "direct components" => Ok(MaterialName::DirectComponents),
                "dragonfire components" => Ok(MaterialName::DragonfireComponents),
                "enhancing components" => Ok(MaterialName::EnhancingComponents),
                "ethereal components" => Ok(MaterialName::EtherealComponents),
                "evasive components" => Ok(MaterialName::EvasiveComponents),
                "explosive components" => Ok(MaterialName::ExplosiveComponents),
                "faceted components" => Ok(MaterialName::FacetedComponents),
                "flexible parts" => Ok(MaterialName::FlexibleParts),
                "fortunate components" => Ok(MaterialName::FortunateComponents),
                "fungal components" => Ok(MaterialName::FungalComponents),
                "harnessed components" => Ok(MaterialName::HarnessedComponents),
                "head parts" => Ok(MaterialName::HeadParts),
                "healthy components" => Ok(MaterialName::HealthyComponents),
                "heavy components" => Ok(MaterialName::HeavyComponents),
                "historic components" => Ok(MaterialName::HistoricComponents),
                "ilujankan components" => Ok(MaterialName::IlujankanComponents),
                "imbued components" => Ok(MaterialName::ImbuedComponents),
                "junk" => Ok(MaterialName::Junk),
                "knightly components" => Ok(MaterialName::KnightlyComponents),
                "light components" => Ok(MaterialName::LightComponents),
                "living components" => Ok(MaterialName::LivingComponents),
                "magic parts" => Ok(MaterialName::MagicParts),
                "metallic parts" => Ok(MaterialName::MetallicParts),
                "noxious components" => Ok(MaterialName::NoxiousComponents),
                "oceanic components" => Ok(MaterialName::OceanicComponents),
                "organic parts" => Ok(MaterialName::OrganicParts),
                "padded parts" => Ok(MaterialName::PaddedParts),
                "pestiferous components" => Ok(MaterialName::PestiferousComponents),
                "pious components" => Ok(MaterialName::PiousComponents),
                "plated parts" => Ok(MaterialName::PlatedParts),
                "powerful components" => Ok(MaterialName::PowerfulComponents),
                "precious components" => Ok(MaterialName::PreciousComponents),
                "precise components" => Ok(MaterialName::PreciseComponents),
                "protective components" => Ok(MaterialName::ProtectiveComponents),
                "refined components" => Ok(MaterialName::RefinedComponents),
                "resilient components" => Ok(MaterialName::ResilientComponents),
                "rumbling components" => Ok(MaterialName::RumblingComponents),
                "saradomin components" => Ok(MaterialName::SaradominComponents),
                "seren components" => Ok(MaterialName::SerenComponents),
                "shadow components" => Ok(MaterialName::ShadowComponents),
                "sharp components" => Ok(MaterialName::SharpComponents),
                "shifting components" => Ok(MaterialName::ShiftingComponents),
                "silent components" => Ok(MaterialName::SilentComponents),
                "simple parts" => Ok(MaterialName::SimpleParts),
                "smooth parts" => Ok(MaterialName::SmoothParts),
                "spiked parts" => Ok(MaterialName::SpikedParts),
                "spiritual parts" => Ok(MaterialName::SpiritualParts),
                "stave parts" => Ok(MaterialName::StaveParts),
                "strong components" => Ok(MaterialName::StrongComponents),
                "stunning components" => Ok(MaterialName::StunningComponents),
                "subtle components" => Ok(MaterialName::SubtleComponents),
                "swift components" => Ok(MaterialName::SwiftComponents),
                "tensile parts" => Ok(MaterialName::TensileParts),
                "third-age components" => Ok(MaterialName::ThirdAgeComponents),
                "timeworn components" => Ok(MaterialName::TimewornComponents),
                "undead components" => Ok(MaterialName::UndeadComponents),
                "variable components" => Ok(MaterialName::VariableComponents),
                "vintage components" => Ok(MaterialName::VintageComponents),
                "zamorak components" => Ok(MaterialName::ZamorakComponents),
                "zaros components" => Ok(MaterialName::ZarosComponents),
                _ => Err("Unknown material name"),
            }
        }

        let mut x = find(mat);
        if x.is_err() {
            x = find(format!("{} parts", mat).as_str());
        }
        if x.is_err() {
            x = find(format!("{} components", mat).as_str());
        }
        x
    }
}

impl From<MaterialName> for usize {
    fn from(value: MaterialName) -> Self {
        value as usize
    }
}
