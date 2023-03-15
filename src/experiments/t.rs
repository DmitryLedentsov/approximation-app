use std::error::Error;

#[derive(Debug)]
enum GaussError {
    SingularMatrix,
    NoSolution,
    UnsupportedMatrixSize,
    DivisionByZero,
}

fn gauss(matrix: &mut Vec<Vec<f64>>) -> Result<Vec<f64>, GaussError> {
    let size = matrix.len();

    if size == 0 {
        return Err(GaussError::UnsupportedMatrixSize);
    }

    // Проходимся по всем строкам
    for row in 0..size {
        // Выбираем главный элемент по столбцу
        let mut max_el = (row, row);
        for col in row..size {
            if matrix[col][row].abs() > matrix[max_el.0][max_el.1].abs() {
                max_el = (col, row);
            }
        }

        // Проверяем, что главный элемент не равен нулю
        if matrix[max_el.0][max_el.1] == 0.0 {
            return Err(GaussError::SingularMatrix);
        }

        // Меняем местами главную строку с текущей
        if max_el.0 != row {
            matrix.swap(max_el.0, row);
        }

        // Делим строку на главный элемент
        let divider = matrix[row][row];
        if divider == 0.0 {
            return Err(GaussError::DivisionByZero);
        }
        for col in row..size {
            matrix[row][col] /= divider;
        }

        // Вычитаем из остальных строк текущую
        for other_row in 0..size {
            if other_row == row {
                continue;
            }

            let factor = matrix[other_row][row];
            for col in row..size {
                matrix[other_row][col] -= factor * matrix[row][col];
            }
        }
    }

    // Проверяем, что система имеет решение
    for row in 0..size {
        let mut sum = 0.0;
        for col in 0..size {
            sum += matrix[row][col];
        }
        if sum != matrix[row][size] {
            return Err(GaussError::NoSolution);
        }
    }

    // Возвращаем решение
    let mut solution = Vec::new();
    for row in 0..size {
        solution.push(matrix[row][size]);
    }
    Ok(solution)
}

fn main() {
    let mut matrix = vec![
        vec![1.0, 2.0, 4.0, 4.0],
        vec![2.0, 3.0, 6.0, 8.0],
        vec![3.0, 5.0, 9.0, 12.0],
    ];
    let solution = gauss(&mut matrix);
    println!("{:?}", solution);
}