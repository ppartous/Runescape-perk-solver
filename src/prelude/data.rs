pub mod prelude;
pub mod stack_map;

use crate::{MaterialName, PerkName};
pub use prelude::*;
pub use stack_map::*;
use strum::EnumCount;

#[derive(Debug)]
pub struct Data {
    pub comps: StackMap<MaterialName, CompPerksPerGizmoType, { MaterialName::COUNT }>,
    pub perks: StackMap<PerkName, PerkRanksData, { PerkName::COUNT }>,
}

impl Data {
    pub fn load() -> Data {
        Data {
            comps: stack_map! {
                MaterialName::ArmadylComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Precise,
                            base: 44,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Devoted,
                            base: 39,
                            roll: 9
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Charitable,
                            base: 25,
                            roll: 28
                        },
                    ])
                },
                MaterialName::AscendedComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Efficient,
                            base: 40,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::EnhancedEfficient,
                            base: 20,
                            roll: 25
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Efficient,
                            base: 40,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::EnhancedEfficient,
                            base: 20,
                            roll: 25
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Efficient,
                            base: 40,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::EnhancedEfficient,
                            base: 20,
                            roll: 25
                        },
                    ])
                },
                MaterialName::AvernicComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Lunging,
                            base: 44,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::BandosComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Genocidal,
                            base: 44,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Genocidal,
                            base: 44,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::Devoted,
                            base: 39,
                            roll: 9
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Careless,
                            base: 15,
                            roll: 10
                        },
                    ])
                },
                MaterialName::BaseParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Talking,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::DragonSlayer,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Inaccurate,
                            base: 8,
                            roll: 33
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Talking,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::DragonSlayer,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Turtling,
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Talking,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::Charitable,
                            base: 7,
                            roll: 25
                        },
                    ])
                },
                MaterialName::BladeParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Biting,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::JunkFood,
                            base: 8,
                            roll: 33
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Biting,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::JunkFood,
                            base: 8,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Honed,
                            base: 7,
                            roll: 25
                        },
                    ])
                },
                MaterialName::BrassicanComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Hallucinogenic,
                            base: 36,
                            roll: 9
                        },
                        ComponentValues {
                            perk: PerkName::Talking,
                            base: 45,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::Brassican,
                            base: 49,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Hallucinogenic,
                            base: 36,
                            roll: 9
                        },
                        ComponentValues {
                            perk: PerkName::Talking,
                            base: 45,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::Brassican,
                            base: 49,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Hallucinogenic,
                            base: 36,
                            roll: 9
                        },
                        ComponentValues {
                            perk: PerkName::Talking,
                            base: 45,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::Brassican,
                            base: 49,
                            roll: 8
                        },
                    ])
                },
                MaterialName::ClassicComponents => CompPerksPerGizmoType {
                    ancient_only: true,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Scavenging,
                            base: 12,
                            roll: 20
                        },
                        ComponentValues {
                            perk: PerkName::Efficient,
                            base: 12,
                            roll: 40
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Scavenging,
                            base: 12,
                            roll: 20
                        },
                        ComponentValues {
                            perk: PerkName::Efficient,
                            base: 12,
                            roll: 40
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Fortune,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Furnace,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Efficient,
                            base: 12,
                            roll: 40
                        },
                    ])
                },
                MaterialName::ClearParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::GlowWorm,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::Hallucinogenic,
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::DemonSlayer,
                            base: 5,
                            roll: 15
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::GlowWorm,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::Hallucinogenic,
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::DemonSlayer,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Profane,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::CrystalShield,
                            base: 5,
                            roll: 15
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::GlowWorm,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::Hallucinogenic,
                            base: 7,
                            roll: 28
                        },
                    ])
                },
                MaterialName::ClockworkComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::EnhancedEfficient,
                            base: 45,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::Flanking,
                            base: 25,
                            roll: 28
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::EnhancedEfficient,
                            base: 45,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::EnhancedEfficient,
                            base: 45,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::Tinker,
                            base: 18,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::Hasty,
                            base: 15,
                            roll: 35
                        },
                    ])
                },
                MaterialName::ConnectorParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Scavenging,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::DragonBait,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::UndeadBait,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Mobile,
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::Precise,
                            base: 5,
                            roll: 15
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Scavenging,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::DragonBait,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::UndeadBait,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Mobile,
                            base: 7,
                            roll: 28
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::CorporealComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::BriefRespite,
                            base: 40,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::CoverParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::ShieldBashing,
                            base: 5,
                            roll: 15
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::ShieldBashing,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Profane,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Bulwark,
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Confused,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Furnace,
                            base: 7,
                            roll: 25
                        },
                    ])
                },
                MaterialName::CraftedParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Hallucinogenic,
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::Fatiguing,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Mediocrity,
                            base: 8,
                            roll: 33
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Hallucinogenic,
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::Fatiguing,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Preparation,
                            base: 5,
                            roll: 15
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Hallucinogenic,
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::Fatiguing,
                            base: 8,
                            roll: 32
                        },
                    ])
                },
                MaterialName::CrystalParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Efficient,
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::TrophyTaker,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::UndeadSlayer,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::DemonBait,
                            base: 8,
                            roll: 32
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Efficient,
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::TrophyTaker,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::UndeadSlayer,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::DemonBait,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Turtling,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Profane,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::EnhancedDevoted,
                            base: 5,
                            roll: 15
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Efficient,
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::Cheapskate,
                            base: 8,
                            roll: 32
                        },
                    ])
                },
                MaterialName::CulinaryComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Invigorating,
                            base: 39,
                            roll: 9
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Invigorating,
                            base: 39,
                            roll: 9
                        },
                        ComponentValues {
                            perk: PerkName::BriefRespite,
                            base: 45,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::CywirComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::PlantedFeet,
                            base: 20,
                            roll: 20
                        },
                    ]),
                    armour: StackVec::new(&[
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::DeflectingParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::ShieldBashing,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Inaccurate,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Mediocrity,
                            base: 8,
                            roll: 32
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::ShieldBashing,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Venomblood,
                            base: 5,
                            roll: 15
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::ImpSouled,
                            base: 7,
                            roll: 27
                        },
                    ])
                },
                MaterialName::DelicateParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::DemonBait,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::JunkFood,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Eruptive,
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::EnhancedEfficient,
                            base: 5,
                            roll: 15
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::DemonBait,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::JunkFood,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Lucky,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::EnhancedEfficient,
                            base: 5,
                            roll: 15
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Butterfingers,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Charitable,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::EnhancedEfficient,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Tinker,
                            base: 7,
                            roll: 25
                        },
                    ])
                },
                MaterialName::DextrousComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::DemonSlayer,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::DragonBait,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::Mobile,
                            base: 12,
                            roll: 44
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::DemonSlayer,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::DragonBait,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::Mobile,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Reflexes,
                            base: 12,
                            roll: 40
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Polishing,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Butterfingers,
                            base: 9,
                            roll: 33
                        },
                    ])
                },
                MaterialName::DirectComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Biting,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Blunted,
                            base: 12,
                            roll: 45
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Biting,
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Charitable,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Pyromaniac,
                            base: 9,
                            roll: 32
                        },
                    ])
                },
                MaterialName::DragonfireComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::DragonSlayer,
                            base: 44,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::DragonSlayer,
                            base: 44,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Furnace,
                            base: 25,
                            roll: 28
                        },
                    ])
                },
                MaterialName::EnhancingComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::DragonSlayer,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Invigorating,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Energising,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Inaccurate,
                            base: 12,
                            roll: 45
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::DragonSlayer,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Invigorating,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Energising,
                            base: 12,
                            roll: 44
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Cheapskate,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::Refined,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Rapid,
                            base: 12,
                            roll: 40
                        },
                    ])
                },
                MaterialName::EtherealComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::Wise,
                            base: 12,
                            roll: 40
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::Wise,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::BriefRespite,
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::Wise,
                            base: 12,
                            roll: 40
                        },
                    ])
                },
                MaterialName::EvasiveComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Blunted,
                            base: 12,
                            roll: 45
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Turtling,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Venomblood,
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Prosper,
                            base: 12,
                            roll: 8
                        },
                    ])
                },
                MaterialName::ExplosiveComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Crackling,
                            base: 48,
                            roll: 5
                        },
                        ComponentValues {
                            perk: PerkName::Ultimatums,
                            base: 40,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Crackling,
                            base: 48,
                            roll: 5
                        },
                        ComponentValues {
                            perk: PerkName::Ultimatums,
                            base: 40,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Pyromaniac,
                            base: 25,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::Explosive,
                            base: 10,
                            roll: 25
                        },
                    ])
                },
                MaterialName::FacetedComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::CrystalShield,
                            base: 39,
                            roll: 9
                        },
                        ComponentValues {
                            perk: PerkName::EnhancedDevoted,
                            base: 45,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::FlexibleParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::Wise,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::DragonBait,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Mobile,
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::ClearHeaded,
                            base: 5,
                            roll: 15
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::Wise,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::DragonBait,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Mobile,
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::ClearHeaded,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Profane,
                            base: 8,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::Wise,
                            base: 9,
                            roll: 33
                        },
                    ])
                },
                MaterialName::FortunateComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Looting,
                            base: 13,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Hoarding,
                            base: 13,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Brassican,
                            base: 13,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Spendthrift,
                            base: 13,
                            roll: 40
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Looting,
                            base: 13,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Hoarding,
                            base: 13,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Brassican,
                            base: 13,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Lucky,
                            base: 13,
                            roll: 40
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Hoarding,
                            base: 13,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Brassican,
                            base: 13,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Polishing,
                            base: 13,
                            roll: 40
                        },
                    ])
                },
                MaterialName::FungalComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Absorbative,
                            base: 40,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Tinker,
                            base: 10,
                            roll: 28
                        },
                    ])
                },
                MaterialName::HarnessedComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Reflexes,
                            base: 45,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::Preparation,
                            base: 44,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::HeadParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Talking,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::ClearHeaded,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Inaccurate,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Mediocrity,
                            base: 8,
                            roll: 32
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Talking,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::ClearHeaded,
                            base: 5,
                            roll: 15
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Talking,
                            base: 7,
                            roll: 27
                        },
                    ])
                },
                MaterialName::HealthyComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Efficient,
                            base: 9,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Inaccurate,
                            base: 12,
                            roll: 45
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Efficient,
                            base: 9,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Venomblood,
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Efficient,
                            base: 9,
                            roll: 32
                        },
                    ])
                },
                MaterialName::HeavyComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::DemonBait,
                            base: 12,
                            roll: 45
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::DemonBait,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::Bulwark,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Preparation,
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Butterfingers,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::Breakdown,
                            base: 12,
                            roll: 40
                        },
                    ])
                },
                MaterialName::HistoricComponents => CompPerksPerGizmoType {
                    ancient_only: true,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Precise,
                            base: 11,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Genocidal,
                            base: 11,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Ultimatums,
                            base: 11,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Looting,
                            base: 11,
                            roll: 33
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Genocidal,
                            base: 11,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Ultimatums,
                            base: 11,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Looting,
                            base: 11,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Turtling,
                            base: 11,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::ImpSouled,
                            base: 11,
                            roll: 33
                        },
                    ])
                },
                MaterialName::IlujankanComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Aftershock,
                            base: 40,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::ImbuedComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Crackling,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Ultimatums,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::JunkFood,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::Energising,
                            base: 12,
                            roll: 44
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Crackling,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Ultimatums,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::JunkFood,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::Energising,
                            base: 12,
                            roll: 44
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Furnace,
                            base: 12,
                            roll: 40
                        },
                    ])
                },
                MaterialName::Junk => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                    ]),
                    armour: StackVec::new(&[
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::KnightlyComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Taunting,
                            base: 44,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Taunting,
                            base: 44,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::LightComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::GlowWorm,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Hallucinogenic,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Inaccurate,
                            base: 12,
                            roll: 45
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::GlowWorm,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Hallucinogenic,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Lucky,
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::GlowWorm,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Hallucinogenic,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Pyromaniac,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Rapid,
                            base: 9,
                            roll: 33
                        },
                    ])
                },
                MaterialName::LivingComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Talking,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::UndeadSlayer,
                            base: 9,
                            roll: 33
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Talking,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::UndeadSlayer,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Profane,
                            base: 12,
                            roll: 45
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Talking,
                            base: 12,
                            roll: 44
                        },
                    ])
                },
                MaterialName::MagicParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::UndeadBait,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Crackling,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Ultimatums,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Energising,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::Spendthrift,
                            base: 5,
                            roll: 13
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::UndeadBait,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Crackling,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Ultimatums,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Energising,
                            base: 7,
                            roll: 27
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Honed,
                            base: 7,
                            roll: 27
                        },
                    ])
                },
                MaterialName::ManufacturedComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                    ]),
                    armour: StackVec::new(&[
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Explosive,
                            base: 25,
                            roll: 10
                        },
                        ComponentValues {
                            perk: PerkName::Oblivious,
                            base: 25,
                            roll: 10
                        },
                        ComponentValues {
                            perk: PerkName::WildRunes,
                            base: 45,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::Preservationist,
                            base: 45,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::Hasty,
                            base: 45,
                            roll: 25
                        },
                    ])
                },
                MaterialName::MetallicParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Mediocrity,
                            base: 8,
                            roll: 33
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Bulwark,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Preparation,
                            base: 5,
                            roll: 15
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Confused,
                            base: 7,
                            roll: 27
                        },
                    ])
                },
                MaterialName::NoxiousComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Biting,
                            base: 40,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Biting,
                            base: 40,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::OceanicComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Invigorating,
                            base: 45,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Invigorating,
                            base: 45,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Polishing,
                            base: 25,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::WildRunes,
                            base: 15,
                            roll: 35
                        },
                    ])
                },
                MaterialName::OffcutComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                    ]),
                    armour: StackVec::new(&[
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Scraps,
                            base: 25,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::Careless,
                            base: 20,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Naturalist,
                            base: 20,
                            roll: 40
                        },
                    ])
                },
                MaterialName::OrganicParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Talking,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::Invigorating,
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::Inaccurate,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Mediocrity,
                            base: 8,
                            roll: 32
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Talking,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::Invigorating,
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::BriefRespite,
                            base: 5,
                            roll: 15
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Talking,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::Pyromaniac,
                            base: 7,
                            roll: 27
                        },
                    ])
                },
                MaterialName::PaddedParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::DemonBait,
                            base: 8,
                            roll: 32
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::DemonBait,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Absorbative,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Profane,
                            base: 8,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Polishing,
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::Butterfingers,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::Breakdown,
                            base: 7,
                            roll: 27
                        },
                    ])
                },
                MaterialName::PestiferousComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Venomblood,
                            base: 44,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::PiousComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Enlightened,
                            base: 9,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Hoarding,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Wise,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::DragonBait,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::Mediocrity,
                            base: 12,
                            roll: 44
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Enlightened,
                            base: 9,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Hoarding,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Wise,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::DragonBait,
                            base: 12,
                            roll: 45
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Enlightened,
                            base: 9,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Hoarding,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Wise,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Charitable,
                            base: 12,
                            roll: 44
                        },
                    ])
                },
                MaterialName::PlatedParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Blunted,
                            base: 8,
                            roll: 32
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Absorbative,
                            base: 5,
                            roll: 15
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 8,
                            roll: 33
                        },
                    ])
                },
                MaterialName::PowerfulComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Genocidal,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::TrophyTaker,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Blunted,
                            base: 12,
                            roll: 45
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Genocidal,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::TrophyTaker,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Bulwark,
                            base: 12,
                            roll: 40
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::PreciousComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Looting,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::Scavenging,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Spendthrift,
                            base: 9,
                            roll: 32
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Looting,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::Scavenging,
                            base: 12,
                            roll: 40
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 12,
                            roll: 45
                        },
                    ])
                },
                MaterialName::PreciseComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Blunted,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::Eruptive,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Precise,
                            base: 9,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Flanking,
                            base: 9,
                            roll: 32
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 12,
                            roll: 44
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Honed,
                            base: 9,
                            roll: 32
                        },
                    ])
                },
                MaterialName::ProtectiveComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Hoarding,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::DemonBait,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::ShieldBashing,
                            base: 12,
                            roll: 40
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Hoarding,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::DemonBait,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::ShieldBashing,
                            base: 12,
                            roll: 40
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Hoarding,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Polishing,
                            base: 12,
                            roll: 40
                        },
                    ])
                },
                MaterialName::RefinedComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Hoarding,
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::DragonBait,
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::DemonBait,
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::Energising,
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::Inaccurate,
                            base: 11,
                            roll: 25
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Hoarding,
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::DragonBait,
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::DemonBait,
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::Energising,
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::BriefRespite,
                            base: 11,
                            roll: 25
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Hoarding,
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::Refined,
                            base: 11,
                            roll: 25
                        },
                    ])
                },
                MaterialName::ResilientComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Bulwark,
                            base: 45,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Refined,
                            base: 25,
                            roll: 28
                        },
                    ])
                },
                MaterialName::RumblingComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Eruptive,
                            base: 40,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::SaradominComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Spendthrift,
                            base: 44,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Devoted,
                            base: 39,
                            roll: 9
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::SerenComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Enlightened,
                            base: 40,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::Wise,
                            base: 48,
                            roll: 5
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Enlightened,
                            base: 40,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::Wise,
                            base: 48,
                            roll: 5
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Enlightened,
                            base: 40,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::Wise,
                            base: 48,
                            roll: 5
                        },
                    ])
                },
                MaterialName::ShadowComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Caroming,
                            base: 40,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::SharpComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::DragonBait,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::Taunting,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Flanking,
                            base: 9,
                            roll: 32
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::DragonBait,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::Taunting,
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Honed,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Furnace,
                            base: 9,
                            roll: 33
                        },
                    ])
                },
                MaterialName::ShiftingComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Efficient,
                            base: 44,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::Ultimatums,
                            base: 45,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Efficient,
                            base: 44,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::Ultimatums,
                            base: 45,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Efficient,
                            base: 44,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::Rapid,
                            base: 25,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::Naturalist,
                            base: 15,
                            roll: 10
                        },
                    ])
                },
                MaterialName::SilentComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Lucky,
                            base: 45,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Honed,
                            base: 25,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::Preservationist,
                            base: 15,
                            roll: 35
                        },
                    ])
                },
                MaterialName::SimpleParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Looting,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Hallucinogenic,
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::Talking,
                            base: 7,
                            roll: 27
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Looting,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Hallucinogenic,
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::Talking,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::Profane,
                            base: 8,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Hallucinogenic,
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::Talking,
                            base: 7,
                            roll: 27
                        },
                    ])
                },
                MaterialName::SmoothParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Scavenging,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::UndeadBait,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Energising,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::Blunted,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Eruptive,
                            base: 5,
                            roll: 13
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Scavenging,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::UndeadBait,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Energising,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::Reflexes,
                            base: 5,
                            roll: 15
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Cheapskate,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Refined,
                            base: 7,
                            roll: 27
                        },
                    ])
                },
                MaterialName::SpikedParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::Fatiguing,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Genocidal,
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::TrophyTaker,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::Taunting,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::JunkFood,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Flanking,
                            base: 5,
                            roll: 15
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::Fatiguing,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Genocidal,
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::TrophyTaker,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::Taunting,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::JunkFood,
                            base: 8,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::Fatiguing,
                            base: 8,
                            roll: 32
                        },
                    ])
                },
                MaterialName::SpiritualParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Enlightened,
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Hoarding,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::Wise,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Inaccurate,
                            base: 8,
                            roll: 33
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Enlightened,
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Hoarding,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::Wise,
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Enlightened,
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::Antitheism,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Hoarding,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Cautious,
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::Wise,
                            base: 9,
                            roll: 33
                        },
                    ])
                },
                MaterialName::StaveParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::GlowWorm,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::Fatiguing,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::UndeadBait,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Energising,
                            base: 7,
                            roll: 27
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::GlowWorm,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::Fatiguing,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::UndeadBait,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Energising,
                            base: 7,
                            roll: 27
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::GlowWorm,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::Fatiguing,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Rapid,
                            base: 7,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::Prosper,
                            base: 5,
                            roll: 5
                        },
                    ])
                },
                MaterialName::StrongComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Fatiguing,
                            base: 12,
                            roll: 45
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Fatiguing,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::Absorbative,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::EnhancedDevoted,
                            base: 9,
                            roll: 32
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Committed,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Fatiguing,
                            base: 12,
                            roll: 45
                        },
                    ])
                },
                MaterialName::StunningComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Fatiguing,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::ClearHeaded,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Mediocrity,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Mysterious,
                            base: 12,
                            roll: 40
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Fatiguing,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::ClearHeaded,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Mysterious,
                            base: 12,
                            roll: 40
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Fatiguing,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::Mysterious,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::Confused,
                            base: 12,
                            roll: 40
                        },
                    ])
                },
                MaterialName::SubtleComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Looting,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::DemonBait,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::Mobile,
                            base: 12,
                            roll: 44
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Looting,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::DemonBait,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::Mobile,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::CrystalShield,
                            base: 12,
                            roll: 40
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Confused,
                            base: 12,
                            roll: 45
                        },
                    ])
                },
                MaterialName::SwiftComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Invigorating,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::ShieldBashing,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::Blunted,
                            base: 12,
                            roll: 45
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Invigorating,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::ShieldBashing,
                            base: 12,
                            roll: 40
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::ImpSouled,
                            base: 12,
                            roll: 40
                        },
                    ])
                },
                MaterialName::TensileParts => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Hoarding,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Blunted,
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::Mysterious,
                            base: 9,
                            roll: 33
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Hoarding,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Profane,
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Mysterious,
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Hoarding,
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::Mysterious,
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::Butterfingers,
                            base: 8,
                            roll: 32
                        },
                    ])
                },
                MaterialName::ThirdAgeComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::DemonSlayer,
                            base: 44,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::DemonSlayer,
                            base: 44,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Prosper,
                            base: 50,
                            roll: 50
                        },
                    ])
                },
                MaterialName::TimewornComponents => CompPerksPerGizmoType {
                    ancient_only: true,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Ruthless,
                            base: 30,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::Eruptive,
                            base: 26,
                            roll: 33
                        },
                    ]),
                    armour: StackVec::new(&[
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Fortune,
                            base: 36,
                            roll: 30
                        },
                        ComponentValues {
                            perk: PerkName::Prosper,
                            base: 13,
                            roll: 26
                        },
                    ])
                },
                MaterialName::UndeadComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Genocidal,
                            base: 40,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::UndeadSlayer,
                            base: 48,
                            roll: 5
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Genocidal,
                            base: 40,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::UndeadSlayer,
                            base: 48,
                            roll: 5
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Breakdown,
                            base: 25,
                            roll: 28
                        },
                    ])
                },
                MaterialName::VariableComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::TrophyTaker,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::DemonBait,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::ClearHeaded,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::EnhancedEfficient,
                            base: 9,
                            roll: 32
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::TrophyTaker,
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::DemonBait,
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::ClearHeaded,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::EnhancedEfficient,
                            base: 9,
                            roll: 32
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Cheapskate,
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::EnhancedEfficient,
                            base: 9,
                            roll: 32
                        },
                    ])
                },
                MaterialName::VintageComponents => CompPerksPerGizmoType {
                    ancient_only: true,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Relentless,
                            base: 50,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::Crackling,
                            base: 26,
                            roll: 33
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Relentless,
                            base: 50,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::Crackling,
                            base: 26,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Fortune,
                            base: 36,
                            roll: 30
                        },
                        ComponentValues {
                            perk: PerkName::Furnace,
                            base: 36,
                            roll: 30
                        },
                    ])
                },
                MaterialName::ZamorakComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Impatient,
                            base: 44,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Impatient,
                            base: 44,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::Devoted,
                            base: 39,
                            roll: 9
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::ImpSouled,
                            base: 28,
                            roll: 29
                        },
                    ])
                },
                MaterialName::ZarosComponents => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Impatient,
                            base: 44,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::Impatient,
                            base: 44,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::EnhancedDevoted,
                            base: 39,
                            roll: 9
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::ImpSouled,
                            base: 20,
                            roll: 25
                        },
                    ])
                },
            },
            perks: stack_map! {
                PerkName::Empty => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Empty,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Absorbative => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Absorbative,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Absorbative,
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Absorbative,
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Absorbative,
                            rank: 3,
                            cost: 175,
                            threshold: 220,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Absorbative,
                            rank: 4,
                            cost: 185,
                            threshold: 260,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Aftershock => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Aftershock,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Aftershock,
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Aftershock,
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Aftershock,
                            rank: 3,
                            cost: 175,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Aftershock,
                            rank: 4,
                            cost: 185,
                            threshold: 200,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Antitheism => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Antitheism,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Antitheism,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Biting => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Biting,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Biting,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Biting,
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Biting,
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Biting,
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Blunted => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Blunted,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Blunted,
                            rank: 1,
                            cost: 30,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Blunted,
                            rank: 2,
                            cost: 30,
                            threshold: 70,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Blunted,
                            rank: 3,
                            cost: 30,
                            threshold: 120,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Blunted,
                            rank: 4,
                            cost: 30,
                            threshold: 155,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Blunted,
                            rank: 5,
                            cost: 30,
                            threshold: 195,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Brassican => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Brassican,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Brassican,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Breakdown => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Breakdown,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Breakdown,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Breakdown,
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Breakdown,
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Breakdown,
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Breakdown,
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Breakdown,
                            rank: 6,
                            cost: 205,
                            threshold: 250,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::BriefRespite => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::BriefRespite,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::BriefRespite,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::BriefRespite,
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::BriefRespite,
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::BriefRespite,
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Bulwark => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Bulwark,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Bulwark,
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Bulwark,
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Bulwark,
                            rank: 3,
                            cost: 175,
                            threshold: 220,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Bulwark,
                            rank: 4,
                            cost: 185,
                            threshold: 260,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Butterfingers => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Butterfingers,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Butterfingers,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Butterfingers,
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Butterfingers,
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Butterfingers,
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Butterfingers,
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Careless => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Careless,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Careless,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Careless,
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Careless,
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Careless,
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Careless,
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Caroming => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Caroming,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Caroming,
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Caroming,
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Caroming,
                            rank: 3,
                            cost: 175,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Caroming,
                            rank: 4,
                            cost: 185,
                            threshold: 200,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Cautious => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Cautious,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Cautious,
                            rank: 1,
                            cost: 35,
                            threshold: 45,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Charitable => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Charitable,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Charitable,
                            rank: 1,
                            cost: 30,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Charitable,
                            rank: 2,
                            cost: 75,
                            threshold: 90,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Charitable,
                            rank: 3,
                            cost: 135,
                            threshold: 180,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Charitable,
                            rank: 4,
                            cost: 145,
                            threshold: 210,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Cheapskate => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Cheapskate,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Cheapskate,
                            rank: 1,
                            cost: 20,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Cheapskate,
                            rank: 2,
                            cost: 30,
                            threshold: 70,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Cheapskate,
                            rank: 3,
                            cost: 40,
                            threshold: 150,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::ClearHeaded => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::ClearHeaded,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::ClearHeaded,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::ClearHeaded,
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::ClearHeaded,
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::ClearHeaded,
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Committed => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Committed,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Committed,
                            rank: 1,
                            cost: 35,
                            threshold: 45,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Confused => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Confused,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Confused,
                            rank: 1,
                            cost: 20,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Confused,
                            rank: 2,
                            cost: 30,
                            threshold: 70,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Confused,
                            rank: 3,
                            cost: 40,
                            threshold: 150,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Crackling => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Crackling,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Crackling,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Crackling,
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Crackling,
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Crackling,
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::CrystalShield => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::CrystalShield,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::CrystalShield,
                            rank: 1,
                            cost: 30,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::CrystalShield,
                            rank: 2,
                            cost: 120,
                            threshold: 150,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::CrystalShield,
                            rank: 3,
                            cost: 150,
                            threshold: 180,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::CrystalShield,
                            rank: 4,
                            cost: 160,
                            threshold: 210,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::DemonBait => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::DemonBait,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::DemonBait,
                            rank: 1,
                            cost: 35,
                            threshold: 45,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::DemonSlayer => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::DemonSlayer,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::DemonSlayer,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Devoted => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Devoted,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Devoted,
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Devoted,
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Devoted,
                            rank: 3,
                            cost: 175,
                            threshold: 220,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Devoted,
                            rank: 4,
                            cost: 185,
                            threshold: 260,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::DragonBait => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::DragonBait,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::DragonBait,
                            rank: 1,
                            cost: 35,
                            threshold: 45,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::DragonSlayer => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::DragonSlayer,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::DragonSlayer,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Efficient => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Efficient,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Efficient,
                            rank: 1,
                            cost: 35,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Efficient,
                            rank: 2,
                            cost: 70,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Efficient,
                            rank: 3,
                            cost: 100,
                            threshold: 120,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Efficient,
                            rank: 4,
                            cost: 110,
                            threshold: 140,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Energising => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Energising,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Energising,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Energising,
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Energising,
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Energising,
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::EnhancedDevoted => PerkRanksData {
                    doubleslot: true,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::EnhancedDevoted,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: true
                        },
                        PerkRankValues {
                            name: PerkName::EnhancedDevoted,
                            rank: 1,
                            cost: 30,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: true
                        },
                        PerkRankValues {
                            name: PerkName::EnhancedDevoted,
                            rank: 2,
                            cost: 120,
                            threshold: 140,
                            ancient_only: false,
                            doubleslot: true
                        },
                        PerkRankValues {
                            name: PerkName::EnhancedDevoted,
                            rank: 3,
                            cost: 220,
                            threshold: 230,
                            ancient_only: false,
                            doubleslot: true
                        },
                        PerkRankValues {
                            name: PerkName::EnhancedDevoted,
                            rank: 4,
                            cost: 230,
                            threshold: 270,
                            ancient_only: true,
                            doubleslot: true
                        },
                    ])
                },
                PerkName::EnhancedEfficient => PerkRanksData {
                    doubleslot: true,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::EnhancedEfficient,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: true
                        },
                        PerkRankValues {
                            name: PerkName::EnhancedEfficient,
                            rank: 1,
                            cost: 30,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: true
                        },
                        PerkRankValues {
                            name: PerkName::EnhancedEfficient,
                            rank: 2,
                            cost: 120,
                            threshold: 140,
                            ancient_only: false,
                            doubleslot: true
                        },
                        PerkRankValues {
                            name: PerkName::EnhancedEfficient,
                            rank: 3,
                            cost: 220,
                            threshold: 230,
                            ancient_only: false,
                            doubleslot: true
                        },
                        PerkRankValues {
                            name: PerkName::EnhancedEfficient,
                            rank: 4,
                            cost: 230,
                            threshold: 270,
                            ancient_only: true,
                            doubleslot: true
                        },
                    ])
                },
                PerkName::Enlightened => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Enlightened,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Enlightened,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Enlightened,
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Enlightened,
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Enlightened,
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Eruptive => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Eruptive,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Eruptive,
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Eruptive,
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Eruptive,
                            rank: 3,
                            cost: 175,
                            threshold: 220,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Eruptive,
                            rank: 4,
                            cost: 185,
                            threshold: 260,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Explosive => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Explosive,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Explosive,
                            rank: 1,
                            cost: 50,
                            threshold: 65,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Fatiguing => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Fatiguing,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Fatiguing,
                            rank: 1,
                            cost: 35,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Fatiguing,
                            rank: 2,
                            cost: 35,
                            threshold: 90,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Fatiguing,
                            rank: 3,
                            cost: 35,
                            threshold: 180,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Flanking => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Flanking,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Flanking,
                            rank: 1,
                            cost: 30,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Flanking,
                            rank: 2,
                            cost: 110,
                            threshold: 140,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Flanking,
                            rank: 3,
                            cost: 180,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Flanking,
                            rank: 4,
                            cost: 190,
                            threshold: 250,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Fortune => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Fortune,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Fortune,
                            rank: 1,
                            cost: 45,
                            threshold: 60,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Fortune,
                            rank: 2,
                            cost: 90,
                            threshold: 130,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Fortune,
                            rank: 3,
                            cost: 180,
                            threshold: 250,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Furnace => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Furnace,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Furnace,
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Furnace,
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Furnace,
                            rank: 3,
                            cost: 175,
                            threshold: 220,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Furnace,
                            rank: 4,
                            cost: 185,
                            threshold: 260,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Genocidal => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Genocidal,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Genocidal,
                            rank: 1,
                            cost: 50,
                            threshold: 65,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::GlowWorm => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::GlowWorm,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::GlowWorm,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Hallucinogenic => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Hallucinogenic,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Hallucinogenic,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Hasty => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Hasty,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Hasty,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Hasty,
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Hasty,
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Hasty,
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Hasty,
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Hoarding => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Hoarding,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Hoarding,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Honed => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Honed,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Honed,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Honed,
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Honed,
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Honed,
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Honed,
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Honed,
                            rank: 6,
                            cost: 205,
                            threshold: 250,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::ImpSouled => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::ImpSouled,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::ImpSouled,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::ImpSouled,
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::ImpSouled,
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::ImpSouled,
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::ImpSouled,
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::ImpSouled,
                            rank: 6,
                            cost: 205,
                            threshold: 250,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Impatient => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Impatient,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Impatient,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Impatient,
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Impatient,
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Impatient,
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Inaccurate => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Inaccurate,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Inaccurate,
                            rank: 1,
                            cost: 30,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Inaccurate,
                            rank: 2,
                            cost: 30,
                            threshold: 70,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Inaccurate,
                            rank: 3,
                            cost: 30,
                            threshold: 120,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Inaccurate,
                            rank: 4,
                            cost: 30,
                            threshold: 155,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Inaccurate,
                            rank: 5,
                            cost: 30,
                            threshold: 195,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Invigorating => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Invigorating,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Invigorating,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Invigorating,
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Invigorating,
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Invigorating,
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::JunkFood => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::JunkFood,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::JunkFood,
                            rank: 1,
                            cost: 30,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::JunkFood,
                            rank: 2,
                            cost: 30,
                            threshold: 90,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::JunkFood,
                            rank: 3,
                            cost: 30,
                            threshold: 180,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Looting => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Looting,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Looting,
                            rank: 1,
                            cost: 50,
                            threshold: 65,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Lucky => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Lucky,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Lucky,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Lucky,
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Lucky,
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Lucky,
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Lucky,
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Lucky,
                            rank: 6,
                            cost: 205,
                            threshold: 250,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Lunging => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Lunging,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Lunging,
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Lunging,
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Lunging,
                            rank: 3,
                            cost: 175,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Lunging,
                            rank: 4,
                            cost: 185,
                            threshold: 200,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Mediocrity => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Mediocrity,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Mediocrity,
                            rank: 1,
                            cost: 30,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Mediocrity,
                            rank: 2,
                            cost: 30,
                            threshold: 90,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Mediocrity,
                            rank: 3,
                            cost: 30,
                            threshold: 180,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Mobile => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Mobile,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Mobile,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Mysterious => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Mysterious,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Mysterious,
                            rank: 1,
                            cost: 20,
                            threshold: 25,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Mysterious,
                            rank: 2,
                            cost: 30,
                            threshold: 45,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Mysterious,
                            rank: 3,
                            cost: 40,
                            threshold: 120,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Mysterious,
                            rank: 4,
                            cost: 50,
                            threshold: 175,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Mysterious,
                            rank: 5,
                            cost: 60,
                            threshold: 250,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Mysterious,
                            rank: 6,
                            cost: 70,
                            threshold: 300,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Naturalist => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Naturalist,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Naturalist,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Naturalist,
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Naturalist,
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Naturalist,
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Naturalist,
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Oblivious => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Oblivious,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Oblivious,
                            rank: 1,
                            cost: 50,
                            threshold: 65,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::PlantedFeet => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::PlantedFeet,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::PlantedFeet,
                            rank: 1,
                            cost: 60,
                            threshold: 60,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Polishing => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Polishing,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Polishing,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Polishing,
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Polishing,
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Polishing,
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Precise => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Precise,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Precise,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Precise,
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Precise,
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Precise,
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Precise,
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Precise,
                            rank: 6,
                            cost: 205,
                            threshold: 250,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Preparation => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Preparation,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Preparation,
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Preparation,
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Preparation,
                            rank: 3,
                            cost: 175,
                            threshold: 220,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Preparation,
                            rank: 4,
                            cost: 185,
                            threshold: 260,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Preservationist => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Preservationist,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Preservationist,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Preservationist,
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Preservationist,
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Preservationist,
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Preservationist,
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Profane => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Profane,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Profane,
                            rank: 1,
                            cost: 35,
                            threshold: 45,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Prosper => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Prosper,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Prosper,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Pyromaniac => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Pyromaniac,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Pyromaniac,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Pyromaniac,
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Pyromaniac,
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Pyromaniac,
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Pyromaniac,
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Pyromaniac,
                            rank: 6,
                            cost: 205,
                            threshold: 250,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Rapid => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Rapid,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Rapid,
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Rapid,
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Rapid,
                            rank: 3,
                            cost: 175,
                            threshold: 220,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Rapid,
                            rank: 4,
                            cost: 185,
                            threshold: 260,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Refined => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Refined,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Refined,
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Refined,
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Refined,
                            rank: 3,
                            cost: 175,
                            threshold: 220,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Refined,
                            rank: 4,
                            cost: 185,
                            threshold: 260,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Reflexes => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Reflexes,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Reflexes,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Relentless => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Relentless,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Relentless,
                            rank: 1,
                            cost: 45,
                            threshold: 60,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Relentless,
                            rank: 2,
                            cost: 75,
                            threshold: 130,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Relentless,
                            rank: 3,
                            cost: 130,
                            threshold: 200,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Relentless,
                            rank: 4,
                            cost: 170,
                            threshold: 280,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Relentless,
                            rank: 5,
                            cost: 205,
                            threshold: 380,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Ruthless => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Ruthless,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Ruthless,
                            rank: 1,
                            cost: 45,
                            threshold: 60,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Ruthless,
                            rank: 2,
                            cost: 90,
                            threshold: 130,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Ruthless,
                            rank: 3,
                            cost: 180,
                            threshold: 250,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Scavenging => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Scavenging,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Scavenging,
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Scavenging,
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Scavenging,
                            rank: 3,
                            cost: 175,
                            threshold: 220,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Scavenging,
                            rank: 4,
                            cost: 185,
                            threshold: 260,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Scraps => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Scraps,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Scraps,
                            rank: 1,
                            cost: 50,
                            threshold: 65,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::ShieldBashing => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::ShieldBashing,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::ShieldBashing,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::ShieldBashing,
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::ShieldBashing,
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::ShieldBashing,
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Spendthrift => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Spendthrift,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Spendthrift,
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Spendthrift,
                            rank: 2,
                            cost: 75,
                            threshold: 90,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Spendthrift,
                            rank: 3,
                            cost: 135,
                            threshold: 150,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Spendthrift,
                            rank: 4,
                            cost: 170,
                            threshold: 185,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Spendthrift,
                            rank: 5,
                            cost: 210,
                            threshold: 220,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Spendthrift,
                            rank: 6,
                            cost: 220,
                            threshold: 260,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Talking => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Talking,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Talking,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Taunting => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Taunting,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Taunting,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Tinker => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Tinker,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Tinker,
                            rank: 1,
                            cost: 20,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Tinker,
                            rank: 2,
                            cost: 30,
                            threshold: 70,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Tinker,
                            rank: 3,
                            cost: 40,
                            threshold: 150,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Tinker,
                            rank: 4,
                            cost: 50,
                            threshold: 180,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::TrophyTaker => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::TrophyTaker,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::TrophyTaker,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::TrophyTaker,
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::TrophyTaker,
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::TrophyTaker,
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::TrophyTaker,
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::TrophyTaker,
                            rank: 6,
                            cost: 205,
                            threshold: 250,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Turtling => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Turtling,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Turtling,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Turtling,
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Turtling,
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Turtling,
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Ultimatums => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Ultimatums,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Ultimatums,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Ultimatums,
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Ultimatums,
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Ultimatums,
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::UndeadBait => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::UndeadBait,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::UndeadBait,
                            rank: 1,
                            cost: 35,
                            threshold: 45,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::UndeadSlayer => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::UndeadSlayer,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::UndeadSlayer,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Venomblood => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Venomblood,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Venomblood,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::WildRunes => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::WildRunes,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::WildRunes,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::WildRunes,
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::WildRunes,
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::WildRunes,
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::WildRunes,
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::Wise => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::Wise,
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Wise,
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Wise,
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Wise,
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::Wise,
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
            },
        }
    }
}
