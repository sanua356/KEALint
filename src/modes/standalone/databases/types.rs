use crate::checkers::Problem;

pub struct DatabaseResults<C> {
    pub connection: C,
}

pub trait DatabaseResultsChecker<R> {
    fn run_migrations(&self) -> R;
    fn load_results(&self, config: String, problems: Vec<Problem>) -> R;
}
