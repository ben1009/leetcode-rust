/// [243] Shortest Word Distance
///
/// Given a list of words and two words word1 and word2, return the shortest distance between these
/// two words in the list.
///
/// Input：["practice", "makes", "perfect", "coding", "makes"], "coding","practice"
/// Output：3
/// Explanation：index("coding") - index("practice") = 3
///
/// Input：["practice", "makes", "perfect", "coding", "makes"], "makes","coding"
/// Output：1
/// Explanation：index("makes") - index("coding") = 1
///
/// You may assume that word1 does not equal to word2, and word1 and word2 are both in the list.
pub struct Solution {}

impl Solution {
    pub fn shortest_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
        if words.len() == 2 {
            return 1;
        }

        let mut ret = words.len() as i32;
        let mut p1 = -1;
        let mut p2 = -1;
        for (i, w) in words.iter().enumerate() {
            if w == &word1 {
                p1 = i as i32;
            }
            if w == &word2 {
                p2 = i as i32;
            }
            if p1 != -1 && p2 != -1 {
                ret = ret.min((p1 - p2).abs());
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_243() {
        assert_eq!(
            Solution::shortest_distance(
                vec_string!["practice", "makes", "perfect", "coding", "makes"],
                "coding".to_string(),
                "practice".to_string()
            ),
            3
        );
        assert_eq!(
            Solution::shortest_distance(
                vec_string![
                    "practice", "makes", "coding", "makes", "perfect", "coding", "practice"
                ],
                "coding".to_string(),
                "practice".to_string()
            ),
            1
        );
    }
}
