fn binary_search(v1: &[i32], target: i32) -> usize {
    let mut lb: usize = 0;
    let mut rb: usize = v1.len() - 1;

    while lb <= rb {
        let mb: usize = lb + (rb - lb) / 2;

        if v1[mb] == target {
            return mb;
        } else if v1[mb] < target {
            lb = mb + 1;
        } else {
            rb = mb - 1;
        }
    }

    return 0;
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

    println!("{}", binary_search(&vec, 6))
}
