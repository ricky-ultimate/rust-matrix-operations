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

    fn deter(&self) -> Option<i32> {
        if self.rows != self.cols && self.rows < 2 {
            return None;
        }

        let result = self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0];

        Some(result)
    }


    fn s_multiply(&self, num: i32) -> Option<Matrix> {
        if false {
            return None;
        }

        let mut result = Matrix::new(self.rows, self.cols);

        for i in 0..self.rows{
            for j in 0..self.cols{
                result.data[i][j] = self.data[i][j] * num;
            }
        }

        Some(result)
    }
}

fn main(){

    let mut matrix1 = Matrix::new(2, 2);
    matrix1.data = vec![vec![1,2], vec![3,5]];

    matrix1.display();

    if let Some(result) = matrix1.s_multiply(3){
        result.display();
    }
    else {
        println!("No result");
    }
}
