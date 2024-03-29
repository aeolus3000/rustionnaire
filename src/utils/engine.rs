use crate::utils::evaluation::Evaluation;
use crate::utils::userio::testuserio::TestUserIO;
use crate::utils::questionnaire::Questionnaire;
use crate::utils::questitem::QuestItem;

pub struct Engine {
    //TODO: we want virtual dispatch here
    pub user_io: TestUserIO,
}

impl Engine {
    pub(crate) fn execute_questionnaire<'a>(&self, questionnaire: &'a Questionnaire) -> Evaluation<'a> {
        //for each question in Questionnaire call execute_question
        //and: add result to evaluation
        let number_of_questions = questionnaire.items.len();
        let given_answers = vec![0; number_of_questions];

        Evaluation {
            questionnaire,
            given_answers,
        }
    }

    fn execute_quest_item(&self, item: QuestItem) -> i32 {
        //showQuestionText()
        //consumeAnswer() -> actual
        //Tupelize questitem and given answer into EvalSnippet
        //return

        //given: 1 question, multiple answers, 1 correct answer
        //when: user enters correct answer
        //then: score = 1
        //and: evaluation text correct answer given


        //given: 1 question, multiple answers, 1 correct answer
        //when: user enters incorrect answer
        //then: score = 0
        //and: evaluation text your answer vs correct answer

        0
    }

    fn show(&self, evaluation: Evaluation) {
        //given: questionnaire
        //when: questionnaire is fully answered
        //then: show summary (absolute / %)
        //and: output each question with correct answered are flagged and wrong answers are opposed to the correct solution o_O
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::userio::UserIO;


    #[test]
    fn number_of_questions_should_match_number_of_answers() {
        // given: a QuestItem
        let answers = vec!["-1", "0", "3"].iter()
            .map(|s| s.to_string()).collect();

        let quest_item = QuestItem {
            question: String::from("how old are you?"),
            answers,
            correct_answer: 2,
        };

        // and: a questionnaire with questions
        let questionnaire = Questionnaire {
            items: vec![quest_item],
        };

        // and: a user IO
        let user_io = TestUserIO::new();

        // and: an engine
        let engine = Engine { user_io };

        // when: executing the questionnaire
        let evaluation = engine.execute_questionnaire(&questionnaire);

        // then: the number of questions should match the number of answers
        assert_eq!(evaluation.questionnaire.items.len(), evaluation.given_answers.len());
        // and: the number of answers should match the given questionnaire
        assert_eq!(evaluation.given_answers.len(), 1);

        //then: find all questions


        //then: find all expected answers
    }
}