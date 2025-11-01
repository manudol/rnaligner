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
    algo: String,
    exp_fold: String,
    fold: String,
    pub score: f32,
}



impl Score {
    fn get_accuracy(exp_fold: &str, fold: &str) -> f32 {
        assert_eq!(exp_fold.len(), fold.len());
        let mut matches = 0;
        for i in 0..exp_fold.len() {
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
        let fold = matrix.predict_fold();
        let accuracy = Self::get_accuracy(&seq.exp_fold, &fold);

        Ok(Score { algo: algo, exp_fold: seq.exp_fold.clone(), fold: fold, score: accuracy })
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

        Ok(Score { algo: algo, exp_fold: seq.exp_fold.clone(), fold: fold, score: accuracy})

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







// pub struct Benchmark {
//     vienna_avg: f64,
//     min_vienna: f64,
//     max_vienna: f64,
//     std_vienna: f64,
// 
//     score_distribution_vienna: [i32; 5],
// 
//     top_scores_vienna: Vec<f64>,
//     bottom_scores_vienna: Vec<f64>,
// 
//     nussinov_avg: f64,
//     min_nussinov: f64,
//     max_nussinov: f64,
//     std_nussinov: f64,
// 
//     score_distribution_nussinov: [i32; 5],
// 
//     top_scores_nussinov: Vec<f64>,
//     bottom_scores_nussinov: Vec<f64>, 
// 
//     quantity: i32,
// }
// 
// 
// impl Benchmark {
// 
//     fn std_dev(avg: f64, score_list: &Vec<f64>) -> f64 {
//         let n_samples = score_list.len();
//         let mut score_sum = 0.0;
//         for i in 0..n_samples {
//             score_sum += (score_list[i] - avg).powf(2.0);
//         }
//         score_sum / (n_samples - 1) as f64
//     }
// 
// 
//     fn merge_sort(mut scores_list: Vec<f64>) -> Vec<f64> {
//         let len = scores_list.len();
//         if len <= 1 {
//             return scores_list;
//         }
// 
//         let mid = len / 2;
//         let right = scores_list.split_off(mid);
//         let left = scores_list; // remaining left half
// 
//         let sorted_left = Self::merge_sort(left);
//         let sorted_right = Self::merge_sort(right);
// 
//         // merge two sorted vectors
//         let mut merged: Vec<f64> = Vec::with_capacity(sorted_left.len() + sorted_right.len());
//         let mut i = 0;
//         let mut j = 0;
// 
//         while i < sorted_left.len() && j < sorted_right.len() {
//             if sorted_left[i] <= sorted_right[j] {
//                 merged.push(sorted_left[i]);
//                 i += 1;
//             } else {
//                 merged.push(sorted_right[j]);
//                 j += 1;
//             }
//         }
// 
//         // append remaining
//         if i < sorted_left.len() {
//             merged.extend_from_slice(&sorted_left[i..]);
//         }
//         if j < sorted_right.len() {
//             merged.extend_from_slice(&sorted_right[j..]);
//         }
// 
//         merged
//     }
// 
//     fn get_distribution(scores_list: &Vec<f64>) -> [i32; 5] {
//         let mut arr: [i32; 5] = [0; 5];
//         for i in 0..scores_list.len() {
//             if scores_list[i] >= 0.0 && scores_list[i] < 20.0 {
//                 arr[0] += 1;
//             } else if scores_list[i] >= 20.0 && scores_list[i] < 40.0 {
//                 arr[1] += 1;
//             } else if scores_list[i] >= 40.0 && scores_list[i] < 60.0 {
//                 arr[2] += 1;
//             } else if scores_list[i] >= 60.0 && scores_list[i] < 80.0 {
//                 arr[3] += 1;
//             } else if scores_list[i] >= 80.0 && scores_list[i] < 100.0 {
//                 arr[4] += 1;
//             }
//         }
//         arr
//     }
// 
// 
//     fn get_avg_score(seq_list: Vec<RnaSequence>, algo: &str, max: i32) -> (f64, f64, f64, f64, Vec<f64>, Vec<f64>, [i32; 5]) {
//         let mut scores_sum: f64 = 0.0;
//         let mut count = 0;
// 
//         let mut max_score: f64= 0.0;
//         let mut min_score: f64 = 100.0;
// 
//         let mut score_list: Vec<f64> = Vec::new();
// 
//         for li in seq_list {
//             if count >= max {
//                 break;
//             }
//             let score = Score::new(li, algo);
//             let percent_score = score.unwrap().score as f64;
//             score_list.push(percent_score);
//             
//             max_score = max_score.max(percent_score);
//             min_score = min_score.min(percent_score);
//             
//             scores_sum += percent_score;
//             count += 1;
//         }
//         
//         let avg = scores_sum / (max as f64);
//         
//         let std = Self::std_dev(avg, &score_list);
// 
//         let sorted_scores = Self::merge_sort(score_list.clone());
// 
//         let top_five: Vec<f64>= sorted_scores.iter().rev().take(5).cloned().collect();
//         let max_score: f64 = top_five[0];
//         let bottom_five: Vec<f64> = sorted_scores.iter().take(5).cloned().collect();
//         let min_score: f64 = bottom_five[0];
// 
//         let distribution = Self::get_distribution(&score_list);
// 
//         (avg, std, max_score, min_score, top_five, bottom_five, distribution)
//     }
//     
//     pub fn new(seq_list: Vec<RnaSequence>, max: i32) -> Self {
// 
//         let (nussinov_avg, 
//              std_dev_nussinov, 
//              max_n, 
//              min_n, 
//              top_five_n, 
//              bottom_five_n, 
//              distribution_n) = Self::get_avg_score(seq_list.clone(), "nussinov", max.clone());
// 
//         let (vienna_avg, 
//              std_dev_vienna, 
//              max_v, 
//              min_v, 
//              top_five_v, 
//              bottom_five_v, 
//              distribution_v) = Self::get_avg_score(seq_list.clone(), "vienna", max.clone());
// 
//         Benchmark { vienna_avg: vienna_avg, 
//                     nussinov_avg: nussinov_avg, 
//                     quantity: max, 
//                     max_nussinov: max_n, 
//                     min_nussinov: min_n, 
//                     std_nussinov: std_dev_nussinov,
//                     top_scores_nussinov: top_five_n,
//                     bottom_scores_nussinov: bottom_five_n,
//                     score_distribution_nussinov: distribution_n,
//                     max_vienna: max_v, 
//                     min_vienna: min_v,
//                     std_vienna: std_dev_vienna,
//                     top_scores_vienna: top_five_v,
//                     bottom_scores_vienna: bottom_five_v,
//                     score_distribution_vienna: distribution_v }
//     }
// 
//     fn max_strnum_len(distrib: [i32; 5]) -> i32 {
//         let mut max_len: i32 = 0;
//         for i in distrib {
//             max_len = max_len.max(i.to_string().len().try_into().unwrap());
//         }
//         max_len
//     }
// 
//     pub fn repr(&self) {
//         println!("=========== Benchmark matching scores for VIENNARNA and NUSSINOV algo ==========");
//         println!("Average Match Score for nussinov:  {:.2}%", self.nussinov_avg);
//         println!("Average Match Score for ViennaRNA: {:.2}%", self.vienna_avg);
//         println!("Total samples number for each: {}", self.quantity);
//         println!();
//         println!("----------- Min / Max Scores -----------");
//         println!("Nussinov:  min={:.0}% | max={:.0}% | std={:.0}%", self.min_nussinov, self.max_nussinov, self.std_nussinov);
//         println!("ViennaRNA: min={:.0}% | max={:.0}% | std={:.0}%", self.min_vienna, self.max_vienna, self.std_vienna);
//         println!();
//         println!("--- Score Distribution (ViennaRNA) ---");
//         let fraction = [
//             "0-20%   ",
//             "20-40%  ",
//             "40-60%  ",
//             "60-80%  ",
//             "80-100% "
//         ];
//         let sp_char = "█";
//         let max_strnum = Self::max_strnum_len(self.score_distribution_vienna);
//         for i in 0..self.score_distribution_vienna.len() {
//             print!("{}| ", fraction[i]);
//             let space_loops = max_strnum - self.score_distribution_vienna[i].to_string().len() as i32 + 1;
//             for _j in 0..space_loops {
//                 print!(" ")
//             }
//             print!("{} | ", self.score_distribution_vienna[i]);
//             for _j in 0..self.score_distribution_vienna[i] {
//                 print!("{}", sp_char);
//             }
//             println!();
//         }
//         println!();
// 
//         println!("----- Top five Nussinov -----");
//         for i in 1..=5 {
//            println!("{}. {:.1}%", i, self.top_scores_nussinov[i-1]);
//         }
//         println!();
//         println!("----- Worst five Nussinov -----");
//         for i in 1..=5 {
//            println!("{}. {:.1}%", i, self.bottom_scores_nussinov[i-1]);
//         }
//         println!();
//         println!("----- Top five ViennaRNA -----");
//         for i in 1..=5 {
//            println!("{}. {:.1}%", i, self.top_scores_vienna[i-1]);
//         }
//         println!();
//         println!("----- Worst five ViennaRNA -----");
//         for i in 1..=5 {
//            println!("{}. {:.1}%", i,self.bottom_scores_vienna[i-1]);
//         }
//     }
// }
// 
