
pub struct Solution {}

impl Solution {

    pub fn two_sums( vec: &Vec<i32>, target: i32 ) -> Vec<i32> {

        /* for (i, &v1) in vec.iter().enumerate() {

            for (j, &v2) in vec.iter().skip(i + 1).enumerate() {
                let j = j + i + 1;
                if v1 + v2 == target { return vec![i as i32, j as i32] }
            }
        } */

        use std::collections::HashMap;

        let mut map: HashMap<i32, i32> = HashMap::new();

        for i in 0 .. vec.len() {

            let v = vec[i];
            match map.get( &(target - v) ) {
                Some( &n ) => { return vec![ n, i as i32 ] }
                None => { map.entry( v ).or_insert( i as i32 ); }
            }

        }

        vec![]
    }

    pub fn linear_search( vec: &Vec<i32>, target: i32 ) -> i32 {

        for i in 0 .. vec.len() {
            if vec[i] == target { return i as i32; }
        }

        -1
    }

    pub fn binary_search( vec: &Vec<i32>, target: i32 ) -> Option<usize> {

        let mut head = 0;
        let mut tail = vec.len() - 1;

        while head <= tail {
            let mid = (head + tail) / 2;

            if vec[mid] == target { return Some(mid); }
            else if vec[mid] < target { head = mid + 1; }
            else { tail = mid - 1; }
        }

        None
    }

    pub fn max_coins(piles: &mut Vec<i32>) -> i32 {

        piles.sort_unstable();
        let numbers = piles.len();

        piles.iter()
            .rev()
            .take( (numbers*2) / 3  )
            .skip(1)
            .step_by(2)
            .sum::<i32>()

    }


}
