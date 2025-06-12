use std::cmp::Ordering;

const K: usize = 8;
const N: usize = 2 * K;

fn next(c: &mut [bool; N]) -> bool {
    for i in 0..N {
        c[i] = c[i] ^ true;
        if c[i] {
            return true;
        }
    }
    return false;
}

fn b2i(b: bool) -> isize {
    if b { 1 } else { -1 }
}
/*
fn cmp(a: &[bool; N], b: &[bool; N]) -> Ordering {
    for i in 0..N {
        if a[N - i] != b[N - i] {
            if a[N - i] {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        }
    }
    Ordering::Equal
}
     */

#[derive(PartialEq, Debug)]
enum Orthogonal {
    Yes,
    No(usize),
}

fn inner_prod_zero(a: &[bool; N], b: &[bool; N]) -> Orthogonal {
    let mut result = 0;
    for i in K..N {
        result += b2i(a[i]) * b2i(b[i]);
    }
    println!("{:?}", result);
    for i in 1..=K {
        result += b2i(a[K - i]) * b2i(b[K - i]);
        println!("r {:?}, k-i {:?}", result, K - i);
        if K - i < result.abs() as usize {
            return Orthogonal::No(K - i);
        }
    }
    //println!("{:?},{:?},{:?}", a, b, result);
    if result == 0 {
        Orthogonal::Yes
    } else {
        Orthogonal::No(0)
    }
}
/*
#[test]
fn inner_prod_zero_0() {
    assert_eq!(
        inner_prod_zero(
            &[
                false, true, false, true, false, true, false, true, false, true, false, true,
                false, true, false, true
            ],
            &[false; N]
        ),
        Orthogonal::Yes
    );
}
#[test]
fn inner_prod_zero_2() {
    assert_eq!(
        inner_prod_zero(
            &[
                false, true, false, true, false, true, false, true, false, true, false, true,
                false, true, false, false
            ],
            &[false; N]
        ),
        Orthogonal::No(0)
    );
}
#[test]
fn inner_prod_zero_1() {
    assert_eq!(inner_prod_zero(&[false; N], &[true; N]), Orthogonal::No(7));
}
*/
fn recurse(depth: usize, matrix: &mut [[bool; N]; N]) {
    if depth != 0 {
        for i in 0..N {
            matrix[depth][i] = matrix[depth - 1][i];
        }
    }
    'outer: while next(&mut matrix[depth]) {
        // Check orthogonal
        for d in 0..depth {
            match inner_prod_zero(&matrix[d], &matrix[depth]) {
                Orthogonal::No(_) => {
                    /*println!(
                        "fail depth {:?} d {:?} matrix {:?} inner {:?}",
                        depth,
                        d,
                        matrix,
                        inner_prod(&matrix[d], &matrix[depth])
                    );*/
                    continue 'outer;
                }
                Orthogonal::Yes => (),
            }
        }
        // Recurse or done
        if depth < N - 1 {
            recurse(depth + 1, matrix);
        } else {
            println!("{:?}", matrix);
        }
    }
}

fn main() {
    recurse(0, &mut [[false; N]; N]);
}
