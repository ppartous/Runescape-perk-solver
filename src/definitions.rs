use clap::{Parser, ValueEnum};
use derive_more::Display;
use std::{fmt, str::FromStr, collections::HashMap, ops::Index};
use serde::Deserialize;
use serde_with::DeserializeFromStr;
use smallvec::{SmallVec, smallvec};

// ---------------------------------------------------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Display, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum GizmoType {
    Weapon,
    Armour,
    Tool
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Display, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum SortType {
    Gizmo,
    Attempt,
    Price
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub struct Perk {
    pub perk: PerkName,
    pub rank: u8
}

impl Default for Perk {
    fn default() -> Self {
        Perk { perk: PerkName::Empty, rank: 0 }
    }
}

impl PartialEq<PerkRankValues> for Perk {
    fn eq(&self, other: &PerkRankValues) -> bool {
        self.perk == other.perk && self.rank == other.rank
    }
}

impl PartialEq<Perk> for PerkRankValues {
    fn eq(&self, other: &Perk) -> bool {
        other == self
    }
}

impl From<&PerkRankValues> for Perk {
    fn from(x: &PerkRankValues) -> Self {
        Perk {
            perk: x.perk,
            rank: x.rank
        }
    }
}

impl Perk {
    pub fn is_empty(&self) -> bool {
        self.perk == PerkName::Empty
    }
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Gizmo {
    pub perks: (Perk, Perk),
    pub cost: i16,
    pub probability: f64
}

impl Default for Gizmo {
    fn default() -> Self {
        Gizmo {
            perks: (Perk::default(), Perk::default()),
            cost: 0,
            probability: 0.0
        }
    }
}

impl Gizmo {
    pub fn eq(&self, other: &Self) -> bool {
        (self.perks.0 == other.perks.0 && self.perks.1 == other.perks.1)
        || (self.perks.1 == other.perks.0 && self.perks.0 == other.perks.1)
    }

    pub fn fuzzy_eq(&self, other: &Self) -> bool {
        self.perks.0 == other.perks.0 || self.perks.1 == other.perks.0
    }

    pub fn create(x: &PerkRankValues, y: Option<&PerkRankValues>) -> Gizmo {
        Gizmo {
            perks: (
                Perk {
                    perk: x.perk,
                    rank: x.rank
                },
                if let Some(y) = y {
                    Perk {
                        perk: y.perk,
                        rank: y.rank
                    }
                } else {
                    Perk { ..Default::default() }
                }
            ),
            cost: (x.cost + if let Some(y) = y { y.cost } else { 0 }) as i16,
            probability: 0.0
        }
    }

    pub fn create_from_doubleslot(x: &PerkRankValues, y: Option<&PerkRankValues>) -> Gizmo {
        Gizmo {
            perks: (
                Perk {
                    perk: x.perk,
                    rank: x.rank
                },
                Perk { ..Default::default() }
            ),
            cost: (x.cost + if let Some(y) = y { y.cost } else { 0 }) as i16,
            probability: 0.0
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PerkRankValuesProbabilityContainer {
    pub values: PerkRankValues,
    pub probability: f64,
}

pub type PRVPC = PerkRankValuesProbabilityContainer;

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub struct RankCombination {
    pub ranks: SmallVec<[PerkRankValues; 8]>,
    pub probability: f64
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct PerkValues {
    pub perk: PerkName,
    pub base: u16,
    pub rolls: SmallVec<[u8; 9]>,
    pub doubleslot: bool,
    pub ranks: SmallVec<[PerkRankValuesProbabilityContainer; 7]>,
    pub i_first: usize,
    pub i_last: usize,
}

impl Default for PerkValues {
    fn default() -> PerkValues {
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

impl PerkValues {
    pub fn iter_ranks<'a>(self: &'a Self) -> impl Iterator<Item = &'a PerkRankValuesProbabilityContainer> {
        let i_first = self.i_first as usize;
        let i_last = self.i_last as usize;
        self.ranks.iter().skip(i_first).take(i_last - i_first + 1)
    }

    pub fn iter_ranks_no_zero<'a>(self: &'a Self) -> impl Iterator<Item = &'a PerkRankValuesProbabilityContainer> {
        let i_first = 1.max(self.i_first as usize);
        let i_last = self.i_last as usize;
        self.ranks.iter().skip(i_first).take(i_last - i_first + 1)
    }
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct PerkRanksData {
    pub doubleslot: bool,
    pub ranks: Vec<PerkRankValues>,
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct ComponentValues {
    pub base: u16,
    pub perk: PerkName,
    pub roll: u16,
}

// ---------------------------------------------------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct Data {
    pub comps: HashMap<MaterialName, CompPerksPerGizmoType>,
    pub perks: HashMap<PerkName, PerkRanksData>
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct SplitMaterials {
    pub conflict: Vec<MaterialName>,
    pub no_conflict: Vec<MaterialName>
}

// ---------------------------------------------------------------------------------------------------------------------

pub struct Range<T> {
    pub min: T,
    pub max: T
}

// ---------------------------------------------------------------------------------------------------------------------

pub struct Budget {
    pub dist: Vec<f64>,
    pub level: u16,
    pub range: Range<u16>
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, DeserializeFromStr, Hash)]
pub enum PerkName {
    Empty,
    Absorbative,
    Aftershock,
    Antitheism,
    Biting,
    Blunted,
    Brassican,
    Breakdown,
    BriefRespite,
    Bulwark,
    Butterfingers,
    Caroming,
    Cautious,
    Charitable,
    Cheapskate,
    ClearHeaded,
    Committed,
    Confused,
    Crackling,
    CrystalShield,
    DemonBait,
    DemonSlayer,
    Devoted,
    DragonBait,
    DragonSlayer,
    Efficient,
    Energising,
    EnhancedDevoted,
    EnhancedEfficient,
    Enlightened,
    Equilibrium,
    Fatiguing,
    Flanking,
    Fortune,
    Furnace,
    Genocidal,
    GlowWorm,
    Hallucinogenic,
    Hoarding,
    Honed,
    Impatient,
    ImpSouled,
    Inaccurate,
    Invigorating,
    JunkFood,
    Looting,
    Lucky,
    Lunging,
    Mediocrity,
    Mobile,
    Mysterious,
    NoEffect,
    PlantedFeet,
    Polishing,
    Precise,
    Preparation,
    Profane,
    Prosper,
    Pyromaniac,
    Rapid,
    Refined,
    Reflexes,
    Relentless,
    Ruthless,
    Scavenging,
    ShieldBashing,
    Spendthrift,
    Talking,
    Taunting,
    Tinker,
    TrophyTaker,
    Turtling,
    Ultimatums,
    UndeadBait,
    UndeadSlayer,
    Venomblood,
    Wise,
}

impl fmt::Display for PerkName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PerkName::Empty => write!(f, "Empty"),
            PerkName::Absorbative => write!(f, "Absorbative"),
            PerkName::Aftershock => write!(f, "Aftershock"),
            PerkName::Antitheism => write!(f, "Antitheism"),
            PerkName::Biting => write!(f, "Biting"),
            PerkName::Blunted => write!(f, "Blunted"),
            PerkName::Brassican => write!(f, "Brassican"),
            PerkName::Breakdown => write!(f, "Breakdown"),
            PerkName::BriefRespite => write!(f, "Brief Respite"),
            PerkName::Bulwark => write!(f, "Bulwark"),
            PerkName::Butterfingers => write!(f, "Butterfingers"),
            PerkName::Caroming => write!(f, "Caroming"),
            PerkName::Cautious => write!(f, "Cautious"),
            PerkName::Charitable => write!(f, "Charitable"),
            PerkName::Cheapskate => write!(f, "Cheapskate"),
            PerkName::ClearHeaded => write!(f, "Clear Headed"),
            PerkName::Committed => write!(f, "Committed"),
            PerkName::Confused => write!(f, "Confused"),
            PerkName::Crackling => write!(f, "Crackling"),
            PerkName::CrystalShield => write!(f, "Crystal Shield"),
            PerkName::DemonBait => write!(f, "Demon Bait"),
            PerkName::DemonSlayer => write!(f, "Demon Slayer"),
            PerkName::Devoted => write!(f, "Devoted"),
            PerkName::DragonBait => write!(f, "Dragon Bait"),
            PerkName::DragonSlayer => write!(f, "Dragon Slayer"),
            PerkName::Efficient => write!(f, "Efficient"),
            PerkName::Energising => write!(f, "Energising"),
            PerkName::EnhancedDevoted => write!(f, "Enhanced Devoted"),
            PerkName::EnhancedEfficient => write!(f, "Enhanced Efficient"),
            PerkName::Enlightened => write!(f, "Enlightened"),
            PerkName::Equilibrium => write!(f, "Equilibrium"),
            PerkName::Fatiguing => write!(f, "Fatiguing"),
            PerkName::Flanking => write!(f, "Flanking"),
            PerkName::Fortune => write!(f, "Fortune"),
            PerkName::Furnace => write!(f, "Furnace"),
            PerkName::Genocidal => write!(f, "Genocidal"),
            PerkName::GlowWorm => write!(f, "Glow Worm"),
            PerkName::Hallucinogenic => write!(f, "Hallucinogenic"),
            PerkName::Hoarding => write!(f, "Hoarding"),
            PerkName::Honed => write!(f, "Honed"),
            PerkName::Impatient => write!(f, "Impatient"),
            PerkName::ImpSouled => write!(f, "Imp Souled"),
            PerkName::Inaccurate => write!(f, "Inaccurate"),
            PerkName::Invigorating => write!(f, "Invigorating"),
            PerkName::JunkFood => write!(f, "Junk Food"),
            PerkName::Looting => write!(f, "Looting"),
            PerkName::Lucky => write!(f, "Lucky"),
            PerkName::Lunging => write!(f, "Lunging"),
            PerkName::Mediocrity => write!(f, "Mediocrity"),
            PerkName::Mobile => write!(f, "Mobile"),
            PerkName::Mysterious => write!(f, "Mysterious"),
            PerkName::NoEffect => write!(f, "No effect"),
            PerkName::PlantedFeet => write!(f, "Planted Feet"),
            PerkName::Polishing => write!(f, "Polishing"),
            PerkName::Precise => write!(f, "Precise"),
            PerkName::Preparation => write!(f, "Preparation"),
            PerkName::Profane => write!(f, "Profane"),
            PerkName::Prosper => write!(f, "Prosper"),
            PerkName::Pyromaniac => write!(f, "Pyromaniac"),
            PerkName::Rapid => write!(f, "Rapid"),
            PerkName::Refined => write!(f, "Refined"),
            PerkName::Reflexes => write!(f, "Reflexes"),
            PerkName::Relentless => write!(f, "Relentless"),
            PerkName::Ruthless => write!(f, "Ruthless"),
            PerkName::Scavenging => write!(f, "Scavenging"),
            PerkName::ShieldBashing => write!(f, "Shield Bashing"),
            PerkName::Spendthrift => write!(f, "Spendthrift"),
            PerkName::Talking => write!(f, "Talking"),
            PerkName::Taunting => write!(f, "Taunting"),
            PerkName::Tinker => write!(f, "Tinker"),
            PerkName::TrophyTaker => write!(f, "Trophy-taker's"),
            PerkName::Turtling => write!(f, "Turtling"),
            PerkName::Ultimatums => write!(f, "Ultimatums"),
            PerkName::UndeadBait => write!(f, "Undead Bait"),
            PerkName::UndeadSlayer => write!(f, "Undead Slayer"),
            PerkName::Venomblood => write!(f, "Venomblood"),
            PerkName::Wise => write!(f, "Wise"),
        }
    }
}

impl FromStr for PerkName {
    type Err = &'static str;

    fn from_str(perk: &str) -> Result<Self, Self::Err> {
        match perk.to_lowercase().as_str() {
            "absorbative" => Ok(PerkName::Absorbative),
            "aftershock" => Ok(PerkName::Aftershock),
            "antitheism" => Ok(PerkName::Antitheism),
            "biting" => Ok(PerkName::Biting),
            "blunted" => Ok(PerkName::Blunted),
            "brassican" => Ok(PerkName::Brassican),
            "breakdown" => Ok(PerkName::Breakdown),
            "brief respite" => Ok(PerkName::BriefRespite),
            "bulwark" => Ok(PerkName::Bulwark),
            "butterfingers" => Ok(PerkName::Butterfingers),
            "caroming" => Ok(PerkName::Caroming),
            "cautious" => Ok(PerkName::Cautious),
            "charitable" => Ok(PerkName::Charitable),
            "cheapskate" => Ok(PerkName::Cheapskate),
            "clear headed" => Ok(PerkName::ClearHeaded),
            "committed" => Ok(PerkName::Committed),
            "confused" => Ok(PerkName::Confused),
            "crackling" => Ok(PerkName::Crackling),
            "crystal shield" => Ok(PerkName::CrystalShield),
            "demon bait" => Ok(PerkName::DemonBait),
            "demon slayer" => Ok(PerkName::DemonSlayer),
            "devoted" => Ok(PerkName::Devoted),
            "dragon bait" => Ok(PerkName::DragonBait),
            "dragon slayer" => Ok(PerkName::DragonSlayer),
            "efficient" => Ok(PerkName::Efficient),
            "energising" => Ok(PerkName::Energising),
            "enhanced devoted" => Ok(PerkName::EnhancedDevoted),
            "enhanced efficient" => Ok(PerkName::EnhancedEfficient),
            "enlightened" => Ok(PerkName::Enlightened),
            "equilibrium" => Ok(PerkName::Equilibrium),
            "fatiguing" => Ok(PerkName::Fatiguing),
            "flanking" => Ok(PerkName::Flanking),
            "fortune" => Ok(PerkName::Fortune),
            "furnace" => Ok(PerkName::Furnace),
            "genocidal" => Ok(PerkName::Genocidal),
            "glow worm" => Ok(PerkName::GlowWorm),
            "hallucinogenic" => Ok(PerkName::Hallucinogenic),
            "hoarding" => Ok(PerkName::Hoarding),
            "honed" => Ok(PerkName::Honed),
            "imp souled" => Ok(PerkName::ImpSouled),
            "impatient" => Ok(PerkName::Impatient),
            "inaccurate" => Ok(PerkName::Inaccurate),
            "invigorating" => Ok(PerkName::Invigorating),
            "junk food" => Ok(PerkName::JunkFood),
            "looting" => Ok(PerkName::Looting),
            "lucky" => Ok(PerkName::Lucky),
            "lunging" => Ok(PerkName::Lunging),
            "mediocrity" => Ok(PerkName::Mediocrity),
            "mobile" => Ok(PerkName::Mobile),
            "mysterious" => Ok(PerkName::Mysterious),
            "no effect" => Ok(PerkName::NoEffect),
            "planted feet" => Ok(PerkName::PlantedFeet),
            "polishing" => Ok(PerkName::Polishing),
            "precise" => Ok(PerkName::Precise),
            "preparation" => Ok(PerkName::Preparation),
            "profane" => Ok(PerkName::Profane),
            "prosper" => Ok(PerkName::Prosper),
            "pyromaniac" => Ok(PerkName::Pyromaniac),
            "rapid" => Ok(PerkName::Rapid),
            "refined" => Ok(PerkName::Refined),
            "reflexes" => Ok(PerkName::Reflexes),
            "relentless" => Ok(PerkName::Relentless),
            "ruthless" => Ok(PerkName::Ruthless),
            "scavenging" => Ok(PerkName::Scavenging),
            "shield bashing" => Ok(PerkName::ShieldBashing),
            "spendthrift" => Ok(PerkName::Spendthrift),
            "talking" => Ok(PerkName::Talking),
            "taunting" => Ok(PerkName::Taunting),
            "tinker" => Ok(PerkName::Tinker),
            "trophy taker" => Ok(PerkName::TrophyTaker),
            "trophy-taker's" => Ok(PerkName::TrophyTaker),
            "trophy-taker" => Ok(PerkName::TrophyTaker),
            "turtling" => Ok(PerkName::Turtling),
            "ultimatums" => Ok(PerkName::Ultimatums),
            "undead bait" => Ok(PerkName::UndeadBait),
            "undead slayer" => Ok(PerkName::UndeadSlayer),
            "venomblood" => Ok(PerkName::Venomblood),
            "wise" => Ok(PerkName::Wise),
            _ => Err("Unknown perk")
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    mod gizmo_tests {
        use super::*;

        #[test]
        fn one_perk_equal() {
            let x = Gizmo {
                perks: (
                    Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                    Perk { perk: PerkName::Empty, rank: 0, ..Default::default() },
                ),
                ..Default::default()
            };
            let y = Gizmo {
                perks: (
                    Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                    Perk { perk: PerkName::Empty, rank: 0, ..Default::default() },
                ),
                ..Default::default()
            };
            assert_eq!(x.eq(&y), true);
        }

        #[test]
        fn one_perk_not_equal_but_same_rank() {
            let x = Gizmo {
                perks: (
                    Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                    Perk { perk: PerkName::Empty, rank: 0, ..Default::default() },
                ),
                ..Default::default()
            };
            let y = Gizmo {
                perks: (
                    Perk { perk: PerkName::Biting, rank: 1, ..Default::default() },
                    Perk { perk: PerkName::Empty, rank: 0, ..Default::default() },
                ),
                ..Default::default()
            };
            assert_eq!(x.eq(&y), false);
        }

        #[test]
        fn one_perk_equal_but_not_same_rank() {
            let x = Gizmo {
                perks: (
                    Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                    Perk { perk: PerkName::Empty, rank: 0, ..Default::default() },
                ),
                ..Default::default()
            };
            let y = Gizmo {
                perks: (
                    Perk { perk: PerkName::Precise, rank: 2, ..Default::default() },
                    Perk { perk: PerkName::Empty, rank: 0, ..Default::default() },
                ),
                ..Default::default()
            };
            assert_eq!(x.eq(&y), false);
        }

        #[test]
        fn two_perks_equal_same_order() {
            let x = Gizmo {
                perks: (
                    Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                    Perk { perk: PerkName::Biting, rank: 2, ..Default::default() },
                ),
                ..Default::default()
            };
            let y = Gizmo {
                perks: (
                    Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                    Perk { perk: PerkName::Biting, rank: 2, ..Default::default() },
                ),
                ..Default::default()
            };
            assert_eq!(x.eq(&y), true);
        }

        #[test]
        fn two_perks_equal_not_same_order() {
            let x = Gizmo {
                perks: (
                    Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                    Perk { perk: PerkName::Biting, rank: 2, ..Default::default() },
                ),
                ..Default::default()
            };
            let y = Gizmo {
                perks: (
                    Perk { perk: PerkName::Biting, rank: 2, ..Default::default() },
                    Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                ),
                ..Default::default()
            };
            assert_eq!(x.eq(&y), true);
        }

        #[test]
        fn two_perks_equal_perks_not_same_ranks() {
            let x = Gizmo {
                perks: (
                    Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                    Perk { perk: PerkName::Biting, rank: 2, ..Default::default() },
                ),
                ..Default::default()
            };
            let y = Gizmo {
                perks: (
                    Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                    Perk { perk: PerkName::Biting, rank: 3, ..Default::default() },
                ),
                ..Default::default()
            };
            assert_eq!(x.eq(&y), false);
        }

        #[test]
        fn two_perks_not_equal_perks_same_ranks() {
            let x = Gizmo {
                perks: (
                    Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                    Perk { perk: PerkName::Biting, rank: 2, ..Default::default() },
                ),
                ..Default::default()
            };
            let y = Gizmo {
                perks: (
                    Perk { perk: PerkName::Equilibrium, rank: 1, ..Default::default() },
                    Perk { perk: PerkName::Biting, rank: 2, ..Default::default() },
                ),
                ..Default::default()
            };
            assert_eq!(x.eq(&y), false);
        }

        #[test]
        fn fuzzy_match_first_perk() {
            let x = Gizmo {
                perks: (
                    Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                    Perk { perk: PerkName::Biting, rank: 2, ..Default::default() },
                ),
                ..Default::default()
            };
            let y = Gizmo {
                perks: (
                    Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                    Perk { ..Default::default() },
                ),
                ..Default::default()
            };
            assert_eq!(x.fuzzy_eq(&y), true);
        }

        #[test]
        fn fuzzy_match_second_perk() {
            let x = Gizmo {
                perks: (
                    Perk { perk: PerkName::Biting, rank: 2, ..Default::default() },
                    Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                ),
                ..Default::default()
            };
            let y = Gizmo {
                perks: (
                    Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                    Perk { ..Default::default() },
                ),
                ..Default::default()
            };
            assert_eq!(x.fuzzy_eq(&y), true);
        }

        #[test]
        fn fuzzy_match_none() {
            let x = Gizmo {
                perks: (
                    Perk { perk: PerkName::Biting, rank: 2, ..Default::default() },
                    Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                ),
                ..Default::default()
            };
            let y = Gizmo {
                perks: (
                    Perk { perk: PerkName::Precise, rank: 2, ..Default::default() },
                    Perk { ..Default::default() },
                ),
                ..Default::default()
            };
            assert_eq!(x.fuzzy_eq(&y), false);
        }
    }
}