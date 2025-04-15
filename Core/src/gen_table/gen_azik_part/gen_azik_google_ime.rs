use crate::config::AzikConfig;
use crate::consonants::Consonants;
use crate::data::vowels::Vowels;
use crate::generate_hiragana::gen_hiragana;
use crate::tokens::AssignableTokens;
use regex::Regex;
use std::str::FromStr;
use std::string::String;

pub fn gen_google_ime_sequence(config: AzikConfig) -> (String, String) {
    let vowels = Vowels::new();
    let consonants = Consonants::new();
    let mut out = String::new();
    let mut out_seq = String::new();
    let mut delete_list: Vec<String> = vec![];

    out.push_str(add_hatsuon_sokuon(&config).as_str());

    for i in &config.Sequence {
        let vowel = vowels[0];

        let sequence: Vec<_> = i.Sequence.to_string().chars().collect();
        let kana_vowel = Vowels::from_str(&sequence[0].to_uppercase().to_string());
        let free_vowel = vowel.vowel_to_hiragana();
        let assignable = i.Token.to_lowercase();

        for j in &consonants {
            let consonant = j;
            let kana = gen_hiragana(*consonant, kana_vowel.expect("It's not vowel or something"));

            if consonant.to_string().chars().count() == 2 {
                out.push_str(
                    &(consonant.to_string().to_lowercase()
                        + &assignable
                        + "	"
                        + kana
                        + free_vowel
                        + "\n"),
                );
            } else {
                out.push_str(
                    &(consonant.to_string().to_lowercase()
                        + &assignable
                        + "		"
                        + kana
                        + free_vowel
                        + "\n"),
                );
            }
        }

        for j in &consonants {
            let consonant = j;
            for i in &config.Sequence {
                if consonant.to_string().chars().last().unwrap().to_string()
                    == i.Token.to_lowercase()
                {
                    for t in &config.Sequence {
                        let re = Regex::new(
                            &(consonant.to_string().to_lowercase()
                                + t.Token.as_str()
                                + r"\t\p{Hiragana}*"),
                        )
                        .unwrap();

                        if let Some(cpp) = re.captures(&out) {
                            delete_list.push(cpp[0].to_string())
                        }
                    }
                    continue;
                }
            }
            if consonant.to_string().to_lowercase() == "v" {
                for t in &config.Sequence {
                    let re =
                        Regex::new(&("v".to_string() + t.Token.as_str() + r"\t\t\p{Hiragana}*"))
                            .unwrap();

                    if let Some(cpp) = re.captures(&out) {
                        delete_list.push(cpp[0].to_string())
                    }
                }
            }
        }

        out_seq.push_str(&assignable);
        out.push("\n".chars().next().unwrap());
    }

    for i in &delete_list {
        out.remove_matches(i);
    }

    (out, out_seq.to_string())
}

fn add_hatsuon_sokuon(config: &AzikConfig) -> String {
    let mut out = String::new();

    let hatsuon_sequence = AssignableTokens::from_str(config.Hatsuon.to_uppercase().as_str())
        .expect("It's not Assignable. Did you use vowels or numbers? change Hatsuon in JSON")
        .to_string()
        .to_lowercase()
        + "\t\t"
        + "ん\n";

    out.push_str(&hatsuon_sequence.to_string());

    let sokuon_sequence = AssignableTokens::from_str(config.Sokuon.to_uppercase().as_str())
        .expect("It's not Assignable. Did you use vowels or numbers? change Sokuon in JSON")
        .to_string()
        .to_lowercase()
        + "\t\t"
        + "っ\n\n";

    out.push_str(&sokuon_sequence.to_string());

    out
}
