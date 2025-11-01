use anyhow::{Result, Context};
use std::fs;

#[derive(Debug, Clone)]
pub struct RnaSequence {
    pub id: String,
    pub sequence: Vec<u8>, // A=0, U=1, G=2, C=3
    pub exp_fold: String,
}

impl RnaSequence {
    pub fn new() -> Self {
        RnaSequence {
            id: String::new(),
            sequence: Vec::new(),
            exp_fold: String::new(),
        }
    }

    pub fn add_id(&mut self, id: &str) {
        self.id.push_str(id)
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn add_seq(&mut self, id: String, seq_str: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.id != id {
            panic!("parsing seq that is not the one indicated by id.");
        }

        let cleaned: String = seq_str
            .chars()
            .filter(|&c| !c.is_whitespace() && c != '_')
            .collect();

       let sequence: Vec<u8> = cleaned
            .chars()
            .map(|c| match c.to_ascii_uppercase() {
                'A' => Ok(0),
                'U' | 'T' => Ok(1),
                'G' => Ok(2),
                'C' => Ok(3),
                _ => anyhow::bail!("Invalid nucleotide: {}. Found in seq: {}", c, id),
            })
            .collect::<Result<Vec<u8>, _>>()?; 

        for i in 0..sequence.len() {
            self.sequence.push(sequence[i])
        }
        Ok(())
    }

    pub fn add_fold(&mut self, id: String, exp_fold: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.id != id {
            panic!("parsing seq that is not the one indicated by id.");
        }
        self.exp_fold.push_str(exp_fold);
        Ok(())
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




pub fn parse_fasta(filepath: &str) -> Vec<RnaSequence> {
    let content = fs::read_to_string(filepath).context("Failed to read FASTA file");

    let mut sequence_list = Vec::new();
    let mut rnaseq = RnaSequence::new();

    let mut id = String::new();

    let mut skip_again = false;

    for line in content.expect("Error: no content in parse_fasta()").lines() {

        if skip_again {
            skip_again = false;
            continue;
        }

        if line.starts_with('>') {
            rnaseq.add_id(line[1..].trim());
            id.push_str(&rnaseq.get_id());

        } else if line.starts_with('(') | line.starts_with('.') {
            if line.contains('[') | line.contains(']') {
                rnaseq = RnaSequence::new();
                skip_again = true;
                continue;
            }

            let _  = rnaseq.add_fold(rnaseq.get_id(), line.trim());
            
        } else if line.starts_with('A') | line.starts_with('U') | line.starts_with('G') | line.starts_with('C') | line.starts_with('T') {
            if line.contains('J') | line.contains('4') {
                rnaseq = RnaSequence::new();
                continue;
            }

            let _ = rnaseq.add_seq(rnaseq.get_id(), line.trim());
            sequence_list.push(rnaseq);
            id.clear();
            rnaseq = RnaSequence::new();
        }
    };
    sequence_list
}

