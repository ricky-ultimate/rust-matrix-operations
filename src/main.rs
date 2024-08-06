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
}

fn main() {
    let mut matrix1 = Matrix::new(2, 3);
    //matrix1.data = vec![vec![1, 2, 3], vec![4, 5, 6]];
    matrix1.display();
}
