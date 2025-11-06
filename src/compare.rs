use crate::io::RnaSequence;
use crate::nussinov::Matrix;

use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ffi::{CString, CStr};

unsafe extern "C" {
    fn vienna_fold(seq: *const c_char) -> *const c_char;
    fn free_vienna_fold_result(ptr: *mut c_void);
}

#[derive(Debug, Clone)]
pub struct Score {
    id: String,
    seq: String,
    algo: String,
    exp_fold: String,
    fold: String,
    score: f32,
}



impl Score {
    fn get_accuracy(exp_fold: &str, fold: &str) -> f32 {
        // assert_eq!(exp_fold.len(), fold.len());
        let mut matches = 0;
        for i in 0..fold.len().min(exp_fold.len()) {
            if exp_fold.as_bytes()[i] == fold.as_bytes()[i] { matches += 1;
            }
        }
        let accuracy = ((matches as f32) / (fold.len() as f32)) * 100.0;
        accuracy
    }

    fn get_nussinov(seq: &RnaSequence, algo: String) -> Result<Self, Box<dyn std::error::Error>> {
        let matrix = Matrix::new(seq.to_string());
        //matrix.print_mat();
        let _pairs = matrix.traceback();
        //println!("here2");
        let fold = matrix.predict_fold();
        //println!("here3");
        let accuracy = Self::get_accuracy(&seq.exp_fold, &fold);
        //println!("here4");

        Ok(Score { id: seq.get_id(), seq: seq.to_string(), algo: algo, exp_fold: seq.exp_fold.clone(), fold: fold, score: accuracy })
    }


    fn get_vienna(seq: &RnaSequence, algo: String) -> Result<Self, Box<dyn std::error::Error>> {
        let fold = unsafe {
            let c_string = CString::new(seq.to_string()).expect("CString::new failed");
            let result_ptr = vienna_fold(c_string.as_ptr());
            if result_ptr.is_null() {
                panic!("pointer is null");
            }
            let result_str = CStr::from_ptr(result_ptr).to_str().unwrap().to_string();
            free_vienna_fold_result(result_ptr as *mut c_void);
            result_str
        };
        let accuracy = Self::get_accuracy(&seq.exp_fold, &fold);

        Ok(Score { id: seq.get_id(), seq: seq.to_string(), algo: algo, exp_fold: seq.exp_fold.clone(), fold: fold, score: accuracy })
    }


    pub fn new(seq: RnaSequence, algo: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if algo == "nussinov" {
            return Self::get_nussinov(&seq, algo.to_string());
        } else if algo == "vienna" {
            return Self::get_vienna(&seq, algo.to_string());
        } else {
            panic!("wrong algorithm!\nOnly 'nussinov' and 'vienna' accepted\nYou entered: {}", algo);
        }
    }
    
    pub fn get_id(&self) -> Result<&str, Box<dyn std::error::Error>> {
        Ok(&self.id)
    }

    pub fn get_seq(&self) -> Result<&str, Box<dyn std::error::Error>> {
        Ok(&self.seq)
    }

    pub fn get_score(&self) -> Result<f64, Box<dyn std::error::Error>> {
        Ok(self.score as f64)
    }

    pub fn repr(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.exp_fold.len() > 0 && self.fold.len() > 0 && self.score >= 0.0 && self.algo.len() > 0 {
            let mut matches = String::new();
            let exp_fold_bytes = self.exp_fold.as_bytes();
            let algo_fold_bytes = self.fold.as_bytes();

            for i in 0..algo_fold_bytes.len() {
                if exp_fold_bytes[i] == algo_fold_bytes[i] {
                    matches.push('|');
                } else {
                    matches.push(' ');
                }
            }

            println!("Algorithm used: {}", self.algo);
            println!("Experimental result: {}", self.exp_fold);
            println!("Algorithmic result:  {}", self.fold);
            println!("Matches:             {}", matches);
            println!("Match score:         {:.2}%", self.score);
            
            Ok(())
        } else {
            panic!("You cannot call this method before having initialized Score with: Score::new()");
        }
    }
}

