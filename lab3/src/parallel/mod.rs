use memoffset::offset_of;

use rand::Rng;

use mpi::datatype::UserDatatype;
use mpi::topology::SimpleCommunicator;
use mpi::traits::*;
use mpi::Address;

use lab1::common::input_size;

#[derive(Debug, Clone)]
pub struct MatrixVectorData {
    pub p_matrix: Vec<f64>,
    pub p_vector: Vec<f64>,
    pub p_result: Vec<f64>,
    pub p_proc_rows: Vec<f64>,
    pub p_proc_vector: Vec<f64>,
    pub p_proc_result: Vec<f64>,
    pub size: usize,
    pub row_num: usize,
}

impl MatrixVectorData {
    pub fn new(communicator: &SimpleCommunicator, proc_rank: i32, proc_num: i32) -> Self {
        let mut size = 0;
        let mut p_matrix = Vec::new();
        let mut p_vector = Vec::new();
        let mut p_result = Vec::new();

        if proc_rank == 0 {
            size = input_size().unwrap() as usize;

            if size < proc_num as usize {
                panic!("Size must be greater than or equal to the number of processes");
            }

            // Dummy initialization
            p_matrix = vec![0.0; size * size];
            p_vector = vec![0.0; size];
            p_result = vec![0.0; size];
            // Initialize matrix and vector
        }

        let root_process = communicator.process_at_rank(0);
        root_process.broadcast_into(&mut size);

        // Calculate rows for this process
        let row_num = calculate_rows_for_process(proc_rank, proc_num, size);

        // Initialize process specific vectors
        let p_proc_rows = vec![0.0; row_num * size];
        let p_proc_vector = vec![0.0; row_num];
        let p_proc_result = vec![0.0; row_num];

        MatrixVectorData {
            p_matrix,
            p_vector,
            p_result,
            p_proc_rows,
            p_proc_vector,
            p_proc_result,
            size,
            row_num,
        }
    }
}

impl MatrixVectorData {
    pub fn dummy_data_initialization(&mut self) {
        for i in 0..self.size {
            self.p_vector[i] = (i + 1) as f64;
            for j in 0..self.size {
                if j <= i {
                    self.p_matrix[i * self.size + j] = 1.0;
                } else {
                    self.p_matrix[i * self.size + j] = 0.0;
                }
            }
        }
    }

    pub fn random_data_initialization(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.size {
            self.p_vector[i] = rng.gen::<f64>() / 1000.0;
            for j in 0..self.size {
                if j <= i {
                    self.p_matrix[i * self.size + j] = rng.gen::<f64>() / 1000.0;
                } else {
                    self.p_matrix[i * self.size + j] = 0.0;
                }
            }
        }
    }
}

#[derive(Default)]
pub struct MaxValueProcRank {
    pub max_value: f64,
    pub proc_rank: i32,
}

unsafe impl Equivalence for MaxValueProcRank {
    type Out = UserDatatype;
    fn equivalent_datatype() -> Self::Out {
        UserDatatype::structured(
            &[1, 1],
            &[
                offset_of!(MaxValueProcRank, max_value) as Address,
                offset_of!(MaxValueProcRank, proc_rank) as Address,
            ],
            &[f64::equivalent_datatype(), f64::equivalent_datatype()],
        )
    }
}

fn calculate_rows_for_process(proc_rank: i32, proc_num: i32, total_size: usize) -> usize {
    let base_rows = total_size / proc_num as usize;
    let extra_rows = total_size % proc_num as usize;
    base_rows
        + if (proc_rank as usize) < extra_rows {
            1
        } else {
            0
        }
}
