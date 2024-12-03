use crate::dice::Dice;

use super::weapon::DamageType;

#[derive(Debug, Clone)]
pub struct Damage {
    pub damage: Dice,
    pub damage_type: DamageType,
}
