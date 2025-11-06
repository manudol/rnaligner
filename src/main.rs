use rnaligner::io::{RnaSequence, parse_fasta};
use rnaligner::compare::Score;
use rnaligner::benchmark::Benchmark;


fn main() {
    let seq_list: Vec<RnaSequence> = parse_fasta("data/trna_unmodified_dot_bracket.txt");

    println!();

    let bench = Benchmark::new(seq_list, 50); // +50 takes a lot time. < 50 works
    
    bench.repr();
    println!();

    // analyze one single seq
    let id = "tdbR00000365";
    let seq = "AAAUAUGAAGCGAUUUAUUGCAAUUAGUUUCGACCUAAUCUUAGGUGAAAUUCACCCAUAUUUUCCA";
    let exp_fold = "(((((((..((((....)))).(((((.......)))))....((((.....)))))))))))....";
    println!("========== Example for id: {} =============", id);

    let rna_seq = RnaSequence::new(id, exp_fold, seq); 
    let nussinov_score = Score::new(rna_seq.clone(), "nussinov");
    let vienna_score = Score::new(rna_seq, "vienna");

    let _ = nussinov_score.expect("error nussinov").repr();
    let _ = vienna_score.expect("error vienna").repr(); 
    println!();
}
