impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut total = 0;
        for i in 1..time_series.len() {
            total += duration.min(time_series[i] - time_series[i - 1]);
        }
        total += duration;
        total
    }
}