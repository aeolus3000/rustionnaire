use crate::utils::questitem::*;
use crate::utils::userio::UserIO;
use crate::utils::userio::testuserio::TestUserIO;
use crate::utils::engine::Engine;

pub struct Questionnaire {
    pub(crate) items: Vec<QuestItem>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_questionnaire_returns_empty_evaluation() {
        // given: empty questionnaire
        let questionnaire = Questionnaire {
            items: vec![],
        };

        // and: a user IO
        let user_io: TestUserIO = UserIO::new();

        // and: an engine
        let engine = Engine {user_io};

        // when: executing the questionnaire
        let evaluation = engine.execute_questionnaire(&questionnaire);

        // then: an empty evaluation should be returned
        assert!(evaluation.questionnaire.items.is_empty());
        assert!(evaluation.given_answers.is_empty());
    }

}