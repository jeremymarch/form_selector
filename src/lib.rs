extern crate hoplite_verbs_rs;
use hoplite_verbs_rs::*;
use rand::seq::SliceRandom;
//use rand::Rng;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::sync::Arc; //https://stackoverflow.com/questions/41770184/arc-reference-to-member-of-field
//use std::rc::Rc;

//if we want a version where verb is an index in an array?
// #[derive(Eq, PartialEq, Debug, Clone)]
// pub struct SmallGreekVerbForm {
//     pub verb: usize,
//     pub person: HcPerson,
//     pub number: HcNumber,
//     pub tense: HcTense,
//     pub voice: HcVoice,
//     pub mood: HcMood,
//     pub gender: Option<HcGender>,
//     pub case: Option<HcCase>,
// }

pub trait FormChooser {
    fn next_form(&mut self) -> Result<String, &str>;
}

// pub struct RandomFormChooser<'a> {
//     pub verbs: Vec<Arc<Box<HcGreekVerb>>>,
//     pub unit: u32,
//     pub history:Vec<Arc<Box<HcGreekVerbForm<'a>>>>
// }

pub struct RandomFormChooser {
    pub verbs: Vec<Arc<HcGreekVerb>>,
    pub unit: u32,
    pub history: Vec<HcGreekVerbForm>,
    pub params_to_change: u8,
    pub reps_per_verb: u8,
    pub persons: Vec<HcPerson>,
    pub numbers: Vec<HcNumber>,
    pub tenses: Vec<HcTense>,
    pub moods: Vec<HcMood>,
    pub voices: Vec<HcVoice>,
    verb_counter: u8,
}

pub fn init_random_form_chooser(path:&str, unit: u32) -> RandomFormChooser {
    let mut verbs = vec![];
    if let Ok(pp_file) = File::open(path) {
        let pp_reader = BufReader::new(pp_file);
        for (idx, pp_line) in pp_reader.lines().enumerate() {
            if let Ok(line) = pp_line {
                verbs.push(Arc::new(HcGreekVerb::from_string_with_properties(idx as u32, &line).unwrap()));
            }
        }
    }

    let persons = vec![HcPerson::First, HcPerson::Second, HcPerson::Third];
    let numbers = vec![HcNumber::Singular, HcNumber::Plural];
    let tenses = vec![HcTense::Present, HcTense::Imperfect, HcTense::Future, HcTense::Aorist, HcTense::Perfect, HcTense::Pluperfect];
    let moods = vec![HcMood::Indicative, HcMood::Subjunctive,HcMood::Optative,HcMood::Imperative];
    let voices = vec![HcVoice::Active,HcVoice::Middle,HcVoice::Passive];

    RandomFormChooser {verbs, unit, history: vec![], params_to_change: 2, reps_per_verb: 4,  persons, numbers, tenses, moods, voices, verb_counter: 0 }
}

impl FormChooser for RandomFormChooser {
    fn next_form(&mut self) -> Result<String, &str> {
        let mut count = 0;
        let mut a:HcGreekVerbForm;
        let mut found = false;
        loop {
            count += 1;
            if count > 1000 {
                return Err("overflow");
            }

            //if self.history.len() < 1 {
                //let l = self.verbs.len();
                let v = self.verbs.choose(&mut rand::thread_rng()).unwrap();
                //let v = rand::thread_rng().gen_range(0..self.verbs.len());
                let person = self.persons.choose(&mut rand::thread_rng()).unwrap().clone();
                let number = self.numbers.choose(&mut rand::thread_rng()).unwrap().clone();
                let tense = self.tenses.choose(&mut rand::thread_rng()).unwrap().clone();
                let voice = self.voices.choose(&mut rand::thread_rng()).unwrap().clone();
                let mood = self.moods.choose(&mut rand::thread_rng()).unwrap().clone();

                a = HcGreekVerbForm { verb:v.clone(), person, number, tense, voice, mood, gender: None, case: None};
                if let Ok(_f) = a.get_form(false) {
                    self.history.push( a );
                    found = true;
                    break;
                }
            //}
            //println!("Form: {} {:?} {:?} {:?} {:?} {:?}", v.pps[0], p, n, t, m, vo);
        }
        if found {
            if let Ok(f) = self.history.last().unwrap().get_form(false) {
                return Ok(f.last().unwrap().form.to_string());
            }
        }

        return Err("overflow");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {        
        let mut chooser = init_random_form_chooser("../hoplite_verbs_rs/testdata/pp.txt", 20);
        // chooser.persons = vec![HcPerson::First];
        // chooser.numbers = vec![HcNumber::Singular];
        assert_eq!(chooser.next_form(), Ok(String::from("ἔλῡσα")));
        assert_ne!(chooser.next_form(), Ok(String::from("ἔλῡσα")));
    }
}
