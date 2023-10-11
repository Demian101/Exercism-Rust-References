use std::collections::HashMap;

const DNA: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count_nucl(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !DNA.contains(&nucleotide){ 
        Err(nucleotide) 
    } else if let Some(error_char) = dna.chars().find(|c| !DNA.contains(&c)) {
        Err(error_char) // find returns a Result & Some(error_char) unpack it.
    } else {
        Ok(dna.chars().filter(|&c| c == nucleotide).count())
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    DNA.iter().map(|&c| 
        Ok((c, count_nucl(c, dna)?)) 
    ).collect()
}