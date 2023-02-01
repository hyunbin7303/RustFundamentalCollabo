
//https://youtu.be/tM2r9HD4ivQ?t=1477
//https://medium.com/digitalfrontiers/rust-dynamic-dispatching-deep-dive-236a5896e49b

fn main() {
}
pub fn spellcheck<C: Spellchecker>(input: &str, spellchecker: C) -> String {
    let mut result = input.to_owned();
    for change in spellchecker.check(input) {
        apply_change(&mut result, change);
    }
    result
}

// Real world implementation
pub fn spellcheck_real(input: &str, spellchecker: &dyn Spellchecker) -> String {
    let mut result = input.to_owned();
    for change in spellchecker.check(input) {
        apply_change(&mut result, change);
    }
    result
}

pub trait Spellchecker { 
    fn check(&self, input: &str) -> Vec<Change>;
}
struct NoopSpellChecker;
impl Spellchecker for NoopSpellChecker {
    fn check(&self, input: &str) -> Vec<Change> {
        Vec::new()
    }
}
struct AntiSpaceChecker;
impl Spellchecker for AntiSpaceChecker {
    fn check(&self, input: &str) -> Vec<Change> {
        input
            .match_indices(" ")
            .map(|(index, space)| Change::Delete(index..index +space.len()))
            .collect()
    }
}

pub fn apply_change(string: *mut String, change: Change) {
    // TODO : Implement.
}
pub enum Change { 
    Delete(std::ops::Range<usize>),
    Replace(std::ops::Range<usize>, String),
}

// Static Dispatching
// Resolving the function calls already at compile time is called static dispatch.
// Limitation - need a generic type parameter for each generic component of our service
// 2. Since the comipler resolves the generic type with a concrete one at compile time, we can use only one type here?..


// Dynamic dispatching 
// Trait objects can be though of like objects of an interface type. 


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works(){
        let txt = "hello world kevin";
        let result= spellcheck(txt, NoopSpellChecker);
        assert_eq!(txt, result);
        let result2= spellcheck(txt, AntiSpaceChecker);
    
        spellcheck_real(txt, &NoopSpellChecker);
        spellcheck_real(txt, &AntiSpaceChecker); 
    }
}