use crate::dice;

pub struct Budget {
    pub dist: Vec<f64>,
    pub level: u8,
    pub range: Range,
}

impl Budget {
    pub fn create(lvl: usize, is_ancient: bool) -> Self {
        let rolls = if is_ancient { 6 } else { 5 };
        let dist = dice::get_cumulative_distribution(lvl / 2 + 20, rolls);
        let max = dist.len() - 1;

        Budget {
            dist,
            level: lvl as u8,
            range: Range {
                min: lvl as u16,
                max: max as u16,
            },
        }
    }
}

pub struct Range {
    pub min: u16,
    pub max: u16,
}
