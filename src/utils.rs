use clap::ValueEnum;
use derive_more::Display;

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