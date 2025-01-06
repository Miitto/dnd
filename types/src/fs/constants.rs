macro_rules! combine {
    ($A:expr, $B:expr) => {{
        const A: &str = $A;
        const B: &str = $B;
        const LEN: usize = A.len() + B.len();
        const fn combined() -> [u8; LEN] {
            let mut out = [0u8; LEN];
            out = copy_slice(A.as_bytes(), out, 0);
            out = copy_slice(B.as_bytes(), out, A.len());
            out
        }
        const fn copy_slice(input: &[u8], mut output: [u8; LEN], offset: usize) -> [u8; LEN] {
            let mut index = 0;
            loop {
                output[offset + index] = input[index];
                index += 1;
                if index == input.len() {
                    break;
                }
            }
            output
        }
        const RESULT: &[u8] = &combined();
        // how bad is the assumption that `&str` and `&[u8]` have the same layout?
        const RESULT_STR: &str = unsafe { std::str::from_utf8_unchecked(RESULT) };
        RESULT_STR
    }};
}

pub const ITEM_PATH: &str = "items";
pub const ITEM_WEAPON_PATH: &str = combine!(ITEM_PATH, "/weapons");
pub const ITEM_WEAPON_PROPERTIES_PATH: &str = combine!(ITEM_WEAPON_PATH, "/properties");
pub const RACE_PATH: &str = "races";
pub const BACKGROUND_PATH: &str = "backgrounds";
pub const CLASS_PATH: &str = "classes";
pub const CLASS_BASE_NAME: &str = "class";
pub const FEAT_PATH: &str = "feats";
pub const SPELL_PATH: &str = "spells";
pub const SPELL_CANTRIPS_PATH: &str = combine!(SPELL_PATH, "/cantrips");
pub const SPELL_LEVELS_PATH: &str = combine!(SPELL_PATH, "/levelled");
pub const SPELL_LIST_PATH: &str = combine!(SPELL_PATH, "/lists");
pub const STAT_BLOCK_PATH: &str = "stat_blocks";
