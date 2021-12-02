

mod mine_ans;

fn main() {
    let mut a: Vec<(usize, i32)> = s

}

pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut a: Vec<(usize, i32)> = score.into_iter().enumerate().collect();
    let mut ans: Vec<String> = vec!["".to_string(); score.len()];
    a.sort_unstable_by_key(|x| std::cmp::Reverse(x.1));
    a.into_iter().enumerate().for_each(|(i, (idx, _))| {
        ans[idx] = match i + 1 {
            1 => "Gold Medal".to_string(),
            2 => "Silver Medal".to_string(),
            3 => "Bronze Medal".to_string(),
            _ => (i + 1).to_string(),
        };
    });
    ans
}
