
pub struct Solution {}

impl Solution {

    pub fn two_sums( vec: &Vec<i32>, target: i32 ) -> Vec<i32> {

        for (i, &v1) in vec.iter().enumerate() {

            for (j, &v2) in vec.iter().skip(i + 1).enumerate() {
                let j = j + i + 1;
                if v1 + v2 == target { return vec![i as i32, j as i32] }
            }
        }

        vec![]
    }

}
