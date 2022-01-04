pub mod testuserio;

use crate::utils::questitem::*;

pub trait UserIO {
    fn new() -> Self;
    fn collect_answer(&self, quest_item: &QuestItem) -> i32;
}
