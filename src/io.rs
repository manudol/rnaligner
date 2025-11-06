use anyhow::{Result, Context};
use std::fs;

#[derive(Debug, Clone)]
pub struct RnaSequence {
    pub id: String,
    pub sequence: Vec<u8>, // A=0, U=1, G=2, C=3
    pub exp_fold: String,
}

impl RnaSequence {
    pub fn new(id: &str, fold: &str, seq: &str) -> Self {
        RnaSequence {
            id: String::from(id),
            sequence: Self::vecu8(seq).expect("RnaSequence::vecu8() fail."),
            exp_fold: String::from(fold),
        }
    }

    fn vecu8(seq: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let cleaned: String = seq
            .chars()
            .filter(|&c| !c.is_whitespace() && c != '_')
            .collect();

        let mut vec_seq = Vec::with_capacity(cleaned.len());
        for c in cleaned.chars() {
            let v = match c.to_ascii_uppercase() {
                'A' => 0,
                'U' | 'T' => 1,
                'G' => 2,
                'C' => 3,
                _ => return Err(format!("Invalid nucleotide: {}", c).into()),
            };
            vec_seq.push(v);
        }

        Ok(vec_seq)
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }
    
    pub fn sizeof(&self) -> usize {
        self.sequence.len()
    }

    pub fn to_string(&self) -> String {
        self.sequence
            .iter()
            .map(|&n| match n {
                0 => 'A',
                1 => 'U',
                2 => 'G',
                3 => 'C',
                _ => '_',
            })
            .collect()
    }
}




pub fn parse_fasta(filepath: &str, max: usize) -> Vec<RnaSequence> {
    let content = fs::read_to_string(filepath)
                        .with_context(|| format!("Failed to read FASTA file: {}", filepath));
    let binding = content.expect("Error lines");
    let mut lines = binding.lines();

    let mut sequence_list: Vec<RnaSequence> = Vec::new();
    let mut count = 0 as usize;
    while let (Some(id_line), Some(fold_line), Some(seq_line)) =
                   (lines.next(), lines.next(), lines.next()) && count < max {

        if fold_line.contains('[') || fold_line.contains(']') {
            continue;
        }

        if seq_line.contains('J') || seq_line.contains('4') {
            continue;
        }

        let id       = id_line.trim_start_matches('>').trim();
        let exp_fold = fold_line.trim();
        let seq      = seq_line.trim();
        
        sequence_list.push(RnaSequence::new(id, exp_fold, seq));
        count += 1;
    };
    sequence_list
}

