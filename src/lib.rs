//! Deterministic experiment evaluation against explicit expectations.
//! No untrusted code execution is performed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Experiment {
    pub id: String,
    pub hypothesis: String,
    pub expected: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    pub experiment_id: String,
    pub observed: String,
    pub matched: bool,
}
pub fn evaluate(experiment: &Experiment, observed: &str) -> Result<Outcome, &'static str> {
    if experiment.id.is_empty() || experiment.hypothesis.is_empty() || experiment.expected.is_empty() {
        return Err("incomplete experiment");
    }
    Ok(Outcome {
        experiment_id: experiment.id.clone(),
        observed: observed.into(),
        matched: observed == experiment.expected,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn experiment() -> Experiment {
        Experiment {
            id: "exp-1".into(),
            hypothesis: "agent returns expected result".into(),
            expected: "verified".into(),
        }
    }
    #[test]
    fn accepts_matching_observation() {
        assert!(evaluate(&experiment(), "verified").unwrap().matched);
    }
    #[test]
    fn records_disagreement() {
        assert!(!evaluate(&experiment(), "unknown").unwrap().matched);
    }
    #[test]
    fn rejects_missing_hypothesis() {
        let mut e = experiment();
        e.hypothesis.clear();
        assert!(evaluate(&e, "verified").is_err());
    }
}
