use crate::io::RnaSequence;
use crate::compare::Score;


pub struct Benchmark {
    vienna_avg: f64,
    min_vienna: (String, f64),
    max_vienna: (String, f64),
    std_vienna: f64,

    score_distribution_vienna: [i32; 5],

    top_scores_vienna:    Vec<(String, f64)>,
    bottom_scores_vienna: Vec<(String, f64)>,

    nussinov_avg: f64,
    min_nussinov: (String, f64),
    max_nussinov: (String, f64),
    std_nussinov: f64,

    score_distribution_nussinov: [i32; 5],

    top_scores_nussinov:    Vec<(String, f64)>,
    bottom_scores_nussinov: Vec<(String, f64)>, 

    quantity: i32,
}


impl Benchmark {

    fn std_dev(avg: f64, score_list: &Vec<(String, f64)>) -> f64 {
        let n = score_list.len();
        if n <= 1 {
            return 0.0;
        }
        let mut score_sum = 0.0;
        for i in 0..n {
            score_sum += (score_list[i].1 - avg).powf(2.0);
        }
        (score_sum / ((n - 1) as f64)).sqrt()
    }


    fn merge_sort(mut scores_list: Vec<(String, f64)>) -> Vec<(String, f64)> {
        let len = scores_list.len();
        if len <= 1 {
            return scores_list;
        }

        let mid = len / 2;
        let right = scores_list.split_off(mid);
        let left = scores_list; // remaining left half

        let sorted_left = Self::merge_sort(left);
        let sorted_right = Self::merge_sort(right);

        // merge two sorted vectors
        let mut merged: Vec<(String, f64)> = Vec::with_capacity(sorted_left.len() + sorted_right.len());
        let mut i = 0;
        let mut j = 0;

        while i < sorted_left.len() && j < sorted_right.len() {
            if sorted_left[i].1 <= sorted_right[j].1 {
                merged.push(sorted_left[i].clone());
                i += 1;
            } else {
                merged.push(sorted_right[j].clone());
                j += 1;
            }
        }

        // append remaining
        if i < sorted_left.len() {
            merged.extend_from_slice(&sorted_left[i..]);
        }
        if j < sorted_right.len() {
            merged.extend_from_slice(&sorted_right[j..]);
        }

        merged
    }

    fn get_distribution(scores_list: &Vec<(String, f64)>) -> [i32; 5] {
        let mut arr: [i32; 5] = [0; 5];
        for i in 0..scores_list.len() {
            if scores_list[i].1 >= 0.0 && scores_list[i].1 < 20.0 {
                arr[0] += 1;
            } else if scores_list[i].1 >= 20.0 && scores_list[i].1 < 40.0 {
                arr[1] += 1;
            } else if scores_list[i].1 >= 40.0 && scores_list[i].1 < 60.0 {
                arr[2] += 1;
            } else if scores_list[i].1 >= 60.0 && scores_list[i].1 < 80.0 {
                arr[3] += 1;
            } else if scores_list[i].1 >= 80.0 && scores_list[i].1 <= 100.0 {
                arr[4] += 1;
            }
        }
        arr
    }


    fn get_data(seq_list: Vec<RnaSequence>, algo: &str, max: i32) -> (f64, f64, (String, f64), (String, f64), Vec<(String, f64)>, Vec<(String, f64)>, [i32; 5]) {
        let mut score_list: Vec<(String, f64)> = Vec::new();
        let mut scores_sum: f64 = 0.0;
        let mut count: i32 = 0;

        for li in seq_list {
            let score = Score::new(li, algo).expect("failed to build Score");
            let score_num: f64 = score.get_score().expect("REASON");
            let score_id: String = score.get_id().expect("REASON").to_string();
            score_list.push((score_id, score_num));
        
            scores_sum += score_num;
            count +=1;
            if count >= max { break; }
        }
    
        let avg = if count > 0 { scores_sum / (count as f64) } else { 0.0 };
    
        let std = Self::std_dev(avg, &score_list);

        let sorted_scores = Self::merge_sort(score_list.clone());

        let top_five: Vec<(String, f64)> = sorted_scores.iter().rev().take(5).cloned().collect();
        let max_score: (String, f64) = if !top_five.is_empty() { top_five[0].clone() } else { (String::new(), 0.0) };
        let bottom_five: Vec<(String, f64)> = sorted_scores.iter().take(5).cloned().collect();
        let min_score: (String, f64) = if !bottom_five.is_empty() { bottom_five[0].clone() } else { (String::new(), 0.0) };

        let distribution = Self::get_distribution(&score_list);

        (avg, std, max_score, min_score, top_five, bottom_five, distribution)
    }

    pub fn new(seq_list: Vec<RnaSequence>, max: i32) -> Self {

        let ( nussinov_avg, 
              std_dev_nussinov, 
              max_n, 
              min_n, 
              top_five_n, 
              bottom_five_n, 
              distribution_n ) = Self::get_data(seq_list.clone(), "nussinov", max.clone());

        let ( vienna_avg, 
              std_dev_vienna, 
              max_v, 
              min_v, 
              top_five_v, 
              bottom_five_v, 
              distribution_v ) = Self::get_data(seq_list.clone(), "vienna", max.clone());

        Benchmark { vienna_avg: vienna_avg, 
                    nussinov_avg: nussinov_avg, 
                    quantity: max, 
                    max_nussinov: max_n, 
                    min_nussinov: min_n, 
                    std_nussinov: std_dev_nussinov,
                    top_scores_nussinov: top_five_n,
                    bottom_scores_nussinov: bottom_five_n,
                    score_distribution_nussinov: distribution_n,
                    max_vienna: max_v, 
                    min_vienna: min_v,
                    std_vienna: std_dev_vienna,
                    top_scores_vienna: top_five_v,
                    bottom_scores_vienna: bottom_five_v,
                    score_distribution_vienna: distribution_v }
    }

    fn max_strnum_len(distrib: [i32; 5]) -> i32 {
        let mut max_len: i32 = 0;
        for i in distrib {
            max_len = max_len.max(i.to_string().len().try_into().unwrap());
        }
        max_len
    }

    pub fn score_distribution(&self, algo: &str) -> Result<(), Box<dyn std::error::Error>> {
        let distribution;

        if algo == "vienna" {
            distribution = self.score_distribution_vienna;
        } else if algo  == "nussinov" {
            distribution = self.score_distribution_nussinov;
        } else {
            panic!("Unknown algorithm: {}", algo);
        }

        println!("--- Score Distribution ({}) ---", algo);
        let fraction = [
            "0-20%   ",
            "20-40%  ",
            "40-60%  ",
            "60-80%  ",
            "80-100% "
        ];
        let sp_char = "█";
        let max_strnum = Self::max_strnum_len(distribution);
        for i in 0..distribution.len() {
            print!("{}| ", fraction[i]);
            let space_loops = max_strnum - distribution[i].to_string().len() as i32 + 1;
            for _j in 0..space_loops {
                print!(" ")
            }
            print!("{} | ", distribution[i]);
            for _j in 0..distribution[i] {
                print!("{}", sp_char);
            }
            println!();
        }
        println!();
        Ok(())
    }

    pub fn min_max(&self, algo: &str) {
        if algo == "nussinov" || algo == "all" {
            println!("Nussinov:  min={:.0}% | max={:.0}% | std={:.0}%", self.min_nussinov.1, self.max_nussinov.1, self.std_nussinov);
        }
        if algo == "vienna" || algo == "all" {
            println!("ViennaRNA: min={:.0}% | max={:.0}% | std={:.0}%", self.min_vienna.1, self.max_vienna.1, self.std_vienna);
        }
    }

    pub fn top_five(&self, algo: &str) {
        if algo == "nussinov" || algo == "all" {
            println!("----- Top five Nussinov -----");
            for i in 1..=5 {
                let top = &self.top_scores_nussinov[i-1];
                println!("{}. {:.1}% {}", i, top.1, top.0);
            }
        }
        if algo == "vienna" || algo == "all" {
            println!("----- Top five ViennaRNA -----");
            for i in 1..=5 {
                let top = &self.top_scores_vienna[i-1];
                println!("{}. {:.1}% {}", i, top.1, top.0);
            }
        }
    }
    pub fn bottom_five(&self, algo: &str) {
        if algo == "nussinov" || algo == "all" {
            println!("----- Worst five Nussinov -----");
            for i in 1..=5 {
                let bottom = &self.bottom_scores_nussinov[i-1];
                println!("{}. {:.1}% {}", i, bottom.1, bottom.0);
            }
        }
        if algo == "vienna" || algo == "all" {
            println!("----- Worst five ViennaRNA -----");
            for i in 1..=5 {
                let bottom = &self.bottom_scores_vienna[i-1];
                println!("{}. {:.1}% {}", i, bottom.1, bottom.0);
            }
        }
    }

    pub fn repr(&self) {
        println!("=========== Benchmark matching scores for VIENNARNA and NUSSINOV algo ==========");
        println!("Average Match Score for nussinov:  {:.2}%", self.nussinov_avg);
        println!("Average Match Score for ViennaRNA: {:.2}%", self.vienna_avg);
        println!("Total samples number for each: {}", self.quantity);
        println!();
        println!("----------- Min / Max Scores -----------");
        self.min_max("all");
        println!();

        let _ = self.score_distribution("vienna");
        let _ = self.score_distribution("nussinov");

        println!("Quantity: {}", self.quantity);

        self.top_five("nussinov");
        println!();
        self.bottom_five("nussinov");
        println!();
        self.top_five("vienna");
        println!();
        self.bottom_five("vienna");
    }
}

