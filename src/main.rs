use RNAligner::io::{RnaSequence, parse_fasta};
use RNAligner::compare::Score;
use RNAligner::benchmark::Benchmark;


fn main() {
    let seq_list: Vec<RnaSequence> = parse_fasta("data/trna_unmodified_dot_bracket.txt");
    let nussinov_score = Score::new(seq_list[0].clone(), "nussinov");
    let vienna_score = Score::new(seq_list[0].clone(), "vienna");

    println!();

    let bench = Benchmark::new(seq_list, 40); // +50 takes a lot time. < 50 works
    // let algos: Vec<&str> = vec!("nussinov", "viennaRNA");
    // Benchmark::new(seq_list, 40, algos); 
    
    bench.repr();
    // bench.csv();
    println!();

    println!("========== Example for id: tdbR00000365 =============");
    
    let _ = nussinov_score.expect("error nussinov").repr();
    let _ = vienna_score.expect("error vienna").repr(); 
    println!();
}
