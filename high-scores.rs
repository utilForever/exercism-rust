#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        let ret = self.scores.last();
        match ret {
            Some(val) => Some(*val),
            None => None,
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        let ret = self.scores.iter().max();
        match ret {
            Some(val) => Some(*val),
            None => None,
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.clone();
        scores.sort();

        match scores.len() {
            0 => Vec::new(),
            1 => vec![scores[0]],
            2 => vec![scores[1], scores[0]],
            _ => {
                let len = scores.len();
                vec![scores[len - 1], scores[len - 2], scores[len - 3]]
            }
        }
    }
}
