#[derive(Debug, PartialEq)]
pub struct Dna { nucleotides: String }

#[derive(Debug, PartialEq)]
pub struct Rna { nucleotides: String }

const DNA: [char; 4] = [ 'G', 'C', 'T', 'A' ];
const RNA: [char; 4] = [ 'C', 'G', 'A', 'U' ];

fn validate(s: &str, chars: [char; 4]) -> Result<String, usize> {
    match s.chars().position(|c| !chars.contains(&c)) {
        Some(x) => Err(x),
        None => Ok(s.to_string())
    }
}

fn transcribe(nucleotide: char) -> char {
    // 查找核苷酸在 DNA 数组中的位置，然后返回 RNA 数组中相应位置的字符来实现的.
    RNA[DNA.iter().position(|&c| c == nucleotide).unwrap()]
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        validate(dna, DNA).map(|nucleotides| Dna { nucleotides })
    }

    pub fn into_rna(self) -> Rna {
        Rna { nucleotides: self.nucleotides.chars().map(|c|
             transcribe(c)).collect() 
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        validate(rna, RNA).map(|nucleotides| Rna { nucleotides })
    }
}

