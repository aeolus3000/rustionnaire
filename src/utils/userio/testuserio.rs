use std::cell::RefCell;
use super::UserIO;
use crate::utils::questitem::*;

pub struct TestUserIO {
    given_answers: RefCell<Vec<i32>>,
    next_answer: i32,
}

impl UserIO for TestUserIO {
    fn new() -> TestUserIO {
        TestUserIO {
            given_answers: RefCell::new(vec![1, 2, 3]),
            next_answer: 0,
        }
    }

    fn collect_answer(&self, quest_item: &QuestItem) -> i32 {
        let v = match self.given_answers.borrow_mut().pop() {
            Some(value) => value,
            None => 0,
        };
        return v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_answer_returns_answer_to_question() {
        //given: a quest item
        let answers = vec!["-1", "0", "3"].iter()
            .map(|s| s.to_string()).collect();

        let quest_item = QuestItem {
            question: String::from("how old are you?"),
            answers,
            correct_answer: 2,
        };
        // and: a user IO
        let user_io = TestUserIO::new();

        //when: presenting the quest item to the user
        let answer = user_io.collect_answer(&quest_item);

        //then: the method collect answer should return the user input
        assert_eq!(answer, 3);

        //when: presenting the quest item to the user
        let answer = user_io.collect_answer(&quest_item);

        //then: the method collect answer should return the user input
        assert_eq!(answer, 2);
    }
}
