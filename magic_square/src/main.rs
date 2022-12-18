use itertools::Itertools;

fn main() {
    let items = vec![1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0];
    let len = items.len();

    for perm in items.into_iter().permutations(len) {
        let sums = add_row_col_dig(&perm);
        let y = filter_same_vals(sums);
        if y.len() == 0 {
            println!("ANS: {:?}", perm);
        }
    }
}

fn filter_same_vals<T>(vals: [T; 8]) -> Vec<T>
where
    T: std::cmp::PartialEq + Copy,
{
    vals.into_iter()
        .filter(|x| *x != vals[0])
        .collect::<Vec<T>>()
}

fn add_row_col_dig(val: &Vec<f64>) -> [f64; 8] {
    [
        //Row
        val[0] + val[1] + val[2],
        val[3] + val[4] + val[5],
        val[6] + val[7] + val[8],
        //Col
        val[0] + val[3] + val[6],
        val[1] + val[4] + val[7],
        val[2] + val[5] + val[8],
        //Dig
        val[0] + val[4] + val[8],
        val[2] + val[4] + val[6],
    ]
}

#[test]
fn check_row() {
    let val: Vec<f64> = vec![1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0];
    assert_eq!(
        add_row_col_dig(&val),
        [4.5, 9.0, 13.5, 7.5, 9.0, 9.5, 9.0, 9.0]
    );
}

#[test]
fn check_tuple_same() {
    let x1 = [1, 2, 3, 3, 3, 3, 3, 3];
    let x2 = [3, 3, 3, 3, 3, 3, 3, 3];

    let y: _ = filter_same_vals(x1);
    assert_eq!(y.len() == 0, false);

    let y: _ = filter_same_vals(x2);
    assert_eq!(y.len() == 0, true);
}
