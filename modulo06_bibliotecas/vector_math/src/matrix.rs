// Structs
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

// Impls...
impl Matrix {
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();

        Self { rows, cols, data }
    }

    pub fn add(&self, other: &Matrix) -> Result<Matrix, &'static str> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err("As dimensoes das matrizes nao sao compativeis");
        }

        let mut result_data = Vec::with_capacity(self.rows);

        for i in 0..self.rows {
            let mut row = Vec::with_capacity(self.cols);

            for j in 0..self.cols {
                row.push(self.data[i][j] + other.data[i][j]);
            }

            result_data.push(row);
        }

        Ok(Matrix::new(result_data))
    }

    pub fn multiply(&self, other: &Matrix) -> Result<Matrix, &'static str> {
        if self.cols != other.rows {
            return Err("As dimensoes das matrizes nao sao compativeis");
        }

        let mut result_data = Vec::with_capacity(self.rows);

        for i in 0..self.rows {
            let mut rows = Vec::with_capacity(self.cols);

            for j in 0..other.cols {
                let mut sum = 0.0;

                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }
                rows.push(sum);
            }

            result_data.push(rows);
        }

        Ok(Matrix::new(result_data))
    }
}
