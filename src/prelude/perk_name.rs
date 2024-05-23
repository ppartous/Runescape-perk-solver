use strum::{Display, EnumIter};
use strum_macros::{EnumCount, EnumString, EnumVariantNames, IntoStaticStr};

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
    EnumString,
    IntoStaticStr,
)]
#[strum(serialize_all = "title_case", ascii_case_insensitive, use_phf)]
pub enum PerkName {
    #[default]
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
    Eruptive,
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
    #[strum(
        serialize = "Trophy-taker's",
        serialize = "Trophy-taker",
        serialize = "Trophy taker"
    )]
    TrophyTaker,
    Turtling,
    Ultimatums,
    UndeadBait,
    UndeadSlayer,
    Venomblood,
    Wise,
}

impl PerkName {
    pub const A: PerkName = PerkName::Absorbative;
    pub const B: PerkName = PerkName::Biting;
    pub const C: PerkName = PerkName::Caroming;
    pub const D: PerkName = PerkName::DemonBait;
    pub const E: PerkName = PerkName::Efficient;
    pub const F: PerkName = PerkName::Fatiguing;
    pub const G: PerkName = PerkName::Genocidal;
    pub const H: PerkName = PerkName::Hallucinogenic;
    pub const I: PerkName = PerkName::Impatient;
    pub const J: PerkName = PerkName::JunkFood;
    pub const L: PerkName = PerkName::Looting;
}

impl From<PerkName> for usize {
    fn from(value: PerkName) -> Self {
        value as usize
    }
}
