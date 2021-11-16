fn main() {
    filename = getFilenameFromStdIn();
    questionnaire = readQuestionnair(filename);
    result = executeQuestionnaire(questionnaire);
    printResult(result);
}
