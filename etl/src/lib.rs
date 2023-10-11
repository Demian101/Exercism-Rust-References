use std::collections::BTreeMap;

pub fn transform(legacy: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    legacy
        .iter()
        .flat_map(|(&scores, ref letters)| {
            letters.iter()
                .map(move |ch| 
                    (ch.to_ascii_lowercase(), scores)
                )
        }).collect()
}
