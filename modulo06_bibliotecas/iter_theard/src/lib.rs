use std::thread;

pub fn calculate_sum<'a, T> (numbers: &'a [T], num_theards: usize) -> T 
where T: Default + Send + Sync + std::ops::Add<Output = T> + Copy + 'static
{ 
    let chunk_size = numbers.len() / num_theards;
    let mut threads = vec![];

    for chunk in numbers.chunks(chunk_size) {
        let c = chunk.to_vec().iter().map(|x| *x).collect::< Vec<T> >();
        let t = thread::spawn( move || {
            let sum = c.iter().fold(T::default(), |acc, &x| acc + x );
            sum
        }
        );

        threads.push(t);
    }
    
    let mut res = T::default();
    for t in threads {
        let v = t.join().unwrap_or_default();
        res = res + v;
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::calculate_sum;


    #[test]
    fn teste_sum() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let res = calculate_sum(&numbers, 4);

        assert_eq!(res, 55);
    }

}
