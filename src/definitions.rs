use clap::{Parser, ValueEnum};
use derive_more::Display;
use std::{fmt, str::FromStr, collections::HashMap, ops::Index};
use serde::Deserialize;
use serde_with::DeserializeFromStr;
use smallvec::{SmallVec, smallvec};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Perk to look for
    pub perk: String,

    /// Rank of the first perk
    #[arg(default_value_t = 1)]
    pub rank: u8,

    /// Second perk in the gizmo
    pub perk_two: Option<String>,

    /// Rank of the second perk
    #[arg(default_value_t = 1)]
    pub rank_two: u8,

    #[arg(value_enum, short('t'), long("type"))]
    pub gizmo_type: GizmoType,

    /// Invention level. Use two values separated by a comma to search in a range
    #[arg(short('l'), long("level"), required(true), use_value_delimiter = true, value_delimiter = ',')]
    pub invention_level: Vec<u32>,

    /// Is ancient gizmo
    #[arg(short, long)]
    pub ancient: bool,

    /// Use this if you don't care what the second perk is
    #[arg(short, long)]
    pub fuzzy: bool,

    /// Comma separated list of material values to exclude. Uses basic substring matching.
    #[arg(short, long, use_value_delimiter = true, value_delimiter = ',')]
    pub exclude: Vec<String>,

    /// Sort the result on probability per consumed gizmo, probability per attempt, or on estimated price.
    #[arg(value_enum, short, long, default_value_t = SortType::Gizmo)]
    pub sort_type: SortType
}

#[derive(Debug, Display, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum GizmoType {
    Weapon,
    Armour,
    Tool
}

#[derive(Debug, Display, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum SortType {
    Gizmo,
    Attempt,
    Price
}

#[derive(Debug)]
pub struct WantedPerk {
    pub perk: PerkName,
    pub rank: u8,
    pub doubleslot: bool
}

impl Default for WantedPerk {
    fn default() -> Self {
        WantedPerk { perk: PerkName::Empty, rank: 0, doubleslot: false }
    }
}

#[derive(Debug)]
pub struct WantedGizmo (pub WantedPerk, pub WantedPerk);

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Display)]
#[display(fmt = "{{\n\tperk = {},\n\trank = {},\n\tdoubleslot = {},\n\tancient_only = {},\n\tcost = {},\n\tthreshold = {}\n}}", perk, rank, doubleslot, ancient_only, cost, threshold)]
pub struct PerkRankValues {
    pub ancient_only: bool,
    pub cost: u16,
    pub doubleslot: bool,
    pub perk: PerkName,
    pub rank: u8,
    pub threshold: u16,
}

impl Default for PerkRankValues {
    fn default() -> Self {
        PerkRankValues {
            ancient_only: false,
            cost: 0,
            doubleslot: false,
            perk: PerkName::Empty,
            rank: 0,
            threshold: 0
        }
    }
}

#[derive(Debug, Clone)]
pub struct PerkRankValuesProbabilityContainer<'a> {
    pub values: &'a PerkRankValues,
    pub probability: f64,
}

#[derive(Debug)]
pub struct PerkValues<'a> {
    pub perk: PerkName,
    pub base: u16,
    pub rolls: SmallVec<[u8; 9]>,
    pub doubleslot: bool,
    pub ranks: SmallVec<[PerkRankValuesProbabilityContainer<'a>; 7]>,
    pub i_first: u8,
    pub i_last: u8,
}

impl<'a> Default for PerkValues<'a> {
    fn default() -> PerkValues<'a> {
        PerkValues {
            perk: PerkName::Empty,
            base: 0,
            rolls: smallvec![],
            doubleslot: false,
            ranks: smallvec![],
            i_first: 0,
            i_last: 0
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PerkRanksData {
    pub doubleslot: bool,
    pub ranks: Vec<PerkRankValues>,
}

#[derive(Debug, Deserialize)]
pub struct ComponentValues {
    pub base: u16,
    pub perk: PerkName,
    pub roll: u16,
}

#[derive(Debug, Deserialize)]
pub struct CompPerksPerGizmoType {
    pub ancient_only: bool,
    pub armour: Vec<ComponentValues>,
    pub tool: Vec<ComponentValues>,
    pub weapon: Vec<ComponentValues>,
}

impl Index<GizmoType> for CompPerksPerGizmoType {
    type Output = Vec<ComponentValues>;

    fn index(&self, index: GizmoType) -> &Self::Output {
        match index {
                GizmoType::Armour => &self.armour,
                GizmoType::Tool => &self.tool,
                GizmoType::Weapon => &self.weapon
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub comps: HashMap<MaterialName, CompPerksPerGizmoType>,
    pub perks: HashMap<PerkName, PerkRanksData>
}

#[derive(Debug)]
pub struct SplitMaterials {
    pub conflict: Vec<MaterialName>,
    pub no_conflict: Vec<MaterialName>
}

pub struct Range<T> {
    pub min: T,
    pub max: T
}

pub struct Budget {
    pub dist: Vec<f64>,
    pub level: u16,
    pub range: Range<u16>
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, DeserializeFromStr, Hash)]
pub enum PerkName {
    Biting,
    Tinker,
    JunkFood,
    Inaccurate,
    DemonSlayer,
    Looting,
    Antitheism,
    Confused,
    Relentless,
    GlowWorm,
    BriefRespite,
    Blunted,
    Wise,
    PlantedFeet,
    Genocidal,
    Invigorating,
    Breakdown,
    Prosper,
    Pyromaniac,
    Furnace,
    Bulwark,
    Reflexes,
    ClearHeaded,
    ShieldBashing,
    Preparation,
    EnhancedEfficient,
    Fatiguing,
    Hoarding,
    Turtling,
    Aftershock,
    Precise,
    Lunging,
    Hallucinogenic,
    Honed,
    Devoted,
    UndeadSlayer,
    Cautious,
    Venomblood,
    Profane,
    Crackling,
    Ultimatums,
    Energising,
    EnhancedDevoted,
    Impatient,
    DragonBait,
    DragonSlayer,
    Refined,
    CrystalShield,
    Cheapskate,
    Committed,
    DemonBait,
    Butterfingers,
    Mediocrity,
    Polishing,
    Talking,
    TrophyTaker,
    NoEffect,
    Mysterious,
    Equilibrium,
    Fortune,
    Lucky,
    ImpSouled,
    Taunting,
    Rapid,
    Absorbative,
    Spendthrift,
    UndeadBait,
    Efficient,
    Brassican,
    Caroming,
    Mobile,
    Flanking,
    Ruthless,
    Charitable,
    Enlightened,
    Scavenging,
    Empty
}

impl fmt::Display for PerkName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PerkName::Biting => write!(f, "Biting"),
            PerkName::Tinker => write!(f, "Tinker"),
            PerkName::JunkFood => write!(f, "Junk Food"),
            PerkName::Inaccurate => write!(f, "Inaccurate"),
            PerkName::DemonSlayer => write!(f, "Demon Slayer"),
            PerkName::Looting => write!(f, "Looting"),
            PerkName::Antitheism => write!(f, "Antitheism"),
            PerkName::Confused => write!(f, "Confused"),
            PerkName::Relentless => write!(f, "Relentless"),
            PerkName::GlowWorm => write!(f, "Glow Worm"),
            PerkName::BriefRespite => write!(f, "Brief Respite"),
            PerkName::Blunted => write!(f, "Blunted"),
            PerkName::Wise => write!(f, "Wise"),
            PerkName::PlantedFeet => write!(f, "Planted Feet"),
            PerkName::Genocidal => write!(f, "Genocidal"),
            PerkName::Invigorating => write!(f, "Invigorating"),
            PerkName::Breakdown => write!(f, "Breakdown"),
            PerkName::Prosper => write!(f, "Prosper"),
            PerkName::Pyromaniac => write!(f, "Pyromaniac"),
            PerkName::Furnace => write!(f, "Furnace"),
            PerkName::Bulwark => write!(f, "Bulwark"),
            PerkName::Reflexes => write!(f, "Reflexes"),
            PerkName::ClearHeaded => write!(f, "Clear Headed"),
            PerkName::ShieldBashing => write!(f, "Shield Bashing"),
            PerkName::Preparation => write!(f, "Preparation"),
            PerkName::EnhancedEfficient => write!(f, "Enhanced Efficient"),
            PerkName::Fatiguing => write!(f, "Fatiguing"),
            PerkName::Hoarding => write!(f, "Hoarding"),
            PerkName::Turtling => write!(f, "Turtling"),
            PerkName::Aftershock => write!(f, "Aftershock"),
            PerkName::Precise => write!(f, "Precise"),
            PerkName::Lunging => write!(f, "Lunging"),
            PerkName::Hallucinogenic => write!(f, "Hallucinogenic"),
            PerkName::Honed => write!(f, "Honed"),
            PerkName::Devoted => write!(f, "Devoted"),
            PerkName::UndeadSlayer => write!(f, "Undead Slayer"),
            PerkName::Cautious => write!(f, "Cautious"),
            PerkName::Venomblood => write!(f, "Venomblood"),
            PerkName::Profane => write!(f, "Profane"),
            PerkName::Crackling => write!(f, "Crackling"),
            PerkName::Ultimatums => write!(f, "Ultimatums"),
            PerkName::Energising => write!(f, "Energising"),
            PerkName::EnhancedDevoted => write!(f, "Enhanced Devoted"),
            PerkName::Impatient => write!(f, "Impatient"),
            PerkName::DragonBait => write!(f, "Dragon Bait"),
            PerkName::DragonSlayer => write!(f, "Dragon Slayer"),
            PerkName::Refined => write!(f, "Refined"),
            PerkName::CrystalShield => write!(f, "Crystal Shield"),
            PerkName::Cheapskate => write!(f, "Cheapskate"),
            PerkName::Committed => write!(f, "Committed"),
            PerkName::DemonBait => write!(f, "Demon Bait"),
            PerkName::Butterfingers => write!(f, "Butterfingers"),
            PerkName::Mediocrity => write!(f, "Mediocrity"),
            PerkName::Polishing => write!(f, "Polishing"),
            PerkName::Talking => write!(f, "Talking"),
            PerkName::TrophyTaker => write!(f, "Trophy-taker's"),
            PerkName::NoEffect => write!(f, "No effect"),
            PerkName::Mysterious => write!(f, "Mysterious"),
            PerkName::Equilibrium => write!(f, "Equilibrium"),
            PerkName::Fortune => write!(f, "Fortune"),
            PerkName::Lucky => write!(f, "Lucky"),
            PerkName::ImpSouled => write!(f, "Imp Souled"),
            PerkName::Taunting => write!(f, "Taunting"),
            PerkName::Rapid => write!(f, "Rapid"),
            PerkName::Absorbative => write!(f, "Absorbative"),
            PerkName::Spendthrift => write!(f, "Spendthrift"),
            PerkName::UndeadBait => write!(f, "Undead Bait"),
            PerkName::Efficient => write!(f, "Efficient"),
            PerkName::Brassican => write!(f, "Brassican"),
            PerkName::Caroming => write!(f, "Caroming"),
            PerkName::Mobile => write!(f, "Mobile"),
            PerkName::Flanking => write!(f, "Flanking"),
            PerkName::Ruthless => write!(f, "Ruthless"),
            PerkName::Charitable => write!(f, "Charitable"),
            PerkName::Enlightened => write!(f, "Enlightened"),
            PerkName::Scavenging => write!(f, "Scavenging"),
            PerkName::Empty => write!(f, "Empty")
        }
    }
}

impl FromStr for PerkName {
    type Err = &'static str;

    fn from_str(perk: &str) -> Result<Self, Self::Err> {
        match perk.to_lowercase().as_str() {
            "biting" => Ok(PerkName::Biting),
            "tinker" => Ok(PerkName::Tinker),
            "junk food" => Ok(PerkName::JunkFood),
            "inaccurate" => Ok(PerkName::Inaccurate),
            "demon slayer" => Ok(PerkName::DemonSlayer),
            "looting" => Ok(PerkName::Looting),
            "antitheism" => Ok(PerkName::Antitheism),
            "confused" => Ok(PerkName::Confused),
            "relentless" => Ok(PerkName::Relentless),
            "glow worm" => Ok(PerkName::GlowWorm),
            "brief respite" => Ok(PerkName::BriefRespite),
            "blunted" => Ok(PerkName::Blunted),
            "wise" => Ok(PerkName::Wise),
            "planted feet" => Ok(PerkName::PlantedFeet),
            "genocidal" => Ok(PerkName::Genocidal),
            "invigorating" => Ok(PerkName::Invigorating),
            "breakdown" => Ok(PerkName::Breakdown),
            "prosper" => Ok(PerkName::Prosper),
            "pyromaniac" => Ok(PerkName::Pyromaniac),
            "furnace" => Ok(PerkName::Furnace),
            "bulwark" => Ok(PerkName::Bulwark),
            "reflexes" => Ok(PerkName::Reflexes),
            "clear headed" => Ok(PerkName::ClearHeaded),
            "shield bashing" => Ok(PerkName::ShieldBashing),
            "preparation" => Ok(PerkName::Preparation),
            "enhanced efficient" => Ok(PerkName::EnhancedEfficient),
            "fatiguing" => Ok(PerkName::Fatiguing),
            "hoarding" => Ok(PerkName::Hoarding),
            "turtling" => Ok(PerkName::Turtling),
            "aftershock" => Ok(PerkName::Aftershock),
            "precise" => Ok(PerkName::Precise),
            "lunging" => Ok(PerkName::Lunging),
            "hallucinogenic" => Ok(PerkName::Hallucinogenic),
            "honed" => Ok(PerkName::Honed),
            "devoted" => Ok(PerkName::Devoted),
            "undead slayer" => Ok(PerkName::UndeadSlayer),
            "cautious" => Ok(PerkName::Cautious),
            "venomblood" => Ok(PerkName::Venomblood),
            "profane" => Ok(PerkName::Profane),
            "crackling" => Ok(PerkName::Crackling),
            "ultimatums" => Ok(PerkName::Ultimatums),
            "energising" => Ok(PerkName::Energising),
            "enhanced devoted" => Ok(PerkName::EnhancedDevoted),
            "impatient" => Ok(PerkName::Impatient),
            "dragon bait" => Ok(PerkName::DragonBait),
            "dragon slayer" => Ok(PerkName::DragonSlayer),
            "refined" => Ok(PerkName::Refined),
            "crystal shield" => Ok(PerkName::CrystalShield),
            "cheapskate" => Ok(PerkName::Cheapskate),
            "committed" => Ok(PerkName::Committed),
            "demon bait" => Ok(PerkName::DemonBait),
            "butterfingers" => Ok(PerkName::Butterfingers),
            "mediocrity" => Ok(PerkName::Mediocrity),
            "polishing" => Ok(PerkName::Polishing),
            "talking" => Ok(PerkName::Talking),
            "trophy-taker's" => Ok(PerkName::TrophyTaker),
            "trophy-taker" => Ok(PerkName::TrophyTaker),
            "trophy taker" => Ok(PerkName::TrophyTaker),
            "no effect" => Ok(PerkName::NoEffect),
            "mysterious" => Ok(PerkName::Mysterious),
            "equilibrium" => Ok(PerkName::Equilibrium),
            "fortune" => Ok(PerkName::Fortune),
            "lucky" => Ok(PerkName::Lucky),
            "imp souled" => Ok(PerkName::ImpSouled),
            "taunting" => Ok(PerkName::Taunting),
            "rapid" => Ok(PerkName::Rapid),
            "absorbative" => Ok(PerkName::Absorbative),
            "spendthrift" => Ok(PerkName::Spendthrift),
            "undead bait" => Ok(PerkName::UndeadBait),
            "efficient" => Ok(PerkName::Efficient),
            "brassican" => Ok(PerkName::Brassican),
            "caroming" => Ok(PerkName::Caroming),
            "mobile" => Ok(PerkName::Mobile),
            "flanking" => Ok(PerkName::Flanking),
            "ruthless" => Ok(PerkName::Ruthless),
            "charitable" => Ok(PerkName::Charitable),
            "enlightened" => Ok(PerkName::Enlightened),
            "scavenging" => Ok(PerkName::Scavenging),
            _ => Err("Unknown perk")
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, DeserializeFromStr, Hash)]
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
    ThirdAgeComponents,
    TimewornComponents,
    UndeadComponents,
    VariableComponents,
    VintageComponents,
    ZamorakComponents,
    ZarosComponents,
}

impl fmt::Display for MaterialName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaterialName::ArmadylComponents => write!(f, "Armadyl components"),
            MaterialName::AscendedComponents => write!(f, "Ascended components"),
            MaterialName::AvernicComponents => write!(f, "Avernic components"),
            MaterialName::BandosComponents => write!(f, "Bandos components"),
            MaterialName::BaseParts => write!(f, "Base parts"),
            MaterialName::BladeParts => write!(f, "Blade parts"),
            MaterialName::BrassicanComponents => write!(f, "Brassican components"),
            MaterialName::ClassicComponents => write!(f, "Classic components"),
            MaterialName::ClearParts => write!(f, "Clear parts"),
            MaterialName::ClockworkComponents => write!(f, "Clockwork components"),
            MaterialName::ConnectorParts => write!(f, "Connector parts"),
            MaterialName::CorporealComponents => write!(f, "Corporeal components"),
            MaterialName::CoverParts => write!(f, "Cover parts"),
            MaterialName::CraftedParts => write!(f, "Crafted parts"),
            MaterialName::CrystalParts => write!(f, "Crystal parts"),
            MaterialName::CulinaryComponents => write!(f, "Culinary components"),
            MaterialName::CywirComponents => write!(f, "Cywir components"),
            MaterialName::DeflectingParts => write!(f, "Deflecting parts"),
            MaterialName::DelicateParts => write!(f, "Delicate parts"),
            MaterialName::DextrousComponents => write!(f, "Dextrous components"),
            MaterialName::DirectComponents => write!(f, "Direct components"),
            MaterialName::DragonfireComponents => write!(f, "Dragonfire components"),
            MaterialName::EnhancingComponents => write!(f, "Enhancing components"),
            MaterialName::EtherealComponents => write!(f, "Ethereal components"),
            MaterialName::EvasiveComponents => write!(f, "Evasive components"),
            MaterialName::ExplosiveComponents => write!(f, "Explosive components"),
            MaterialName::FacetedComponents => write!(f, "Faceted components"),
            MaterialName::FlexibleParts => write!(f, "Flexible parts"),
            MaterialName::FortunateComponents => write!(f, "Fortunate components"),
            MaterialName::FungalComponents => write!(f, "Fungal components"),
            MaterialName::HarnessedComponents => write!(f, "Harnessed components"),
            MaterialName::HeadParts => write!(f, "Head parts"),
            MaterialName::HealthyComponents => write!(f, "Healthy components"),
            MaterialName::HeavyComponents => write!(f, "Heavy components"),
            MaterialName::HistoricComponents => write!(f, "Historic components"),
            MaterialName::IlujankanComponents => write!(f, "Ilujankan components"),
            MaterialName::ImbuedComponents => write!(f, "Imbued components"),
            MaterialName::Junk => write!(f, "Junk"),
            MaterialName::KnightlyComponents => write!(f, "Knightly components"),
            MaterialName::LightComponents => write!(f, "Light components"),
            MaterialName::LivingComponents => write!(f, "Living components"),
            MaterialName::MagicParts => write!(f, "Magic parts"),
            MaterialName::MetallicParts => write!(f, "Metallic parts"),
            MaterialName::NoxiousComponents => write!(f, "Noxious components"),
            MaterialName::OceanicComponents => write!(f, "Oceanic components"),
            MaterialName::OrganicParts => write!(f, "Organic parts"),
            MaterialName::PaddedParts => write!(f, "Padded parts"),
            MaterialName::PestiferousComponents => write!(f, "Pestiferous components"),
            MaterialName::PiousComponents => write!(f, "Pious components"),
            MaterialName::PlatedParts => write!(f, "Plated parts"),
            MaterialName::PowerfulComponents => write!(f, "Powerful components"),
            MaterialName::PreciousComponents => write!(f, "Precious components"),
            MaterialName::PreciseComponents => write!(f, "Precise components"),
            MaterialName::ProtectiveComponents => write!(f, "Protective components"),
            MaterialName::RefinedComponents => write!(f, "Refined components"),
            MaterialName::ResilientComponents => write!(f, "Resilient components"),
            MaterialName::RumblingComponents => write!(f, "Rumbling components"),
            MaterialName::SaradominComponents => write!(f, "Saradomin components"),
            MaterialName::SerenComponents => write!(f, "Seren components"),
            MaterialName::ShadowComponents => write!(f, "Shadow components"),
            MaterialName::SharpComponents => write!(f, "Sharp components"),
            MaterialName::ShiftingComponents => write!(f, "Shifting components"),
            MaterialName::SilentComponents => write!(f, "Silent components"),
            MaterialName::SimpleParts => write!(f, "Simple parts"),
            MaterialName::SmoothParts => write!(f, "Smooth parts"),
            MaterialName::SpikedParts => write!(f, "Spiked parts"),
            MaterialName::SpiritualParts => write!(f, "Spiritual parts"),
            MaterialName::StaveParts => write!(f, "Stave parts"),
            MaterialName::StrongComponents => write!(f, "Strong components"),
            MaterialName::StunningComponents => write!(f, "Stunning components"),
            MaterialName::SubtleComponents => write!(f, "Subtle components"),
            MaterialName::SwiftComponents => write!(f, "Swift components"),
            MaterialName::TensileParts => write!(f, "Tensile parts"),
            MaterialName::ThirdAgeComponents => write!(f, "Third-age components"),
            MaterialName::TimewornComponents => write!(f, "Timeworn components"),
            MaterialName::UndeadComponents => write!(f, "Undead components"),
            MaterialName::VariableComponents => write!(f, "Variable components"),
            MaterialName::VintageComponents => write!(f, "Vintage components"),
            MaterialName::ZamorakComponents => write!(f, "Zamorak components"),
            MaterialName::ZarosComponents => write!(f, "Zaros components"),
        }
    }
}

impl FromStr for MaterialName {
    type Err = &'static str;

    fn from_str(mat: &str) -> Result<Self, Self::Err> {
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
            _ => Err("Unknown material name")
        }
    }
}
