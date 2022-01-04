#[derive(Clone)]
pub struct QuestItem {
    pub(crate) question: String,
    pub(crate) answers: Vec<String>,
    pub(crate) correct_answer: u32,
}

#[cfg(test)]
mod tests {

}