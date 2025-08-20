pub mod main2;
mod main3;

use std::collections::HashMap;        
fn sono_anagrammi(str1:&str,str2:&str)->bool{
    if str1.len() != str2.len(){
        return false;
    }
    let mut dizionario1=HashMap::new();
    for ch in str1.chars(){
        *dizionario1.entry(ch).or_insert(0) +=1;
    }
    let mut dizionario2=HashMap::new();
    for ch in str2.chars(){
        *dizionario2.entry(ch).or_insert(0) +=1;
    }
    dizionario1==dizionario2
}
#[cfg(test)]
mod tests {
    use super::*;

    fn test_anagrammi() {
        assert!(sono_anagrammi("test","sett"));
        assert!(sono_anagrammi("hello","world"));
    }
}
fn main() {
    println!("{}", sono_anagrammi("hello","world")); 
}




