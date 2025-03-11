mod myers_diff;
use myers_diff::lcs_greedy;

fn main() {
    let a = vec!['A', 'B', 'C', 'A', 'B', 'B', 'A'];
    // let b = vec!['A', 'B', 'C', 'A', 'B', 'B', 'A'];
    let b = vec!['C', 'B', 'A', 'B', 'A', 'C'];
    let result = lcs_greedy(&a, &b, None);

    println!("Result: {:?}", result); // Should print the edit distance
}
