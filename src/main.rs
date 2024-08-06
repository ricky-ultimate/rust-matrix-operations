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
            data: vec![vec![0;rows];cols]
        }
    }
}

fn main() {
    println!("Hello, world!");
}
