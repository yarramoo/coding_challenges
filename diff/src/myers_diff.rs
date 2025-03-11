use std::collections::HashMap;

fn find_furthest_x(v: &HashMap<isize, usize>, k: isize, d: isize) -> usize {
    if k == -d || (k != d && v.get(&(k - 1)).unwrap_or(&0) < v.get(&(k + 1)).unwrap_or(&0)) {
        *v.get(&(k + 1)).unwrap_or(&0)
    } else {
        v.get(&(k - 1)).unwrap_or(&0) + 1
    }
}

pub fn shortest_edit_script<T>(a: &[T], b: &[T], max: Option<usize>) -> Option<(usize, Vec<HashMap<isize, usize>>)>
where
    T: Eq,
{
    let n = a.len();
    let m = b.len();
    let max = max.unwrap_or(n + m);

    let mut furthest_reach: HashMap<isize, usize> = HashMap::new();
    furthest_reach.insert(1, 0);

    let mut trace = Vec::new();

    for distance in 0..=max as isize {
        for diagonal in (-distance..=distance).step_by(2) {
            let current_x = find_furthest_x(&furthest_reach, diagonal, distance);

            let mut x = current_x;
            let mut y = (x as isize - diagonal) as usize;

            while x < n && y < m && a[x] == b[y] {
                x += 1;
                y += 1;
            }

            furthest_reach.insert(diagonal, x);

            if x >= n && y >= m {
                return Some((distance as usize, trace));
            }
        }
        trace.push(furthest_reach.clone());
    }

    None
}

enum EditCommand<T> {
    Insert(usize, Vec<T>),
    Delete(usize),
}

fn reconstruct_shortest_edit<T>(k: isize, trace: &[HashMap<isize, usize>]) -> Vec<EditCommand<T>> {

    todo!();
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_same() {
        let a = "abcd";
        let b = "abcd";
        println!("{:?}", lcs_greedy(a.as_bytes(), b.as_bytes(), None));
    }
}