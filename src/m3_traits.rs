/// This is about how to use traits and enum 
/// If we have Some type of Enums where they share same methods instead of 
/// writing for each one we can abstract them into trait and implemnt 

trait Attacker {
    fn choose_styel(&self)->String;
}
#[derive(Debug)]
enum Charecter{
    Warrier,
    Archer, 
    Wizard
}


impl Attacker for Charecter{
    fn choose_styel(&self)->String {
        match self {
            Charecter::Archer => String::from("kung fu"),
            Charecter::Warrier => String::from("wing chun"),
            Charecter::Wizard => String::from("thai chai"),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_traits(){
       
       let charecter = Charecter::Wizard;
       let style = charecter.choose_styel();

       dbg!(style);

    }
}