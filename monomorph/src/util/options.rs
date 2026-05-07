
#[derive(Clone, Debug)]
pub struct AnalysisOptions;

impl Default for AnalysisOptions {
    fn default() -> Self {
        Self {}
    }
}

impl AnalysisOptions {
    /// Parses options from a list of strings. Any content beyond the leftmost `--` token
    /// will be returned (excluding this token).
    pub fn parse_from_args(&mut self, _args: &[String], _from_env: bool) -> Vec<String> {
        // TODO
        vec![]
    }
}
