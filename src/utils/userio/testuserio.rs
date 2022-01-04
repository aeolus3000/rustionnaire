use std::borrow::BorrowMut;
use std::cell::RefCell;
use super::UserIO;
use crate::utils::questitem::*;

pub struct TestUserIO {
    given_answers: Vec<i32>,
    next_answer:  RefCell<usize>,
}


impl TestUserIO {

    fn new(rolling_given_answers: Vec<i32>) -> TestUserIO {
        TestUserIO {
            given_answers: rolling_given_answers,
            next_answer: RefCell::new(0),
        }
    }
}

impl UserIO for TestUserIO {
    fn new() -> Self {
        TestUserIO::new(vec![1])
    }

    fn collect_answer(&self, quest_item: &QuestItem) -> i32 {
        // primitive type, so no need to copy :) since this happen automatically for them on deref, legal notice: Copyright @siko
        let current = *self.next_answer.borrow();

        // SLOW POKES could use this very unfancy thing::: return self.given_answers[next_answer];
        let v = match self.given_answers.get(current) {
            Some(&value) => value,
            None => 0,
        };
        self.next_answer.replace((current + 1) % self.given_answers.len());
        return v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_answer_returns_answer_to_question() {
        //given: a quest item
        let answers = vec![">20", "21-39", "<40"].iter()
            .map(|s| s.to_string()).collect();

        let quest_item = QuestItem {
            question: String::from("how old are you?"),
            answers,
            correct_answer: 2,
        };
        // and: a user IO
        let user_io = TestUserIO::new(vec![0,1,99]);

        //when: presenting the quest item to the user
        let answer = user_io.collect_answer(&quest_item);

        //then: the method collect answer should return the user input
        assert_eq!(answer, 0);

        //when: presenting the quest item to the user
        let answer = user_io.collect_answer(&quest_item);

        //then: the method collect answer should return the user input
        assert_eq!(answer, 1);
    }
}
