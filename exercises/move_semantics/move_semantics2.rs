fn main() {
    let mut vec0 = Vec::new();
    vec0.push(22);
    vec0.push(44);
    vec0.push(66);

    let mut vec1 = vec0.clone();
    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);
    vec1.push(88);
    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}
