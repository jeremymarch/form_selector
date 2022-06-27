extern crate hoplite_verbs_rs;
use hoplite_verbs_rs::*;
use rand::seq::SliceRandom;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub trait FormChooser {
    fn next_form(&self) -> String;
}

pub struct RandomFormChooser {
    pub verbs: Vec<HcGreekVerb>,
    pub unit: u32
}

pub fn init_random_form_chooser(path:&str, unit: u32) -> RandomFormChooser {
    let mut verbs = vec![];
    if let Ok(pp_file) = File::open(path) {
        let pp_reader = BufReader::new(pp_file);
        for (idx, pp_line) in pp_reader.lines().enumerate() {
            if let Ok(line) = pp_line {
                verbs.push(HcGreekVerb::from_string_with_properties(idx as u32, &line).unwrap());
            }
        }
    }
    RandomFormChooser {verbs, unit}
}

impl FormChooser for RandomFormChooser {
    fn next_form(&self) -> String {
        let persons = [HcPerson::First, HcPerson::Second, HcPerson::Third];
        let numbers = [HcNumber::Singular, HcNumber::Plural];
        let tenses = [HcTense::Present, HcTense::Imperfect, HcTense::Future, HcTense::Aorist, HcTense::Perfect, HcTense::Pluperfect];
        let moods = [HcMood::Indicative, HcMood::Subjunctive,HcMood::Optative,HcMood::Imperative];
        let voices = [HcVoice::Active,HcVoice::Middle,HcVoice::Passive];
    
    
        //let luw = "λῡ́ω, λῡ́σω, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην";
        //let a = HcGreekVerb::from_string(1, luw, REGULAR).unwrap();
        if let Some(v) = self.verbs.choose(&mut rand::thread_rng()) {
            //println!("verbs {}", self.verbs.len());
            let p = persons.choose(&mut rand::thread_rng()).unwrap();
            let n = numbers.choose(&mut rand::thread_rng()).unwrap();
            let t = tenses.choose(&mut rand::thread_rng()).unwrap();
            let vo = voices.choose(&mut rand::thread_rng()).unwrap();
            let m = moods.choose(&mut rand::thread_rng()).unwrap();

            println!("Form: {} {:?} {:?} {:?} {:?} {:?}", v.pps[0], p, n, t, m, vo);

            let b = HcGreekVerbForm {verb:&v, person: *p, number: *n, tense: *t, voice: *vo, mood: *m, gender:None, case:None};
            b.get_form(false).unwrap().last().unwrap().form.to_string()
        }
        else {
            String::from("")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {        
        let chooser = init_random_form_chooser("../hoplite_verbs_rs/testdata/pp.txt", 20);

        assert_eq!(chooser.next_form(), "ἔλῡσα");
        assert_ne!(chooser.next_form(), "ἔλῡσfα");
    }
}
