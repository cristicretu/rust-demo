use std::cmp::Ordering;

fn binary_search<T: Ord>(v1: &[T], target: &T) -> Option<usize> {
    let mut lb: usize = 0;
    let mut rb: usize = v1.len() - 1;

    while lb <= rb {
        let mb: usize = lb + (rb - lb) / 2;

        match target.cmp(&v1[mb]) {
            Ordering::Less => rb = mb,
            Ordering::Equal => return Some(mb),
            Ordering::Greater => lb = mb + 1,
        }
    }

    None
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let index = binary_search(&vec, &6);

    println!("{:?}", index); // use debugging mode for 'Option<T>
                             // 'Some(x)' isn't suitable for end-user consumption

    let vec2 = vec!["a", "b", "c", "d", "e"];

    let index2 = binary_search(&vec2, &"c");

    println!("{:?}", index2);
}
