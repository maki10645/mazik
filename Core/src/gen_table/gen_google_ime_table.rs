use crate::config::AzikConfig;
use crate::data::consonants::Consonants;
use crate::data::table_base::GOOGLE_IME_TABLE_BASE;
use crate::data::vowels::Vowels;
use crate::gen_table::gen_azik_part::gen_azik_google_ime::gen_google_ime_sequence;
use crate::generate_hiragana::gen_hiragana;
use regex::Regex;

pub fn gen_google_ime_table(config: AzikConfig) -> String {
    let mut out = String::new();
    out.push_str(GOOGLE_IME_TABLE_BASE);

    let gen_seq = gen_google_ime_sequence(config);
    let sequences = gen_seq.0;
    let tokens = gen_seq.1;
    out.push_str(sequences.as_str());

    for i in 0..39 {
        let consonant = *Consonants::new().get(i).unwrap();
        let consonant_alph = consonant.to_string().to_lowercase();
        for j in 0..5 {
            let mut dup_flag = false;
            let vowel = *Vowels::new().get(j).unwrap();
            let kana = gen_hiragana(consonant, vowel).to_string() + "\n";
            let vowel_alph = vowel.to_string();
            let re =
                Regex::new(&(consonant_alph.to_lowercase() + "\t\t" + r"([ぁ-んー]*)")).unwrap();
            let consonant_last = consonant_alph
                .to_lowercase()
                .as_str()
                .chars()
                .last()
                .unwrap();

            let mut alphs = consonant_alph.clone().to_lowercase() + &vowel_alph.to_lowercase();

            for t in tokens.split("") {
                if consonant_last.to_string() == t && consonant_alph.len() >= 2 {
                    //println!("{}\t{}", consonant_alph_low, t);

                    let lhs = match re.captures(&sequences.to_lowercase()) {
                        Some(caps) => alphs.replace(&consonant_alph, &caps[1]),
                        None => "How".to_string(),
                    };

                    out.push_str(&(lhs + "	" + kana.as_str()));
                    alphs.remove_matches(&(alphs.clone() + "\t" + kana.as_str()));
                    dup_flag = true
                }
            }
            if consonant_alph == "v" {
                let lhs = match re.captures(&sequences.to_lowercase()) {
                    Some(caps) => alphs.replace(&consonant_alph, &caps[1]),
                    None => "How".to_string(),
                };

                out.push_str(&(lhs + "	" + kana.as_str()));
                alphs.remove_matches(&(alphs.clone() + "\t" + kana.as_str()));
                dup_flag = true
            }

            if !dup_flag {
                out.push_str(&(alphs.clone() + "\t" + kana.as_str()));
            }
        }
    }
    out.push("\n".chars().next().unwrap());
    out
}
