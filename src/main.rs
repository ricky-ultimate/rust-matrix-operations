struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<i32>>,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0; cols]; rows],
        }
    }

    fn display(&self) {
        for row in &self.data {
            for val in row {
                print!("{} ", val);
            }
            println!();
        }
    }

    fn add(&self, other: &Matrix) -> Option<Matrix> {
        if self.rows != other.rows && self.cols != other.cols {
            return None; //Incompatible dimensions
        }

        let mut result = Matrix::new(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 00..self.cols {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }

        Some(result)
    }

    fn subtract(&self, other: &Matrix) -> Option<Matrix> {
        if self.rows != other.rows && self.cols != other.cols {
            return None; //Incompatible dimensions
        }

        let mut result = Matrix::new(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 00..self.cols {
                result.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }

        Some(result)
    }

    fn multiply(&self, other: &Matrix) -> Option<Matrix> {
        if self.cols != other.rows {
            return None; // Incompatible dimensions
        }
        let mut result = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        Some(result)
    }
}

fn main() {
    let mut matrix1 = Matrix::new(2, 3);
    matrix1.data = vec![vec![1, 2, 3], vec![4, 5, 6]];

    let mut matrix2 = Matrix::new(3, 2);
    matrix2.data = vec![vec![7, 8], vec![9, 10], vec![11, 12]];

    println!("Matrix 1:");
    matrix1.display();
    println!("\nMatrix 2:");
    matrix2.display();

    // if let Some(result) = matrix1.multiply(&matrix2) {
    //     println!("\nResult of multiplication:");
    //     result.display();
    // } else {
    //     println!("Matrices cannot be multiplied due to incompatible dimensions.");
    // }
    let result = matrix1.multiply(&matrix2);
    match result {
        Some(result) => {
            println!("\nResult of multiplication:");
            result.display();
        },
        None => println!("Matrices cannot be multiplied"),
    }
}
