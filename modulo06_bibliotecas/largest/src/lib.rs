pub fn largest<T>(list: &[T]) -> Option<T>
where
    T: PartialOrd + Copy,
{
    let mut largest = *list.get(0)?;

    for item in list {
        largest = if *item > largest { *item } else { largest }
    }

    Some(largest)
}
