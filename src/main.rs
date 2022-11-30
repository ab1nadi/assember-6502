mod lexical_analyzer;

use crate::lexical_analyzer::LexicalAnalyzer;

fn main() {
        let mut z = LexicalAnalyzer::new("file.txt".to_string(), false).unwrap();

        // barrow z
        for i in &mut z 
        {
            let t = i.unwrap();
            println!("{:?}", t);
        } 



}
