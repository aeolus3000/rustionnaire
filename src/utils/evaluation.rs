use crate::utils::questionnaire::Questionnaire;

pub struct Evaluation<'a> {
    pub(crate) questionnaire: &'a Questionnaire,
    pub(crate) given_answers: Vec<i32>,
}

#[cfg(test)]
mod tests {

}