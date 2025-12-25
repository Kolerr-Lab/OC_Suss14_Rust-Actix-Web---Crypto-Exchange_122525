use std::collections::HashSet;

fn remove_duplicates(arr: &[i32]) -> Vec<i32> {
    let mut seen = HashSet::new();
    arr.iter().filter(|&&x| seen.insert(x)).cloned().collect()
}