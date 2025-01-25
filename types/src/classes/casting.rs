#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum CastType {
    Prepared,
    Known,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub enum CastLevel {
    Full,
    Half,
    Third,
    #[default]
    None,
}

impl CastLevel {
    pub fn max_level(&self) -> u8 {
        match self {
            Self::Full => 9,
            Self::Half => 5,
            Self::Third => 3,
            Self::None => 0,
        }
    }

    pub fn slots_at_level(&self, level: u8, slot_level: u8) -> u8 {
        match self {
            Self::Full => match slot_level {
                1 => match level {
                    0 => 0,
                    1 => 2,
                    2 => 3,
                    3..=u8::MAX => 4,
                },
                2 => match level {
                    0..=2 => 0,
                    3 => 2,
                    4..=u8::MAX => 3,
                },
                3 => match level {
                    0..=4 => 0,
                    5 => 2,
                    6..=u8::MAX => 3,
                },
                4 => match level {
                    0..=6 => 0,
                    7 => 1,
                    8 => 2,
                    9..=u8::MAX => 3,
                },
                5 => match level {
                    0..=8 => 0,
                    9 => 1,
                    10..=17 => 2,
                    18..=u8::MAX => 3,
                },
                6 => match level {
                    0..=10 => 0,
                    11..=18 => 1,
                    19..=u8::MAX => 2,
                },
                7 => match level {
                    0..=12 => 0,
                    13..=19 => 1,
                    20..=u8::MAX => 2,
                },
                8 => match level {
                    0..=14 => 0,
                    15..=u8::MAX => 1,
                },
                9 => match level {
                    0..=16 => 0,
                    17..=u8::MAX => 1,
                },
                _ => 0,
            },
            Self::Half => match slot_level {
                1 => match level {
                    0 => 0,
                    1..=2 => 2,
                    3..=4 => 3,
                    5..=u8::MAX => 4,
                },
                2 => match level {
                    0..=4 => 0,
                    5..=6 => 2,
                    7..=u8::MAX => 3,
                },
                3 => match level {
                    0..=8 => 0,
                    9..=10 => 2,
                    11..=u8::MAX => 3,
                },
                4 => match level {
                    0..=12 => 0,
                    13..=14 => 1,
                    15..=16 => 2,
                    17..=u8::MAX => 3,
                },
                5 => match level {
                    0..=16 => 0,
                    17..=18 => 1,
                    19..=u8::MAX => 2,
                },
                _ => 0,
            },
            Self::Third => match slot_level {
                1 => match level {
                    1..=2 => 2,
                    3..=4 => 3,
                    5..=6 => 3,
                    7..=8 => 3,
                    9..=20 => 3,
                    _ => 0,
                },
                2 => match level {
                    1..=2 => 0,
                    3..=4 => 2,
                    5..=6 => 2,
                    7..=8 => 2,
                    9..=20 => 2,
                    _ => 0,
                },
                3 => match level {
                    1..=2 => 0,
                    3..=4 => 0,
                    5..=6 => 2,
                    7..=8 => 2,
                    9..=20 => 2,
                    _ => 0,
                },
                4 => match level {
                    1..=2 => 0,
                    3..=4 => 0,
                    5..=6 => 0,
                    7..=8 => 1,
                    9..=20 => 1,
                    _ => 0,
                },
                5 => match level {
                    1..=2 => 0,
                    3..=4 => 0,
                    5..=6 => 0,
                    7..=8 => 0,
                    9..=20 => 1,
                    _ => 0,
                },
                _ => 0,
            },
            Self::None => 0,
        }
    }
}
