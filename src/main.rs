fn main() {
    /*
    let engine = Engine{};
    let filename = getFilenameFromStdIn();
    let questionnaire = readQuestionnaire(filename);
    let evaluation = engine.execute_questionnaire(questionnaire);
    engine.show(evaluation);
     */
}


struct Questionnaire {
    items: Vec<QuestItem>,
}

struct QuestItem {
    question: String,
    answers: Vec<String>,
    correctAnswer: u32,
}


struct Evaluation {
    questionnaire: Questionnaire,
    given_answers: Vec<i32>,
}

struct Engine {}

impl Engine {
    fn execute_questionnaire(&self, questionnaire: Questionnaire) -> Evaluation {
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

    #[test]
    fn empty_questionnaire_returns_empty_evaluation() {
        // given: empty questionnaire
        let questionnaire = Questionnaire {
            items: vec![],
        };

        // and: an engine
        let engine = Engine {};

        // when: executing the questionnaire
        let evaluation = engine.execute_questionnaire(questionnaire);

        // then: an empty evaluation should be returned
        assert!(evaluation.questionnaire.items.is_empty());
        assert!(evaluation.given_answers.is_empty());
    }

    #[test]
    fn number_of_questions_should_match_number_of_answers() {
        // given: a QuestItem
        let answers = vec!["-1", "0", "3"].iter()
            .map(|s| s.to_string()).collect();

        let quest_item = QuestItem {
            question: String::from("how old are you?"),
            answers,
            correctAnswer: 2,
        };

        // and: a questionnaire with questions
        let questionnaire = Questionnaire {
            items: vec![quest_item],
        };

        // and: an engine
        let engine = Engine {};

        // when: executing the questionnaire
        let evaluation = engine.execute_questionnaire(questionnaire);

        // then: the number of questions should match the number of answers
        assert_eq!(evaluation.questionnaire.items.len(), evaluation.given_answers.len());
        // and: the number of answers should match the given questionnaire
        assert_eq!(evaluation.given_answers.len(), 1);
    }
}

