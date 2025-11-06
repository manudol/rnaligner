pub struct Matrix {
    seq_len: usize,
    seq_str: String,
    matrix: Vec<Vec<Node>>,
}

#[derive(Debug, Clone)]
pub struct Node {
    default_0: bool,
    index: [usize; 2],
    is_match: bool,
    possible_paths: Vec<[usize; 2]>,
    value: u8,
    optimal_k: Option<usize>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            default_0: false,
            index: [0, 0],
            is_match: false,
            possible_paths: Vec::new(), 
            value: 0,
            optimal_k: None,
        }
    }
}

impl Matrix {
    fn can_pair(a: u8, b: u8) -> bool {
        match (a, b) {
                (b'A', b'U') | (b'U', b'A') => true, // A - U
                (b'G', b'C') | (b'C', b'G') => true, // G - C
                (b'G', b'U') | (b'U', b'G') => true, // G - U wobble pair
                _ => false,
            }
    }

    pub fn init_matrix(seq_str: String) -> Self {

        let mut matrix: Vec<Vec<Node>> = Vec::new();
        let mut row: Vec<Node> = Vec::new();
        
        let seq_bytes = seq_str.as_bytes();
        let seq_byte_len = seq_bytes.len();

        let seq_len = seq_str.len();

        for i in 0..seq_byte_len { 
            for j in 0..seq_byte_len {
                let mut node = Node::new();
                if j <= i {
                    node.default_0 = true;
                } else {
                    node.index = [i , j];
                    node.is_match = Self::can_pair(seq_bytes[i], seq_bytes[j]);
                }
                row.push(node);
            }
            matrix.push(row.clone());
            row.clear();
        }
        return Self { seq_len: seq_len, seq_str: seq_str, matrix: matrix }
    }

    fn calculate_bifucation(&self, j: usize, k: usize) -> (u8, usize) {
        let mut bifurcation = 0;
        let mut optimal_k: usize = 0 as usize;
        // if k <= j + 1 {
        //     return (0, j);
        // }
        for bifurc in j..k {
            let bifurc_value = self.matrix[j][bifurc].value + self.matrix[bifurc+1][k].value;
            if bifurc_value > bifurcation {
                bifurcation = bifurc_value;
                optimal_k = bifurc;
            }
        }
        (bifurcation, optimal_k)
    }

    pub fn find_values(&mut self) {
        // Fill diagonally: for each subsequence length
        const MIN_LS: usize = 3;
        for length in (MIN_LS + 1)..self.seq_len {
            for i in 0..(self.seq_len - length) {
                let j = i + length;
                
                if j <= i {
                    continue;
                }
                
                let is_match = self.matrix[i][j].is_match;
                
                // Case 1: i unpaired
                let i_unpaired = if i + 1 <= j { self.matrix[i + 1][j].value } else { 0 };
                
                // Case 2: j unpaired
                let j_unpaired = if j > 0 && i <= j - 1 { self.matrix[i][j - 1].value } else { 0 };
                
                // Case 3: i-j paired (if they can pair)
                let paired = if is_match && i + 1 < j {
                    self.matrix[i + 1][j - 1].value + 1
                } else {
                    0
                };
                
                // Case 4: Bifurcation
                let (bifurcation, optimal_k) = self.calculate_bifucation(i, j);
                
                let max_val = i_unpaired.max(j_unpaired).max(paired).max(bifurcation);
                
                let node = &mut self.matrix[i][j];
                node.value = max_val;
                
                // Track which path led to max
                if paired == max_val && paired > 0 {
                    node.possible_paths.push([i + 1, j - 1]);
                }
                if j_unpaired == max_val {
                    node.possible_paths.push([i, j - 1]);
                }
                if i_unpaired == max_val {
                    node.possible_paths.push([i + 1, j]);
                }
                if bifurcation == max_val {
                    node.optimal_k = Some(optimal_k);
                }
            }
        }
    }
//     pub fn find_values(&mut self) {
//         for i in 1..self.seq_len {
//             for j in 0..self.seq_len {
//                 let _counter = 0;
//                 for k in 1..self.seq_len {
//                     
//                     if self.matrix[j][k].default_0 {
//                         continue;
//                     }
//                     
//                     if k == i + j {
//                         let is_match = self.matrix[j][k].is_match;
//                         
//                         if is_match {
//                             let k_unpaired  = self.matrix[j][k-1].value;
//                             let j_unpaired  = self.matrix[j+1][k].value;
//                             let paired      = self.matrix[j+1][k-1].value;
//                             let paired_plus = paired + 1;
//                             
//                             let (bifurcation, optimal_k) = self.calculate_bifucation(j, k);
//                             
//                             let max_val = paired_plus.max(j_unpaired).max(k_unpaired).max(bifurcation);
//                             
//                             let node: &mut Node = &mut self.matrix[j][k];
//                             node.value = max_val;
// 
//                             if paired_plus == max_val {
//                                 node.possible_paths.push([j+1, k-1]);
//                             }
//                             if k_unpaired == max_val {
//                                 node.possible_paths.push([j, k-1]);
//                             }
//                             if j_unpaired == max_val {
//                                 node.possible_paths.push([j+1, k]);
//                             }
//                             if bifurcation == max_val {
//                                 node.optimal_k = Some(optimal_k);
//                             }
//                             
//                         } else {
//                             let k_unpaired = self.matrix[j][k-1].value;
//                             let j_unpaired = self.matrix[j+1][k].value;
// 
//                             let (bifurcation, optimal_k) = self.calculate_bifucation(j, k);
//                             
//                             let max_val = j_unpaired.max(k_unpaired).max(bifurcation);
// 
//                             let node: &mut Node = &mut self.matrix[j][k];
//                             node.value = max_val;
// 
//                             if k_unpaired == max_val {
//                                 node.possible_paths.push([j, k-1]);
//                             }
//                             if j_unpaired == max_val {
//                                 node.possible_paths.push([j+1, k]);
//                             }
//                             if bifurcation == max_val {
//                                 node.optimal_k = Some(optimal_k);
//                             }
//                         }
// 
//                     } else if k > i + j {
//                         break;
//                     }
//                 }
// 
//                 if j == (self.seq_len - 1) - i {
//                     break;
//                 }
//             }
//         }
//     }

    pub fn new(seq_str: String) -> Matrix {
        let mut matrix = Matrix::init_matrix(seq_str);
        matrix.find_values();
        matrix
    }

    pub fn print_mat(&self) {
        let mut counter = 0;
        let bytes = self.seq_str.as_bytes();
        print!(" ");
        for c in self.seq_str.chars() {
            print!(" {}", c);
        }
        println!();
        for i in 0..self.seq_len {
            print!("{}", bytes[i] as char);
            for j in 0..self.seq_len {
                print!(" {}", self.matrix[i][j].value);
                counter += 1;
            }
            println!();
        }
        println!("counter: {}", counter);
        println!("seq_len: {}", self.seq_len);
        assert_eq!(counter, self.seq_len * self.seq_len);
    }
    
    pub fn traceback(&self) -> Vec<[usize; 2]> {
        let mut base_pairs: Vec<[usize; 2]> = Vec::new();

        let mut stack: Vec<[usize; 2]> = Vec::new();

        stack.push([0, self.seq_len - 1]);

        while let Some([i, j]) = stack.pop() {
            
            if j < i + 3 {
                continue;
            }

            let node = &self.matrix[i][j];

            if let Some(k_opt) = node.optimal_k {
                stack.push([i, k_opt]);
                stack.push([k_opt + 1, j]);
            } else if !node.possible_paths.is_empty() {
                let next_step = node.possible_paths[0]; 
                
                if next_step == [i + 1, j - 1] {
                    let inner_score = self.matrix[i + 1][j - 1].value;
                    if node.value == inner_score + 1 {
                        base_pairs.push([i, j]);
                    }
                }

                stack.push(next_step);
            }
        }

        base_pairs
    }

    pub fn predict_fold(&self) -> String {
        let pairs = self.traceback();
        let mut fold: Vec<char> = vec!['.'; self.seq_len];
    
        for [i, j] in &pairs {
            fold[*i] = '(';
            fold[*j] = ')';
        }
    
        let fold_string: String = fold.iter().collect();

        fold_string
    }
}

