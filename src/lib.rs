use core::cmp::{Ord, Ordering};
use std::ops::{Add, AddAssign, Sub, SubAssign};

mod bindings;
pub use bindings::game::auto_rogue::types::{
    ActionTarget, AttackParams, Direction, EquipmentSlot, MicroAction,
};
pub use bindings::{
    actions, config_data, creature_at, events, export, get_equipment_state, inventory, item_at,
    visible_tiles, Guest, *,
};

impl Default for Loc {
    fn default() -> Loc {
        Loc { x: 0, y: 0 }
    }
}

impl Add for Loc {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Loc {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl AddAssign for Loc {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
impl SubAssign for Loc {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl Loc {
    pub fn magnitude(&self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }
}
impl Ord for Loc {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x, self.y).cmp(&(other.x, other.y))
    }
}

impl PartialOrd for Loc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl From<Direction> for String {
    fn from(slot: Direction) -> String {
        match slot {
            Direction::North => "North".to_string(),
            Direction::NorthEast => "NorthEast".to_string(),
            Direction::East => "East".to_string(),
            Direction::SouthEast => "SouthEast".to_string(),
            Direction::South => "South".to_string(),
            Direction::SouthWest => "SouthWest".to_string(),
            Direction::West => "West".to_string(),
            Direction::NorthWest => "NorthWest".to_string(),
        }
    }
}

impl From<EquipmentSlot> for String {
    fn from(slot: EquipmentSlot) -> String {
        match slot {
            EquipmentSlot::LeftHand => "LeftHand".to_string(),
            EquipmentSlot::RightHand => "RightHand".to_string(),
        }
    }
}

impl std::convert::TryFrom<String> for EquipmentSlot {
    type Error = anyhow::Error;
    fn try_from(slot: String) -> anyhow::Result<EquipmentSlot> {
        Ok(match slot.as_str() {
            "LeftHand" => EquipmentSlot::LeftHand,
            "RightHand" => EquipmentSlot::RightHand,
            _ => anyhow::bail!("Unknown slot: {slot}"),
        })
    }
}
