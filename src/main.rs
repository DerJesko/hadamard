const N: usize = 4;

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

fn inner_prod(a: &[bool; N], b: &[bool; N]) -> isize {
    let mut result = 0;
    for i in 0..N {
        result += b2i(a[i]) * b2i(b[i])
    }
    //println!("{:?},{:?},{:?}", a, b, result);
    result
}

fn recurse(depth: usize, matrix: &mut [[bool; N]; N]) {
    if depth != 0 {
        for i in 0..N {
            matrix[depth][i] = matrix[depth - 1][i];
        }
    }
    'outer: while !next(&mut matrix[depth]) {
        // Check orthogonal
        for d in 0..depth {
            if inner_prod(&matrix[d], &matrix[depth]) != 0 {
                /*println!(
                    "fail depth {:?} d {:?} matrix {:?} inner {:?}",
                    depth,
                    d,
                    matrix,
                    inner_prod(&matrix[d], &matrix[depth])
                );*/
                continue 'outer;
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
