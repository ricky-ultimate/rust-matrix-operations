struct Matrix{
    rows: usize,
    cols: usize,
    data: Vec<Vec<i32>>
}

impl Matrix {
    fn new(rows:usize, cols:usize) -> Matrix{
        Matrix{
            rows,
            cols,
            data: vec![vec![0;cols];rows]
        }
    }

    fn display(&self){
        for row in &self.data {
            for val in row {
                print!("{} ", val);
            }
            println!();
        }
    }

    fn add(&self, other: &Matrix) -> Option<Matrix>{
       if self.rows != other.rows && self.cols != other.cols{
        return None;
       }

        let mut result = Matrix::new(self.rows, other.cols);

        for i in 0..self.rows{
            for j in 00..self.cols{
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }

        Some(result)
    }

}

fn main() {
    let mut matrix1 = Matrix::new(2, 3);
    matrix1.data = vec![vec![1, 2, 3], vec![4, 5, 6]];

    let mut matrix2 = Matrix::new(2, 3);
    matrix2.data = vec![vec![1, 2, 3], vec![4, 5, 7]];

    println!("Matrix 1:");
    matrix1.display();
    println!("\nMatrix 2:");
    matrix2.display();

    if let Some(result) = matrix1.add(&matrix2) {
        println!("\nResult of addition:");
        result.display();
    } else {
        println!("Matrices cannot be added");
    }
}
