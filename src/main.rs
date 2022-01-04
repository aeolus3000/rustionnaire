use std::cell::RefCell;
use crate::Questionnaire;
use crate::Evaluation;
use crate::QuestItem;
use crate::Engine;

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
    items: Vec<QuestItem>
}

struct QuestItem {
    question: String,
    answers: Vec<String>,
    correctAnswer: u32,
}

struct Evaluation {
    questionnaire: Questionnaire,
}

struct Engine {

}

struct EvalSnippet {
    questItem: QuestItem,
    givenAnswer: dyn std::any::Any, //Todo: make compiler chill
}

impl Engine {

    fn execute_questionnaire(&self, questionnaire: Questionnaire) -> Evaluation {
        //for each question in Questionnaire call execute_question
        //and: add result to evaluation
        return Evaluation()
    }

    fn execute_quest_item(&self, item: QuestItem) -> Box<EvalSnippet> {
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



        // ä
    }

    fn show(&self, evaluation: Evaluation) {
        //given: questionnaire
        //when: questionnaire is fully answered
        //then: show summary (absolute / %)
        //and: output each question with correct answered are flagged and wrong answers are opposed to the correct solution o_O
    }
}
