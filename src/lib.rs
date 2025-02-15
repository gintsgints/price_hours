#[derive(serde::Deserialize, Clone)]
pub struct PeriodPrice {
    #[serde(alias = "priceStartTime")]
    price_start_time: String,
    #[serde(alias = "priceEndTime")]
    price_end_time: String,
    value: f64,
    #[serde(alias = "levelInDailyView")]
    level_in_daily_view: i8,
    #[serde(alias = "isActiveHourNow")]
    is_active_hour_now: bool,
}

pub struct PeriodPrices {
    pub prices: Vec<PeriodPrice>,
}

impl PeriodPrices {
    pub fn is_in_n_cheepest(&self, hour: String, n: usize) -> bool {
        let mut cheepest: Vec<PeriodPrice> = vec![];
        // add next to cheepest
        self.prices.iter().for_each(|pp| {
            cheepest.push(pp.clone());

            // if size bigger than n, remove most expensive
            if cheepest.len() > n {
                if let Some(expensive) = cheepest
                    .iter()
                    .max_by(|&a, &b| a.value.partial_cmp(&b.value).unwrap())
                {
                    if let Some(exp_index) = cheepest.iter().position(|pp| pp.value == expensive.value) {
                        cheepest.swap_remove(exp_index);
                    };
                };
            }
        });

        cheepest.iter().any(|pp| pp.price_start_time == hour)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_is_in_n_cheepest() {
        let prices: PeriodPrices = PeriodPrices {
            prices: vec![
                PeriodPrice {
                    price_start_time: "00:00:00".to_string(),
                    price_end_time: "01:00:00".to_string(),
                    value: 0.108,
                    level_in_daily_view: 1,
                    is_active_hour_now: false,
                },
                PeriodPrice {
                    price_start_time: "01:00:00".to_string(),
                    price_end_time: "02:00:00".to_string(),
                    value: 0.207,
                    level_in_daily_view: 2,
                    is_active_hour_now: false,
                },
                PeriodPrice {
                    price_start_time: "02:00:00".to_string(),
                    price_end_time: "03:00:00".to_string(),
                    value: 0.195,
                    level_in_daily_view: 2,
                    is_active_hour_now: false,
                  }
            ],
        };
        assert!(prices.is_in_n_cheepest("00:00:00".to_string(), 1));
        assert!(!prices.is_in_n_cheepest("01:00:00".to_string(), 1));
        assert!(!prices.is_in_n_cheepest("02:00:00".to_string(), 1));
        assert!(prices.is_in_n_cheepest("02:00:00".to_string(), 2));
        assert!(!prices.is_in_n_cheepest("01:00:00".to_string(), 2));
    }
}
