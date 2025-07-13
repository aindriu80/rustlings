fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // let v: Vec<i32> = a.to_vec();
    let v = a.to_vec(); // Corrected line

    // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro.
    // let v = ???;

    (a, v)
}

fn main() {
    // You can optionally experiment here.
    let (my_array, my_vec) = array_and_vec();
    println!("Array: {:?}", my_array);
    println!("Vector: {:?}", my_vec);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
