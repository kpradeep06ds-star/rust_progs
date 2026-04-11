pub struct Allergies{
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
         Allergies{score: score & 255}
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let v = match allergen{
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4, 
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16, 
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128
        };
        self.score & v != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let list:Vec<Allergen> = vec!(Allergen::Eggs, Allergen::Peanuts, Allergen::Shellfish,
        Allergen::Strawberries, Allergen::Tomatoes, Allergen::Chocolate, Allergen::Pollen,
        Allergen::Cats );
        
        let mut v:Vec<Allergen> = Vec::new();

        for i in list{
            if self.is_allergic_to(&i){
                v.push(i);
            }
        }
        v
    }
}
