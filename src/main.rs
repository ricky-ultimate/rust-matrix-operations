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
    println!("Hello, world!");
}
