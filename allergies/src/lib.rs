pub struct Allergies{ score: u32 }

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 1 << 1,     // 2
    Shellfish = 1 << 2,   // 4 
    Strawberries = 1 << 3, // 8
    Tomatoes = 1 << 4,  // 16
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

impl Allergies {
    pub fn new(score: u32) -> Self { Self { score } }

    // 检查 score 是否包含该过敏原: 只需按位与, 看是否 !=0 即可
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & (*allergen as u32) != 0
    }

    // 给出该分数对应的所有过敏原
    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergens = Vec::new();
        for i in 0..8 {
            let spec_score = 1 << i;
            if (self.score & spec_score) !=0 {  // 按位与
                if let Some(allergen) = Allergen::from_u32(spec_score) {
                    allergens.push(allergen);
                }
            }
        }
        allergens
    }
}

impl Allergen {
    fn from_u32(value: u32) -> Option<Self> {
        match value {
            1 => Some(Allergen::Eggs),
            2 => Some(Allergen::Peanuts),
            4 => Some(Allergen::Shellfish),
            8 => Some(Allergen::Strawberries),
            16 => Some(Allergen::Tomatoes),
            32 => Some(Allergen::Chocolate),
            64 => Some(Allergen::Pollen),
            128 => Some(Allergen::Cats),
            _ => None,
        }
    }
}