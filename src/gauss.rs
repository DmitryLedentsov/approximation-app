#[derive(Debug)]
pub enum GaussError {
    NoSolution,
    SingularMatrix,
    NonLinearSystem,
    OtherError,
}

pub fn solve(a: &mut Vec<Vec<f64>>, b: &mut Vec<f64>) -> Result<Vec<f64>, GaussError> {
    let n = a.len();
    let mut x = vec![0.0; n];

    // проверяем размерность
    if n != b.len() {
        return Err(GaussError::OtherError);
    }

    // проверяем наличие решения
    let det = a.iter().enumerate().fold(1.0, |acc, (i, row)| {
        acc * a[i][i]
    });
    if det == 0.0 {
        return Err(GaussError::SingularMatrix);
    }

    /*// проверяем линейность системы
    for row in a.iter() {
        for i in 0..n {
            for j in 0..n {
                if i != j && row[i] != 0.0 {
                    return Err(GaussError::NonLinearSystem);
                }
            }
        }
    }*/

    // прямой ход
    for k in 0..n {
        for i in k + 1..n {
            let m = a[i][k] / a[k][k];
            for j in k..n {
                a[i][j] -= m * a[k][j];
            }
            b[i] -= m * b[k];
        }
    }

    // обратный ход
    for k in (0..n).rev() {
        let mut s = 0.0;
        for j in (k + 1)..n {
            s += a[k][j] * x[j];
        }
        x[k] = (b[k] - s) / a[k][k];
    }

    Ok(x)
}

