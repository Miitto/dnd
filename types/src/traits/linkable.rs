use std::sync::{Arc, Mutex};

use crate::{meta::Table, spells::Spell, stat_block::StatBlock};

pub trait Linkable {
    fn link(&mut self) -> &mut Self {
        self.link_tables().link_stat_blocks()
    }
    fn link_tables(&mut self) -> &mut Self {
        self
    }
    fn link_stat_blocks(&mut self) -> &mut Self {
        self
    }

    #[allow(unused_variables)]
    fn link_external_tables(&mut self, tables: &[Arc<Mutex<Table>>]) -> &mut Self {
        self
    }

    #[allow(unused_variables)]
    fn link_external_stat_blocks(&mut self, stat_blocks: &[Arc<Mutex<StatBlock>>]) -> &mut Self {
        self
    }

    #[allow(unused_variables)]
    fn link_external_spells(&mut self, spells: &[Arc<Mutex<Spell>>]) -> &mut Self {
        self
    }
}
