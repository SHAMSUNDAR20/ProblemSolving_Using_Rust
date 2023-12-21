impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut time: Vec<_> = dist.into_iter().zip(speed.into_iter()).map(|(d, s)| (d + s - 1) / s).collect();
        time.sort_unstable();
        for i in 0..time.len() {
            if i as i32 >= time[i] {
                return i as i32;
            }
        }
        time.len() as i32
    }
}