pub mod prelude;
pub mod stack_map;

use crate::{MaterialName, PerkName};
pub use prelude::*;
pub use stack_map::*;
use std::str::FromStr;
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
                MaterialName::from_str("Armadyl components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Precise").unwrap(),
                            base: 44,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Devoted").unwrap(),
                            base: 39,
                            roll: 9
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Charitable").unwrap(),
                            base: 25,
                            roll: 28
                        },
                    ])
                },
                MaterialName::from_str("Ascended components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Efficient").unwrap(),
                            base: 40,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Enhanced Efficient").unwrap(),
                            base: 20,
                            roll: 25
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Efficient").unwrap(),
                            base: 40,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Enhanced Efficient").unwrap(),
                            base: 20,
                            roll: 25
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Efficient").unwrap(),
                            base: 40,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Enhanced Efficient").unwrap(),
                            base: 20,
                            roll: 25
                        },
                    ])
                },
                MaterialName::from_str("Avernic components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Lunging").unwrap(),
                            base: 44,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::from_str("Bandos components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Genocidal").unwrap(),
                            base: 44,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Genocidal").unwrap(),
                            base: 44,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Devoted").unwrap(),
                            base: 39,
                            roll: 9
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::from_str("Base parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Talking").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Dragon Slayer").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Inaccurate").unwrap(),
                            base: 8,
                            roll: 33
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Talking").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Dragon Slayer").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Turtling").unwrap(),
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Talking").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Charitable").unwrap(),
                            base: 7,
                            roll: 25
                        },
                    ])
                },
                MaterialName::from_str("Blade parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Biting").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Junk Food").unwrap(),
                            base: 8,
                            roll: 33
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Biting").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Junk Food").unwrap(),
                            base: 8,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Honed").unwrap(),
                            base: 7,
                            roll: 25
                        },
                    ])
                },
                MaterialName::from_str("Brassican components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Hallucinogenic").unwrap(),
                            base: 36,
                            roll: 9
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Talking").unwrap(),
                            base: 45,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Brassican").unwrap(),
                            base: 49,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Hallucinogenic").unwrap(),
                            base: 36,
                            roll: 9
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Talking").unwrap(),
                            base: 45,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Brassican").unwrap(),
                            base: 49,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Hallucinogenic").unwrap(),
                            base: 36,
                            roll: 9
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Talking").unwrap(),
                            base: 45,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Brassican").unwrap(),
                            base: 49,
                            roll: 8
                        },
                    ])
                },
                MaterialName::from_str("Classic components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: true,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Scavenging").unwrap(),
                            base: 12,
                            roll: 20
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Efficient").unwrap(),
                            base: 12,
                            roll: 40
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Scavenging").unwrap(),
                            base: 12,
                            roll: 20
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Efficient").unwrap(),
                            base: 12,
                            roll: 40
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Fortune").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Furnace").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Efficient").unwrap(),
                            base: 12,
                            roll: 40
                        },
                    ])
                },
                MaterialName::from_str("Clear parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Glow Worm").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Hallucinogenic").unwrap(),
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Demon Slayer").unwrap(),
                            base: 5,
                            roll: 15
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Glow Worm").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Hallucinogenic").unwrap(),
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Demon Slayer").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Profane").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Crystal Shield").unwrap(),
                            base: 5,
                            roll: 15
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Glow Worm").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Hallucinogenic").unwrap(),
                            base: 7,
                            roll: 28
                        },
                    ])
                },
                MaterialName::from_str("Clockwork components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Enhanced Efficient").unwrap(),
                            base: 45,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Flanking").unwrap(),
                            base: 25,
                            roll: 28
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Enhanced Efficient").unwrap(),
                            base: 45,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Enhanced Efficient").unwrap(),
                            base: 45,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Tinker").unwrap(),
                            base: 18,
                            roll: 28
                        },
                    ])
                },
                MaterialName::from_str("Connector parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Scavenging").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Dragon Bait").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Undead Bait").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mobile").unwrap(),
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Precise").unwrap(),
                            base: 5,
                            roll: 15
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Scavenging").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Dragon Bait").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Undead Bait").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mobile").unwrap(),
                            base: 7,
                            roll: 28
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::from_str("Corporeal components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Brief Respite").unwrap(),
                            base: 40,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::from_str("Cover parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Shield Bashing").unwrap(),
                            base: 5,
                            roll: 15
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Shield Bashing").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Profane").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Bulwark").unwrap(),
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Confused").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Furnace").unwrap(),
                            base: 7,
                            roll: 25
                        },
                    ])
                },
                MaterialName::from_str("Crafted parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Hallucinogenic").unwrap(),
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Fatiguing").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mediocrity").unwrap(),
                            base: 8,
                            roll: 33
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Hallucinogenic").unwrap(),
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Fatiguing").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Preparation").unwrap(),
                            base: 5,
                            roll: 15
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Hallucinogenic").unwrap(),
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Fatiguing").unwrap(),
                            base: 8,
                            roll: 32
                        },
                    ])
                },
                MaterialName::from_str("Crystal parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Efficient").unwrap(),
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Trophy-taker's").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Undead Slayer").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Demon Bait").unwrap(),
                            base: 8,
                            roll: 32
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Efficient").unwrap(),
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Trophy-taker's").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Undead Slayer").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Demon Bait").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Turtling").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Profane").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Enhanced Devoted").unwrap(),
                            base: 5,
                            roll: 15
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Efficient").unwrap(),
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Cheapskate").unwrap(),
                            base: 8,
                            roll: 32
                        },
                    ])
                },
                MaterialName::from_str("Culinary components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Invigorating").unwrap(),
                            base: 39,
                            roll: 9
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Invigorating").unwrap(),
                            base: 39,
                            roll: 9
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Brief Respite").unwrap(),
                            base: 45,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::from_str("Cywir components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Planted Feet").unwrap(),
                            base: 20,
                            roll: 20
                        },
                    ]),
                    armour: StackVec::new(&[
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::from_str("Deflecting parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Shield Bashing").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Inaccurate").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mediocrity").unwrap(),
                            base: 8,
                            roll: 32
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Shield Bashing").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Venomblood").unwrap(),
                            base: 5,
                            roll: 15
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Imp Souled").unwrap(),
                            base: 7,
                            roll: 27
                        },
                    ])
                },
                MaterialName::from_str("Delicate parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Demon Bait").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Junk Food").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Eruptive").unwrap(),
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Enhanced Efficient").unwrap(),
                            base: 5,
                            roll: 15
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Demon Bait").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Junk Food").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Lucky").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Enhanced Efficient").unwrap(),
                            base: 5,
                            roll: 15
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Butterfingers").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Charitable").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Enhanced Efficient").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Tinker").unwrap(),
                            base: 7,
                            roll: 25
                        },
                    ])
                },
                MaterialName::from_str("Dextrous components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Demon Slayer").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Dragon Bait").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mobile").unwrap(),
                            base: 12,
                            roll: 44
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Demon Slayer").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Dragon Bait").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mobile").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Reflexes").unwrap(),
                            base: 12,
                            roll: 40
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Polishing").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Butterfingers").unwrap(),
                            base: 9,
                            roll: 33
                        },
                    ])
                },
                MaterialName::from_str("Direct components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Biting").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Blunted").unwrap(),
                            base: 12,
                            roll: 45
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Biting").unwrap(),
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Charitable").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Pyromaniac").unwrap(),
                            base: 9,
                            roll: 32
                        },
                    ])
                },
                MaterialName::from_str("Dragonfire components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Dragon Slayer").unwrap(),
                            base: 44,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Dragon Slayer").unwrap(),
                            base: 44,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Furnace").unwrap(),
                            base: 25,
                            roll: 28
                        },
                    ])
                },
                MaterialName::from_str("Enhancing components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Dragon Slayer").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Invigorating").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Energising").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Inaccurate").unwrap(),
                            base: 12,
                            roll: 45
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Dragon Slayer").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Invigorating").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Energising").unwrap(),
                            base: 12,
                            roll: 44
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Cheapskate").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Refined").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Rapid").unwrap(),
                            base: 12,
                            roll: 40
                        },
                    ])
                },
                MaterialName::from_str("Ethereal components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Wise").unwrap(),
                            base: 12,
                            roll: 40
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Wise").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Brief Respite").unwrap(),
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Wise").unwrap(),
                            base: 12,
                            roll: 40
                        },
                    ])
                },
                MaterialName::from_str("Evasive components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Blunted").unwrap(),
                            base: 12,
                            roll: 45
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Turtling").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Venomblood").unwrap(),
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Prosper").unwrap(),
                            base: 12,
                            roll: 8
                        },
                    ])
                },
                MaterialName::from_str("Explosive components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Crackling").unwrap(),
                            base: 48,
                            roll: 5
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Ultimatums").unwrap(),
                            base: 40,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Crackling").unwrap(),
                            base: 48,
                            roll: 5
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Ultimatums").unwrap(),
                            base: 40,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Pyromaniac").unwrap(),
                            base: 25,
                            roll: 28
                        },
                    ])
                },
                MaterialName::from_str("Faceted components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Crystal Shield").unwrap(),
                            base: 39,
                            roll: 9
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Enhanced Devoted").unwrap(),
                            base: 45,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::from_str("Flexible parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Wise").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Dragon Bait").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mobile").unwrap(),
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Clear Headed").unwrap(),
                            base: 5,
                            roll: 15
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Wise").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Dragon Bait").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mobile").unwrap(),
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Clear Headed").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Profane").unwrap(),
                            base: 8,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Wise").unwrap(),
                            base: 9,
                            roll: 33
                        },
                    ])
                },
                MaterialName::from_str("Fortunate components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Looting").unwrap(),
                            base: 13,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Hoarding").unwrap(),
                            base: 13,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Brassican").unwrap(),
                            base: 13,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Spendthrift").unwrap(),
                            base: 13,
                            roll: 40
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Looting").unwrap(),
                            base: 13,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Hoarding").unwrap(),
                            base: 13,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Brassican").unwrap(),
                            base: 13,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Lucky").unwrap(),
                            base: 13,
                            roll: 40
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Hoarding").unwrap(),
                            base: 13,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Brassican").unwrap(),
                            base: 13,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Polishing").unwrap(),
                            base: 13,
                            roll: 40
                        },
                    ])
                },
                MaterialName::from_str("Fungal components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Absorbative").unwrap(),
                            base: 40,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Tinker").unwrap(),
                            base: 10,
                            roll: 28
                        },
                    ])
                },
                MaterialName::from_str("Harnessed components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Reflexes").unwrap(),
                            base: 45,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Preparation").unwrap(),
                            base: 44,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::from_str("Head parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Talking").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Clear Headed").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Inaccurate").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mediocrity").unwrap(),
                            base: 8,
                            roll: 32
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Talking").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Clear Headed").unwrap(),
                            base: 5,
                            roll: 15
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Talking").unwrap(),
                            base: 7,
                            roll: 27
                        },
                    ])
                },
                MaterialName::from_str("Healthy components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Efficient").unwrap(),
                            base: 9,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Inaccurate").unwrap(),
                            base: 12,
                            roll: 45
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Efficient").unwrap(),
                            base: 9,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Venomblood").unwrap(),
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Efficient").unwrap(),
                            base: 9,
                            roll: 32
                        },
                    ])
                },
                MaterialName::from_str("Heavy components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Demon Bait").unwrap(),
                            base: 12,
                            roll: 45
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Demon Bait").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Bulwark").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Preparation").unwrap(),
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Butterfingers").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Breakdown").unwrap(),
                            base: 12,
                            roll: 40
                        },
                    ])
                },
                MaterialName::from_str("Historic components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: true,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Precise").unwrap(),
                            base: 11,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Genocidal").unwrap(),
                            base: 11,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Ultimatums").unwrap(),
                            base: 11,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Looting").unwrap(),
                            base: 11,
                            roll: 33
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Genocidal").unwrap(),
                            base: 11,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Ultimatums").unwrap(),
                            base: 11,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Looting").unwrap(),
                            base: 11,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Turtling").unwrap(),
                            base: 11,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Imp Souled").unwrap(),
                            base: 11,
                            roll: 33
                        },
                    ])
                },
                MaterialName::from_str("Ilujankan components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Aftershock").unwrap(),
                            base: 40,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::from_str("Imbued components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Crackling").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Ultimatums").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Junk Food").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Energising").unwrap(),
                            base: 12,
                            roll: 44
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Crackling").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Ultimatums").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Junk Food").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Energising").unwrap(),
                            base: 12,
                            roll: 44
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Furnace").unwrap(),
                            base: 12,
                            roll: 40
                        },
                    ])
                },
                MaterialName::from_str("Junk").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                    ]),
                    armour: StackVec::new(&[
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::from_str("Knightly components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Taunting").unwrap(),
                            base: 44,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Taunting").unwrap(),
                            base: 44,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::from_str("Light components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Glow Worm").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Hallucinogenic").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Inaccurate").unwrap(),
                            base: 12,
                            roll: 45
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Glow Worm").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Hallucinogenic").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Lucky").unwrap(),
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Glow Worm").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Hallucinogenic").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Pyromaniac").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Rapid").unwrap(),
                            base: 9,
                            roll: 33
                        },
                    ])
                },
                MaterialName::from_str("Living components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Talking").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Undead Slayer").unwrap(),
                            base: 9,
                            roll: 33
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Talking").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Undead Slayer").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Profane").unwrap(),
                            base: 12,
                            roll: 45
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Talking").unwrap(),
                            base: 12,
                            roll: 44
                        },
                    ])
                },
                MaterialName::from_str("Magic parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Undead Bait").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Crackling").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Ultimatums").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Energising").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Spendthrift").unwrap(),
                            base: 5,
                            roll: 13
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Undead Bait").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Crackling").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Ultimatums").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Energising").unwrap(),
                            base: 7,
                            roll: 27
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Honed").unwrap(),
                            base: 7,
                            roll: 27
                        },
                    ])
                },
                MaterialName::from_str("Metallic parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mediocrity").unwrap(),
                            base: 8,
                            roll: 33
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Bulwark").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Preparation").unwrap(),
                            base: 5,
                            roll: 15
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Confused").unwrap(),
                            base: 7,
                            roll: 27
                        },
                    ])
                },
                MaterialName::from_str("Noxious components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Biting").unwrap(),
                            base: 40,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Biting").unwrap(),
                            base: 40,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::from_str("Oceanic components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Invigorating").unwrap(),
                            base: 45,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Invigorating").unwrap(),
                            base: 45,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Polishing").unwrap(),
                            base: 25,
                            roll: 28
                        },
                    ])
                },
                MaterialName::from_str("Organic parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Talking").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Invigorating").unwrap(),
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Inaccurate").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mediocrity").unwrap(),
                            base: 8,
                            roll: 32
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Talking").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Invigorating").unwrap(),
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Brief Respite").unwrap(),
                            base: 5,
                            roll: 15
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Talking").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Pyromaniac").unwrap(),
                            base: 7,
                            roll: 27
                        },
                    ])
                },
                MaterialName::from_str("Padded parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Demon Bait").unwrap(),
                            base: 8,
                            roll: 32
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Demon Bait").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Absorbative").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Profane").unwrap(),
                            base: 8,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Polishing").unwrap(),
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Butterfingers").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Breakdown").unwrap(),
                            base: 7,
                            roll: 27
                        },
                    ])
                },
                MaterialName::from_str("Pestiferous components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Venomblood").unwrap(),
                            base: 44,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::from_str("Pious components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Enlightened").unwrap(),
                            base: 9,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Hoarding").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Wise").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Dragon Bait").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mediocrity").unwrap(),
                            base: 12,
                            roll: 44
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Enlightened").unwrap(),
                            base: 9,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Hoarding").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Wise").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Dragon Bait").unwrap(),
                            base: 12,
                            roll: 45
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Enlightened").unwrap(),
                            base: 9,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Hoarding").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Wise").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Charitable").unwrap(),
                            base: 12,
                            roll: 44
                        },
                    ])
                },
                MaterialName::from_str("Plated parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Blunted").unwrap(),
                            base: 8,
                            roll: 32
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Absorbative").unwrap(),
                            base: 5,
                            roll: 15
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 8,
                            roll: 33
                        },
                    ])
                },
                MaterialName::from_str("Powerful components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Genocidal").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Trophy-taker's").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Blunted").unwrap(),
                            base: 12,
                            roll: 45
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Genocidal").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Trophy-taker's").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Bulwark").unwrap(),
                            base: 12,
                            roll: 40
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::from_str("Precious components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Looting").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Scavenging").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Spendthrift").unwrap(),
                            base: 9,
                            roll: 32
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Looting").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Scavenging").unwrap(),
                            base: 12,
                            roll: 40
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 12,
                            roll: 45
                        },
                    ])
                },
                MaterialName::from_str("Precise components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Blunted").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Eruptive").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Precise").unwrap(),
                            base: 9,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Flanking").unwrap(),
                            base: 9,
                            roll: 32
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 12,
                            roll: 44
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Honed").unwrap(),
                            base: 9,
                            roll: 32
                        },
                    ])
                },
                MaterialName::from_str("Protective components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Hoarding").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Demon Bait").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Shield Bashing").unwrap(),
                            base: 12,
                            roll: 40
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Hoarding").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Demon Bait").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Shield Bashing").unwrap(),
                            base: 12,
                            roll: 40
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Hoarding").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Polishing").unwrap(),
                            base: 12,
                            roll: 40
                        },
                    ])
                },
                MaterialName::from_str("Refined components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Hoarding").unwrap(),
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Dragon Bait").unwrap(),
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Demon Bait").unwrap(),
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Energising").unwrap(),
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Inaccurate").unwrap(),
                            base: 11,
                            roll: 25
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Hoarding").unwrap(),
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Dragon Bait").unwrap(),
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Demon Bait").unwrap(),
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Energising").unwrap(),
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Brief Respite").unwrap(),
                            base: 11,
                            roll: 25
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Hoarding").unwrap(),
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 11,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Refined").unwrap(),
                            base: 11,
                            roll: 25
                        },
                    ])
                },
                MaterialName::from_str("Resilient components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Bulwark").unwrap(),
                            base: 45,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Refined").unwrap(),
                            base: 25,
                            roll: 28
                        },
                    ])
                },
                MaterialName::from_str("Rumbling components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Eruptive").unwrap(),
                            base: 40,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::from_str("Saradomin components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Spendthrift").unwrap(),
                            base: 44,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Devoted").unwrap(),
                            base: 39,
                            roll: 9
                        },
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::from_str("Seren components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Enlightened").unwrap(),
                            base: 40,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Wise").unwrap(),
                            base: 48,
                            roll: 5
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Enlightened").unwrap(),
                            base: 40,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Wise").unwrap(),
                            base: 48,
                            roll: 5
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Enlightened").unwrap(),
                            base: 40,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Wise").unwrap(),
                            base: 48,
                            roll: 5
                        },
                    ])
                },
                MaterialName::from_str("Shadow components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Caroming").unwrap(),
                            base: 40,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                    ]),
                    tool: StackVec::new(&[
                    ])
                },
                MaterialName::from_str("Sharp components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Dragon Bait").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Taunting").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Flanking").unwrap(),
                            base: 9,
                            roll: 32
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Dragon Bait").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Taunting").unwrap(),
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Honed").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Furnace").unwrap(),
                            base: 9,
                            roll: 33
                        },
                    ])
                },
                MaterialName::from_str("Shifting components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Efficient").unwrap(),
                            base: 44,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Ultimatums").unwrap(),
                            base: 45,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Efficient").unwrap(),
                            base: 44,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Ultimatums").unwrap(),
                            base: 45,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Efficient").unwrap(),
                            base: 44,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Rapid").unwrap(),
                            base: 25,
                            roll: 28
                        },
                    ])
                },
                MaterialName::from_str("Silent components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Lucky").unwrap(),
                            base: 45,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Honed").unwrap(),
                            base: 25,
                            roll: 28
                        },
                    ])
                },
                MaterialName::from_str("Simple parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Looting").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Hallucinogenic").unwrap(),
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Talking").unwrap(),
                            base: 7,
                            roll: 27
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Looting").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Hallucinogenic").unwrap(),
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Talking").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Profane").unwrap(),
                            base: 8,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Hallucinogenic").unwrap(),
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Talking").unwrap(),
                            base: 7,
                            roll: 27
                        },
                    ])
                },
                MaterialName::from_str("Smooth parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Scavenging").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Undead Bait").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Energising").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Blunted").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Eruptive").unwrap(),
                            base: 5,
                            roll: 13
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Scavenging").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Undead Bait").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Energising").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Reflexes").unwrap(),
                            base: 5,
                            roll: 15
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Cheapskate").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Refined").unwrap(),
                            base: 7,
                            roll: 27
                        },
                    ])
                },
                MaterialName::from_str("Spiked parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Fatiguing").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Genocidal").unwrap(),
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Trophy-taker's").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Taunting").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Junk Food").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Flanking").unwrap(),
                            base: 5,
                            roll: 15
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Fatiguing").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Genocidal").unwrap(),
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Trophy-taker's").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Taunting").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Junk Food").unwrap(),
                            base: 8,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Fatiguing").unwrap(),
                            base: 8,
                            roll: 32
                        },
                    ])
                },
                MaterialName::from_str("Spiritual parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Enlightened").unwrap(),
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Hoarding").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Wise").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Inaccurate").unwrap(),
                            base: 8,
                            roll: 33
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Enlightened").unwrap(),
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Hoarding").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Wise").unwrap(),
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Enlightened").unwrap(),
                            base: 5,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Antitheism").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Hoarding").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Cautious").unwrap(),
                            base: 7,
                            roll: 27
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Wise").unwrap(),
                            base: 9,
                            roll: 33
                        },
                    ])
                },
                MaterialName::from_str("Stave parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Glow Worm").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Fatiguing").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Undead Bait").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Energising").unwrap(),
                            base: 7,
                            roll: 27
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Glow Worm").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Fatiguing").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Undead Bait").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Energising").unwrap(),
                            base: 7,
                            roll: 27
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Glow Worm").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 7,
                            roll: 28
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Fatiguing").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Rapid").unwrap(),
                            base: 7,
                            roll: 25
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Prosper").unwrap(),
                            base: 5,
                            roll: 5
                        },
                    ])
                },
                MaterialName::from_str("Strong components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Fatiguing").unwrap(),
                            base: 12,
                            roll: 45
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Fatiguing").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Absorbative").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Enhanced Devoted").unwrap(),
                            base: 9,
                            roll: 32
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Committed").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Fatiguing").unwrap(),
                            base: 12,
                            roll: 45
                        },
                    ])
                },
                MaterialName::from_str("Stunning components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Fatiguing").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Clear Headed").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mediocrity").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mysterious").unwrap(),
                            base: 12,
                            roll: 40
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Fatiguing").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Clear Headed").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mysterious").unwrap(),
                            base: 12,
                            roll: 40
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Fatiguing").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mysterious").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Confused").unwrap(),
                            base: 12,
                            roll: 40
                        },
                    ])
                },
                MaterialName::from_str("Subtle components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Looting").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Demon Bait").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mobile").unwrap(),
                            base: 12,
                            roll: 44
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Looting").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Demon Bait").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mobile").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Crystal Shield").unwrap(),
                            base: 12,
                            roll: 40
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Confused").unwrap(),
                            base: 12,
                            roll: 45
                        },
                    ])
                },
                MaterialName::from_str("Swift components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Invigorating").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Shield Bashing").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Blunted").unwrap(),
                            base: 12,
                            roll: 45
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Invigorating").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Shield Bashing").unwrap(),
                            base: 12,
                            roll: 40
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Imp Souled").unwrap(),
                            base: 12,
                            roll: 40
                        },
                    ])
                },
                MaterialName::from_str("Tensile parts").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Hoarding").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Blunted").unwrap(),
                            base: 8,
                            roll: 32
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mysterious").unwrap(),
                            base: 9,
                            roll: 33
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Hoarding").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Profane").unwrap(),
                            base: 8,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mysterious").unwrap(),
                            base: 9,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Hoarding").unwrap(),
                            base: 5,
                            roll: 15
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Mysterious").unwrap(),
                            base: 9,
                            roll: 33
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Butterfingers").unwrap(),
                            base: 8,
                            roll: 32
                        },
                    ])
                },
                MaterialName::from_str("Third-age components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Demon Slayer").unwrap(),
                            base: 44,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Demon Slayer").unwrap(),
                            base: 44,
                            roll: 8
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Prosper").unwrap(),
                            base: 50,
                            roll: 50
                        },
                    ])
                },
                MaterialName::from_str("Timeworn components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: true,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Ruthless").unwrap(),
                            base: 30,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Eruptive").unwrap(),
                            base: 26,
                            roll: 33
                        },
                    ]),
                    armour: StackVec::new(&[
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Fortune").unwrap(),
                            base: 36,
                            roll: 30
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Prosper").unwrap(),
                            base: 13,
                            roll: 26
                        },
                    ])
                },
                MaterialName::from_str("Undead components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Genocidal").unwrap(),
                            base: 40,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Undead Slayer").unwrap(),
                            base: 48,
                            roll: 5
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Genocidal").unwrap(),
                            base: 40,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Undead Slayer").unwrap(),
                            base: 48,
                            roll: 5
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Breakdown").unwrap(),
                            base: 25,
                            roll: 28
                        },
                    ])
                },
                MaterialName::from_str("Variable components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Trophy-taker's").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Demon Bait").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Clear Headed").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Enhanced Efficient").unwrap(),
                            base: 9,
                            roll: 32
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Trophy-taker's").unwrap(),
                            base: 12,
                            roll: 44
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Demon Bait").unwrap(),
                            base: 12,
                            roll: 45
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Clear Headed").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Enhanced Efficient").unwrap(),
                            base: 9,
                            roll: 32
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Cheapskate").unwrap(),
                            base: 12,
                            roll: 40
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Enhanced Efficient").unwrap(),
                            base: 9,
                            roll: 32
                        },
                    ])
                },
                MaterialName::from_str("Vintage components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: true,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Relentless").unwrap(),
                            base: 50,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Crackling").unwrap(),
                            base: 26,
                            roll: 33
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Relentless").unwrap(),
                            base: 50,
                            roll: 13
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Crackling").unwrap(),
                            base: 26,
                            roll: 33
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Fortune").unwrap(),
                            base: 36,
                            roll: 30
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Furnace").unwrap(),
                            base: 36,
                            roll: 30
                        },
                    ])
                },
                MaterialName::from_str("Zamorak components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Impatient").unwrap(),
                            base: 44,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Impatient").unwrap(),
                            base: 44,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Devoted").unwrap(),
                            base: 39,
                            roll: 9
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Imp Souled").unwrap(),
                            base: 28,
                            roll: 29
                        },
                    ])
                },
                MaterialName::from_str("Zaros components").unwrap() => CompPerksPerGizmoType {
                    ancient_only: false,
                    weapon: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Impatient").unwrap(),
                            base: 44,
                            roll: 8
                        },
                    ]),
                    armour: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Impatient").unwrap(),
                            base: 44,
                            roll: 8
                        },
                        ComponentValues {
                            perk: PerkName::from_str("Enhanced Devoted").unwrap(),
                            base: 39,
                            roll: 9
                        },
                    ]),
                    tool: StackVec::new(&[
                        ComponentValues {
                            perk: PerkName::from_str("Imp Souled").unwrap(),
                            base: 20,
                            roll: 25
                        },
                    ])
                },
            },
            perks: stack_map! {
                PerkName::from_str("Empty").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Empty").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Absorbative").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Absorbative").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Absorbative").unwrap(),
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Absorbative").unwrap(),
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Absorbative").unwrap(),
                            rank: 3,
                            cost: 175,
                            threshold: 220,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Absorbative").unwrap(),
                            rank: 4,
                            cost: 185,
                            threshold: 260,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Aftershock").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Aftershock").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Aftershock").unwrap(),
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Aftershock").unwrap(),
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Aftershock").unwrap(),
                            rank: 3,
                            cost: 175,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Aftershock").unwrap(),
                            rank: 4,
                            cost: 185,
                            threshold: 200,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Antitheism").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Antitheism").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Antitheism").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Biting").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Biting").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Biting").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Biting").unwrap(),
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Biting").unwrap(),
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Biting").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Blunted").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Blunted").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Blunted").unwrap(),
                            rank: 1,
                            cost: 30,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Blunted").unwrap(),
                            rank: 2,
                            cost: 30,
                            threshold: 70,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Blunted").unwrap(),
                            rank: 3,
                            cost: 30,
                            threshold: 120,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Blunted").unwrap(),
                            rank: 4,
                            cost: 30,
                            threshold: 155,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Blunted").unwrap(),
                            rank: 5,
                            cost: 30,
                            threshold: 195,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Brassican").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Brassican").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Brassican").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Breakdown").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Breakdown").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Breakdown").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Breakdown").unwrap(),
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Breakdown").unwrap(),
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Breakdown").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Breakdown").unwrap(),
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Breakdown").unwrap(),
                            rank: 6,
                            cost: 205,
                            threshold: 250,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Brief Respite").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Brief Respite").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Brief Respite").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Brief Respite").unwrap(),
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Brief Respite").unwrap(),
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Brief Respite").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Bulwark").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Bulwark").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Bulwark").unwrap(),
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Bulwark").unwrap(),
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Bulwark").unwrap(),
                            rank: 3,
                            cost: 175,
                            threshold: 220,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Bulwark").unwrap(),
                            rank: 4,
                            cost: 185,
                            threshold: 260,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Butterfingers").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Butterfingers").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Butterfingers").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Butterfingers").unwrap(),
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Butterfingers").unwrap(),
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Butterfingers").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Butterfingers").unwrap(),
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Caroming").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Caroming").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Caroming").unwrap(),
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Caroming").unwrap(),
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Caroming").unwrap(),
                            rank: 3,
                            cost: 175,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Caroming").unwrap(),
                            rank: 4,
                            cost: 185,
                            threshold: 200,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Cautious").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Cautious").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Cautious").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 45,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Charitable").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Charitable").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Charitable").unwrap(),
                            rank: 1,
                            cost: 30,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Charitable").unwrap(),
                            rank: 2,
                            cost: 75,
                            threshold: 90,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Charitable").unwrap(),
                            rank: 3,
                            cost: 135,
                            threshold: 180,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Charitable").unwrap(),
                            rank: 4,
                            cost: 145,
                            threshold: 210,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Cheapskate").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Cheapskate").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Cheapskate").unwrap(),
                            rank: 1,
                            cost: 20,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Cheapskate").unwrap(),
                            rank: 2,
                            cost: 30,
                            threshold: 70,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Cheapskate").unwrap(),
                            rank: 3,
                            cost: 40,
                            threshold: 150,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Clear Headed").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Clear Headed").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Clear Headed").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Clear Headed").unwrap(),
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Clear Headed").unwrap(),
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Clear Headed").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Committed").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Committed").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Committed").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 45,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Confused").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Confused").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Confused").unwrap(),
                            rank: 1,
                            cost: 20,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Confused").unwrap(),
                            rank: 2,
                            cost: 30,
                            threshold: 70,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Confused").unwrap(),
                            rank: 3,
                            cost: 40,
                            threshold: 150,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Crackling").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Crackling").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Crackling").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Crackling").unwrap(),
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Crackling").unwrap(),
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Crackling").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Crystal Shield").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Crystal Shield").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Crystal Shield").unwrap(),
                            rank: 1,
                            cost: 30,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Crystal Shield").unwrap(),
                            rank: 2,
                            cost: 120,
                            threshold: 150,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Crystal Shield").unwrap(),
                            rank: 3,
                            cost: 150,
                            threshold: 180,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Crystal Shield").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 210,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Demon Bait").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Demon Bait").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Demon Bait").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 45,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Demon Slayer").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Demon Slayer").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Demon Slayer").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Devoted").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Devoted").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Devoted").unwrap(),
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Devoted").unwrap(),
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Devoted").unwrap(),
                            rank: 3,
                            cost: 175,
                            threshold: 220,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Devoted").unwrap(),
                            rank: 4,
                            cost: 185,
                            threshold: 260,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Dragon Bait").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Dragon Bait").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Dragon Bait").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 45,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Dragon Slayer").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Dragon Slayer").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Dragon Slayer").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Efficient").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Efficient").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Efficient").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Efficient").unwrap(),
                            rank: 2,
                            cost: 70,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Efficient").unwrap(),
                            rank: 3,
                            cost: 100,
                            threshold: 120,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Efficient").unwrap(),
                            rank: 4,
                            cost: 110,
                            threshold: 140,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Energising").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Energising").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Energising").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Energising").unwrap(),
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Energising").unwrap(),
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Energising").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Enhanced Devoted").unwrap() => PerkRanksData {
                    doubleslot: true,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Enhanced Devoted").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: true
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Enhanced Devoted").unwrap(),
                            rank: 1,
                            cost: 30,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: true
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Enhanced Devoted").unwrap(),
                            rank: 2,
                            cost: 120,
                            threshold: 140,
                            ancient_only: false,
                            doubleslot: true
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Enhanced Devoted").unwrap(),
                            rank: 3,
                            cost: 220,
                            threshold: 230,
                            ancient_only: false,
                            doubleslot: true
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Enhanced Devoted").unwrap(),
                            rank: 4,
                            cost: 230,
                            threshold: 270,
                            ancient_only: true,
                            doubleslot: true
                        },
                    ])
                },
                PerkName::from_str("Enhanced Efficient").unwrap() => PerkRanksData {
                    doubleslot: true,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Enhanced Efficient").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: true
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Enhanced Efficient").unwrap(),
                            rank: 1,
                            cost: 30,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: true
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Enhanced Efficient").unwrap(),
                            rank: 2,
                            cost: 120,
                            threshold: 140,
                            ancient_only: false,
                            doubleslot: true
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Enhanced Efficient").unwrap(),
                            rank: 3,
                            cost: 220,
                            threshold: 230,
                            ancient_only: false,
                            doubleslot: true
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Enhanced Efficient").unwrap(),
                            rank: 4,
                            cost: 230,
                            threshold: 270,
                            ancient_only: true,
                            doubleslot: true
                        },
                    ])
                },
                PerkName::from_str("Enlightened").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Enlightened").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Enlightened").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Enlightened").unwrap(),
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Enlightened").unwrap(),
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Enlightened").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Eruptive").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Eruptive").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Eruptive").unwrap(),
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Eruptive").unwrap(),
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Eruptive").unwrap(),
                            rank: 3,
                            cost: 175,
                            threshold: 220,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Eruptive").unwrap(),
                            rank: 4,
                            cost: 185,
                            threshold: 260,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Fatiguing").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Fatiguing").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Fatiguing").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Fatiguing").unwrap(),
                            rank: 2,
                            cost: 35,
                            threshold: 90,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Fatiguing").unwrap(),
                            rank: 3,
                            cost: 35,
                            threshold: 180,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Flanking").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Flanking").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Flanking").unwrap(),
                            rank: 1,
                            cost: 30,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Flanking").unwrap(),
                            rank: 2,
                            cost: 110,
                            threshold: 140,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Flanking").unwrap(),
                            rank: 3,
                            cost: 180,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Flanking").unwrap(),
                            rank: 4,
                            cost: 190,
                            threshold: 250,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Fortune").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Fortune").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Fortune").unwrap(),
                            rank: 1,
                            cost: 45,
                            threshold: 60,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Fortune").unwrap(),
                            rank: 2,
                            cost: 90,
                            threshold: 130,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Fortune").unwrap(),
                            rank: 3,
                            cost: 180,
                            threshold: 250,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Furnace").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Furnace").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Furnace").unwrap(),
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Furnace").unwrap(),
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Furnace").unwrap(),
                            rank: 3,
                            cost: 175,
                            threshold: 220,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Furnace").unwrap(),
                            rank: 4,
                            cost: 185,
                            threshold: 260,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Genocidal").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Genocidal").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Genocidal").unwrap(),
                            rank: 1,
                            cost: 50,
                            threshold: 65,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Glow Worm").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Glow Worm").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Glow Worm").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Hallucinogenic").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Hallucinogenic").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Hallucinogenic").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Hoarding").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Hoarding").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Hoarding").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Honed").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Honed").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Honed").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Honed").unwrap(),
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Honed").unwrap(),
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Honed").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Honed").unwrap(),
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Honed").unwrap(),
                            rank: 6,
                            cost: 205,
                            threshold: 250,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Imp Souled").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Imp Souled").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Imp Souled").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Imp Souled").unwrap(),
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Imp Souled").unwrap(),
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Imp Souled").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Imp Souled").unwrap(),
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Imp Souled").unwrap(),
                            rank: 6,
                            cost: 205,
                            threshold: 250,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Impatient").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Impatient").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Impatient").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Impatient").unwrap(),
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Impatient").unwrap(),
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Impatient").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Inaccurate").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Inaccurate").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Inaccurate").unwrap(),
                            rank: 1,
                            cost: 30,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Inaccurate").unwrap(),
                            rank: 2,
                            cost: 30,
                            threshold: 70,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Inaccurate").unwrap(),
                            rank: 3,
                            cost: 30,
                            threshold: 120,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Inaccurate").unwrap(),
                            rank: 4,
                            cost: 30,
                            threshold: 155,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Inaccurate").unwrap(),
                            rank: 5,
                            cost: 30,
                            threshold: 195,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Invigorating").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Invigorating").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Invigorating").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Invigorating").unwrap(),
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Invigorating").unwrap(),
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Invigorating").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Junk Food").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Junk Food").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Junk Food").unwrap(),
                            rank: 1,
                            cost: 30,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Junk Food").unwrap(),
                            rank: 2,
                            cost: 30,
                            threshold: 90,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Junk Food").unwrap(),
                            rank: 3,
                            cost: 30,
                            threshold: 180,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Looting").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Looting").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Looting").unwrap(),
                            rank: 1,
                            cost: 50,
                            threshold: 65,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Lucky").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Lucky").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Lucky").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Lucky").unwrap(),
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Lucky").unwrap(),
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Lucky").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Lucky").unwrap(),
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Lucky").unwrap(),
                            rank: 6,
                            cost: 205,
                            threshold: 250,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Lunging").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Lunging").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Lunging").unwrap(),
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Lunging").unwrap(),
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Lunging").unwrap(),
                            rank: 3,
                            cost: 175,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Lunging").unwrap(),
                            rank: 4,
                            cost: 185,
                            threshold: 200,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Mediocrity").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Mediocrity").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Mediocrity").unwrap(),
                            rank: 1,
                            cost: 30,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Mediocrity").unwrap(),
                            rank: 2,
                            cost: 30,
                            threshold: 90,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Mediocrity").unwrap(),
                            rank: 3,
                            cost: 30,
                            threshold: 180,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Mobile").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Mobile").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Mobile").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Mysterious").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Mysterious").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Mysterious").unwrap(),
                            rank: 1,
                            cost: 20,
                            threshold: 25,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Mysterious").unwrap(),
                            rank: 2,
                            cost: 30,
                            threshold: 45,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Mysterious").unwrap(),
                            rank: 3,
                            cost: 40,
                            threshold: 120,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Mysterious").unwrap(),
                            rank: 4,
                            cost: 50,
                            threshold: 175,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Mysterious").unwrap(),
                            rank: 5,
                            cost: 60,
                            threshold: 250,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Mysterious").unwrap(),
                            rank: 6,
                            cost: 70,
                            threshold: 300,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Planted Feet").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Planted Feet").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Planted Feet").unwrap(),
                            rank: 1,
                            cost: 60,
                            threshold: 60,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Polishing").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Polishing").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Polishing").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Polishing").unwrap(),
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Polishing").unwrap(),
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Polishing").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Precise").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Precise").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Precise").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Precise").unwrap(),
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Precise").unwrap(),
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Precise").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Precise").unwrap(),
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Precise").unwrap(),
                            rank: 6,
                            cost: 205,
                            threshold: 250,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Preparation").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Preparation").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Preparation").unwrap(),
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Preparation").unwrap(),
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Preparation").unwrap(),
                            rank: 3,
                            cost: 175,
                            threshold: 220,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Preparation").unwrap(),
                            rank: 4,
                            cost: 185,
                            threshold: 260,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Profane").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Profane").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Profane").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 45,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Prosper").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Prosper").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Prosper").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Pyromaniac").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Pyromaniac").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Pyromaniac").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Pyromaniac").unwrap(),
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Pyromaniac").unwrap(),
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Pyromaniac").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Pyromaniac").unwrap(),
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Pyromaniac").unwrap(),
                            rank: 6,
                            cost: 205,
                            threshold: 250,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Rapid").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Rapid").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Rapid").unwrap(),
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Rapid").unwrap(),
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Rapid").unwrap(),
                            rank: 3,
                            cost: 175,
                            threshold: 220,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Rapid").unwrap(),
                            rank: 4,
                            cost: 185,
                            threshold: 260,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Refined").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Refined").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Refined").unwrap(),
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Refined").unwrap(),
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Refined").unwrap(),
                            rank: 3,
                            cost: 175,
                            threshold: 220,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Refined").unwrap(),
                            rank: 4,
                            cost: 185,
                            threshold: 260,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Reflexes").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Reflexes").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Reflexes").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Relentless").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Relentless").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Relentless").unwrap(),
                            rank: 1,
                            cost: 45,
                            threshold: 60,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Relentless").unwrap(),
                            rank: 2,
                            cost: 75,
                            threshold: 130,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Relentless").unwrap(),
                            rank: 3,
                            cost: 130,
                            threshold: 200,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Relentless").unwrap(),
                            rank: 4,
                            cost: 170,
                            threshold: 280,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Relentless").unwrap(),
                            rank: 5,
                            cost: 205,
                            threshold: 380,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Ruthless").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Ruthless").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Ruthless").unwrap(),
                            rank: 1,
                            cost: 45,
                            threshold: 60,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Ruthless").unwrap(),
                            rank: 2,
                            cost: 90,
                            threshold: 130,
                            ancient_only: true,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Ruthless").unwrap(),
                            rank: 3,
                            cost: 180,
                            threshold: 250,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Scavenging").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Scavenging").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Scavenging").unwrap(),
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Scavenging").unwrap(),
                            rank: 2,
                            cost: 90,
                            threshold: 115,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Scavenging").unwrap(),
                            rank: 3,
                            cost: 175,
                            threshold: 220,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Scavenging").unwrap(),
                            rank: 4,
                            cost: 185,
                            threshold: 260,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Shield Bashing").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Shield Bashing").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Shield Bashing").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Shield Bashing").unwrap(),
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Shield Bashing").unwrap(),
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Shield Bashing").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Spendthrift").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Spendthrift").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Spendthrift").unwrap(),
                            rank: 1,
                            cost: 40,
                            threshold: 55,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Spendthrift").unwrap(),
                            rank: 2,
                            cost: 75,
                            threshold: 90,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Spendthrift").unwrap(),
                            rank: 3,
                            cost: 135,
                            threshold: 150,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Spendthrift").unwrap(),
                            rank: 4,
                            cost: 170,
                            threshold: 185,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Spendthrift").unwrap(),
                            rank: 5,
                            cost: 210,
                            threshold: 220,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Spendthrift").unwrap(),
                            rank: 6,
                            cost: 220,
                            threshold: 260,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Talking").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Talking").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Talking").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Taunting").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Taunting").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Taunting").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Tinker").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Tinker").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Tinker").unwrap(),
                            rank: 1,
                            cost: 20,
                            threshold: 40,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Tinker").unwrap(),
                            rank: 2,
                            cost: 30,
                            threshold: 70,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Tinker").unwrap(),
                            rank: 3,
                            cost: 40,
                            threshold: 150,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Tinker").unwrap(),
                            rank: 4,
                            cost: 50,
                            threshold: 180,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Trophy-taker's").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Trophy-taker's").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Trophy-taker's").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Trophy-taker's").unwrap(),
                            rank: 2,
                            cost: 65,
                            threshold: 80,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Trophy-taker's").unwrap(),
                            rank: 3,
                            cost: 120,
                            threshold: 130,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Trophy-taker's").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 170,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Trophy-taker's").unwrap(),
                            rank: 5,
                            cost: 195,
                            threshold: 210,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Trophy-taker's").unwrap(),
                            rank: 6,
                            cost: 205,
                            threshold: 250,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Turtling").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Turtling").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Turtling").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Turtling").unwrap(),
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Turtling").unwrap(),
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Turtling").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Ultimatums").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Ultimatums").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Ultimatums").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Ultimatums").unwrap(),
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Ultimatums").unwrap(),
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Ultimatums").unwrap(),
                            rank: 4,
                            cost: 160,
                            threshold: 240,
                            ancient_only: true,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Undead Bait").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Undead Bait").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Undead Bait").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 45,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Undead Slayer").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Undead Slayer").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Undead Slayer").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Venomblood").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Venomblood").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Venomblood").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                    ])
                },
                PerkName::from_str("Wise").unwrap() => PerkRanksData {
                    doubleslot: false,
                    ranks: StackVec::new(&[
                        PerkRankValues {
                            name: PerkName::from_str("Wise").unwrap(),
                            rank: 0,
                            cost: 0,
                            threshold: 0,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Wise").unwrap(),
                            rank: 1,
                            cost: 35,
                            threshold: 50,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Wise").unwrap(),
                            rank: 2,
                            cost: 80,
                            threshold: 100,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Wise").unwrap(),
                            rank: 3,
                            cost: 150,
                            threshold: 200,
                            ancient_only: false,
                            doubleslot: false
                        },
                        PerkRankValues {
                            name: PerkName::from_str("Wise").unwrap(),
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
