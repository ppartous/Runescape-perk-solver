use std::{fmt, str::FromStr, sync::atomic::AtomicBool};
use strum::{EnumIter, Display};
use strum_macros::{EnumCount, EnumVariantNames, EnumString, IntoStaticStr};

#[derive(Debug, Display, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, EnumCount, EnumIter, EnumVariantNames, EnumString, IntoStaticStr)]
#[strum(serialize_all = "title_case")]
#[strum(ascii_case_insensitive)]
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
    #[strum(serialize = "Trophy-taker's", serialize = "Trophy-taker", serialize = "Trophy taker")]
    TrophyTaker,
    Turtling,
    Ultimatums,
    UndeadBait,
    UndeadSlayer,
    Venomblood,
    Wise,
}

static mut USE_SIMPLE_PRINT_STYLE: AtomicBool = AtomicBool::new(false);

impl PerkName {
    pub const A: PerkName = PerkName::Absorbative;
    pub const B: PerkName = PerkName::Aftershock;
    pub const C: PerkName = PerkName::Antitheism;
    pub const D: PerkName = PerkName::Biting;
    pub const E: PerkName = PerkName::Blunted;
    pub const F: PerkName = PerkName::Brassican;
    pub const G: PerkName = PerkName::Breakdown;
    pub const H: PerkName = PerkName::BriefRespite;
    pub const I: PerkName = PerkName::Bulwark;
    pub const J: PerkName = PerkName::Butterfingers;
    pub const K: PerkName = PerkName::Caroming;

    pub fn using_simplified_names() {
        unsafe {
            *USE_SIMPLE_PRINT_STYLE.get_mut() = true;
        }
    }

    pub fn using_full_names() {
        unsafe {
            *USE_SIMPLE_PRINT_STYLE.get_mut() = false;
        }
    }
}

impl From<PerkName> for usize {
    fn from(value: PerkName) -> Self {
        value as usize
    }
}

impl std::default::Default for PerkName {
    fn default() -> Self {
        PerkName::Empty
    }
}