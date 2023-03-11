// #![allow(unused)]
// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use perk_solver::*;
// use smallvec::smallvec;

// pub fn get_empty_gizmo_chance_bench(c: &mut Criterion) {
//     let budget = Budget::create(45, true);
//     let pv = vec![
//         PerkValues {
//             name: PerkName::Precise,
//             i_first: 0,
//             i_last: 2,
//             ranks: StackVec::new(&[
//                 PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 0, cost: 0, ..Default::default() }, probability: 1.0/8.0 },
//                 PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 1, cost: 10, ..Default::default() }, probability: 1.0/8.0 },
//                 PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 2, cost: 50, ..Default::default() }, probability: 6.0/8.0 },
//                 PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 3, cost: 60, ..Default::default() }, probability: 0.0 },
//                 PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 5, cost: 100, ..Default::default() }, probability: 0.0 },
//             ]),
//             ..Default::default()
//         },
//         PerkValues {
//             name: PerkName::Biting,
//             i_first: 2,
//             i_last: 4,
//             ranks: StackVec::new(&[
//                 PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 0, cost: 0, ..Default::default() }, probability: 0.0 },
//                 PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 1, cost: 40, ..Default::default() }, probability: 0.0 },
//                 PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 2, cost: 50, ..Default::default() }, probability: 6.0/8.0 },
//                 PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 2, cost: 100, ..Default::default() }, probability: 6.0/8.0 },
//                 PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 2, cost: 200, ..Default::default() }, probability: 6.0/8.0 },
//             ]),
//             ..Default::default()
//         },
//     ];
//     c.bench_function("get_empty_gizmo_chance", |b| b.iter(|| {
//         get_empty_gizmo_chance(black_box(&budget), black_box(&pv))
//     }));
// }

// pub fn permutate_perk_ranks_bench(c: &mut Criterion) {
//     let perk_list: PerkValuesVec = smallvec![
//         PerkValues {
//             name: PerkName::Precise,
//             i_first: 1,
//             i_last: 3,
//             ranks: StackVec::new(&[
//                 PerkRankValuesProbabilityContainer { probability: 0.0, values: PerkRankValues { rank: 0, name: PerkName::Precise, ..Default::default() }},
//                 PerkRankValuesProbabilityContainer { probability: 0.125, values: PerkRankValues { rank: 1, name: PerkName::Precise, ..Default::default() }},
//                 PerkRankValuesProbabilityContainer { probability: 0.25, values: PerkRankValues { rank: 2, name: PerkName::Precise, ..Default::default() }},
//                 PerkRankValuesProbabilityContainer { probability: 0.5, values: PerkRankValues { rank: 3, name: PerkName::Precise, ..Default::default() }},
//             ]),
//             ..Default::default()
//         },
//         PerkValues {
//             name: PerkName::Biting,
//             i_first: 2,
//             i_last: 3,
//             ranks: StackVec::new(&[
//                 PerkRankValuesProbabilityContainer { probability: 0.0, values: PerkRankValues { rank: 0, name: PerkName::Biting, ..Default::default() }},
//                 PerkRankValuesProbabilityContainer { probability: 0.125, values: PerkRankValues { rank: 1, name: PerkName::Biting, ..Default::default() }},
//                 PerkRankValuesProbabilityContainer { probability: 0.25, values: PerkRankValues { rank: 2, name: PerkName::Biting, ..Default::default() }},
//                 PerkRankValuesProbabilityContainer { probability: 0.5, values: PerkRankValues { rank: 3, name: PerkName::Biting, ..Default::default() }},
//             ]),
//             ..Default::default()
//         },
//         PerkValues {
//             name: PerkName::Equilibrium,
//             i_first: 1,
//             i_last: 2,
//             ranks: StackVec::new(&[
//                 PerkRankValuesProbabilityContainer { probability: 0.0, values: PerkRankValues { rank: 0, name: PerkName::Equilibrium, ..Default::default() }},
//                 PerkRankValuesProbabilityContainer { probability: 0.25, values: PerkRankValues { rank: 1, name: PerkName::Equilibrium, ..Default::default() }},
//                 PerkRankValuesProbabilityContainer { probability: 0.5, values: PerkRankValues { rank: 2, name: PerkName::Equilibrium, ..Default::default() }},
//             ]),
//             ..Default::default()
//         },
//     ];
//     let wanted_gizmo = Gizmo {
//         perks: (
//             Perk { name: PerkName::Precise, rank: 2, ..Default::default() },
//             Perk { name: PerkName::Empty, ..Default::default() },
//         ),
//         ..Default::default()
//     };
//     c.bench_function("permutate_perk_ranks", |b| b.iter(|| {
//         permutate_perk_ranks(black_box(&perk_list), black_box(Some(wanted_gizmo)))
//     }));
// }



// criterion_group!(benches, permutate_perk_ranks_bench);
// criterion_main!(benches);

fn main() {}