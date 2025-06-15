use std::fmt;

const K: usize = 3;
const N: usize = 2 * K;

struct Row {
    r: [bool; N],
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[").unwrap();
        for i in 0..N {
            if self.r[i] {
                write!(f, " 1,").unwrap();
            } else {
                write!(f, "-1,").unwrap();
            }
        }
        write!(f, "]")
    }
}

fn next(c: &mut [bool; N]) {
    for i in 0..N {
        c[i] = c[i] ^ true;
        if c[i] {
            return;
        }
    }
}

fn all_ones(c: &[bool; N]) -> bool {
    for i in 0..N {
        if !c[i] {
            return false;
        }
    }
    true
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
    //println!("{:?}", result);
    for i in 1..=K {
        result += b2i(a[K - i]) * b2i(b[K - i]);
        //println!("r {:?}, k-i {:?}", result, K - i);
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
fn recurse(depth: usize, matrix: &mut [[bool; N]; N]) -> usize {
    let mut result = 0;
    if depth != 0 {
        for i in 0..N {
            matrix[depth][i] = matrix[depth - 1][i];
        }
        next(&mut matrix[depth]);
    }
    'outer: loop {
        for i in 0..N {
            println!("{}", Row { r: matrix[i] });
        }
        println!();

        // Check orthogonal
        for d in 0..depth {
            match inner_prod_zero(&matrix[d], &matrix[depth]) {
                Orthogonal::No(k) => {
                    if k > 1 {
                        if all_ones(&matrix[depth]) {
                            return result;
                        }
                        if matrix[depth][k] == false {
                            for j in 0..=k {
                                matrix[depth][j] ^= true;
                            }
                        } else {
                            let mut all_true = true;
                            for j in 0..k {
                                all_true &= matrix[depth][j];
                                matrix[depth][j] = true;
                            }
                            if all_true {
                                next(&mut matrix[depth]);
                            }
                        }
                        //println!("after  {}", Row { r: matrix[depth] });
                        //println!("");
                    } else {
                        next(&mut matrix[depth]);
                    }
                    continue 'outer;
                }
                Orthogonal::Yes => (),
            }
        }
        // Recurse or done
        if depth < N - 1 {
            result += recurse(depth + 1, matrix);
        } else {
            result += 1;
            //println!("{:?}", matrix);
        }

        if result >= 1000 {
            return result;
        }
        if all_ones(&matrix[depth]) {
            return result;
        }
    }
}

fn main() {
    println!("{}", recurse(0, &mut [[false; N]; N]));
}
