# RNAligner - RNA alignement benchmarking tool

RNAligner is a benchmarking tool for state of the art
and legacy RNA alignement prediction algorithms.

## Current algorithms to benchmark and compare:

 - Nussinov
 - ViennaRNA

## Getting an alignement for your first RNA sequence

### 1. Import the necessary structs and functions.

The two structs below allow us to build an RnaSequence structure. This struct can be passed in the Score struct to specify vairous folding algorithms and predict various rna structres.

```rust
use rnaligner::io::RnaSequence;
use rnaligner::compare::Score;
```

### 2. Specify the data that will be used.
 - id: the unique identifier of the sequence. 
    - Note that the id is not really useful in this demonstration, since we only have one sequence to deal with. The id will become more useful when dealing with multiple RNA sequences.

 - seq: the RNA sequence used to predict the folded 2d structure

 - exp_fold: the experimental 2d structure used to evaluate the prediction made by our algorithm.


```rust
let id = "123";
let seq = "AAAUAUGAAGCGAUUUAUUGCAAUUAGUUUCGACCUAAUCUUAGGUGAAAUUCACCCAUAUUUUCCA";
let exp_fold = "(((((((..((((....)))).(((((.......)))))....((((.....)))))))))))....";
```

### 3. Create a RnaSequence struct

```rust
let rna_seq = RnaSequence::new(id, exp_fold, seq); 
```

### 4. Now let's calculate the scores for each algorithm

Here we are running both algorithms to see how they stack up against each other:

```rust
// Calculate the score and clone
let nussinov_score = Score::new(rna_seq.clone(), "nussinov");
let vienna_score = Score::new(rna_seq, "vienna");
```

The scores will tell us how accurate each algorithm is compared to the experimental structure.

## Running benchmarks on multiple sequences

If you wanna test a bunch of RNA sequences at once (which is way more useful), here's how:

```rust
use rnaligner::io::parse_fasta;
use rnaligner::benchmark::Benchmark;

// Load sequences from a file (395 is the max my machine can handle with Nussinov)
let seq_list: Vec<RnaSequence> = parse_fasta("data/trna_unmodified_dot_bracket.txt", 395);

// Create and run the benchmark
let bench = Benchmark::new(seq_list);
bench.repr();
```

## What's going on under the hood?

The tool currently implements two folding algorithms:

1. **Nussinov Algorithm**: This is like the grandpa of RNA folding - simple but gets the job done. It's not super accurate but helps understand the basics.

2. **ViennaRNA**: This is the fancy one. It's wrapped from the C implementation and uses way more sophisticated rules for prediction.

## Building the project on macos

The ViennaRNA package is available in the brewsci/bio tap, which needs to be added to your Homebrew installation.
```bash
brew tap brewsci/bio
```

Once the tap is added, you can install the ViennaRNA Package using the brew install command:
```bash
brew install viennarna
```

Then, run:
```bash
cargo build
cargo run
```

The C code for ViennaRNA will be compiled automatically thanks to the build.rs script.



