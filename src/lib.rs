extern crate hoplite_verbs_rs;
use hoplite_verbs_rs::*;
use rand::seq::SliceRandom;
//use rand::Rng;
use rand::thread_rng;
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
    fn next_form(&mut self) -> Result<HcGreekVerbForm, &str>;
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
    verb_idx:usize,
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
    verbs.shuffle(&mut thread_rng());
    for (idx,a) in verbs.iter().enumerate() {
        println!("v{}: {}", idx, a.pps[0]);
    }

    let persons = vec![HcPerson::First, HcPerson::Second, HcPerson::Third];
    let numbers = vec![HcNumber::Singular, HcNumber::Plural];
    let tenses = vec![HcTense::Present, HcTense::Imperfect, HcTense::Future, HcTense::Aorist, HcTense::Perfect, HcTense::Pluperfect];
    let moods = vec![HcMood::Indicative, HcMood::Subjunctive,HcMood::Optative,HcMood::Imperative];
    let voices = vec![HcVoice::Active,HcVoice::Middle,HcVoice::Passive];

    RandomFormChooser {verbs, unit, history: vec![], params_to_change: 2, reps_per_verb: 4,  persons, numbers, tenses, moods, voices, verb_counter: 0, verb_idx: 0 }
}

impl FormChooser for RandomFormChooser {
    fn next_form(&mut self) -> Result<HcGreekVerbForm, &str> {
        let mut count = 0;
        let mut a:HcGreekVerbForm;
        let mut found = false;

        if self.verbs.len() < 1 {
            return Err("no verbs");
        }
        
        if self.verb_counter >= self.reps_per_verb  {
            //println!("counter over");
            self.verb_idx += 1;
            self.verb_counter = 0;
            
            //shuffle verbs, then cycle through to end and re-shuffle
            if self.verb_idx >= self.verbs.len() {
                println!("shuffle");
                self.verbs.shuffle(&mut thread_rng());
                self.verb_idx = 0;
            }
        }

        self.verb_counter += 1;

        loop {
            count += 1;
            if count > 10000 {
                return Err("overflow");
            }

            //println!("{} {} - ", self.verb_counter, self.verb_idx);
            if self.history.len() < 1 || self.verb_counter == 1 {
                //println!("change verb idx: {}", self.verb_idx);
                //println!("\tverb2: {}", self.verbs[self.verb_idx].pps[0]);
                let person = self.persons.choose(&mut rand::thread_rng()).unwrap().clone();
                let number = self.numbers.choose(&mut rand::thread_rng()).unwrap().clone();
                let tense = self.tenses.choose(&mut rand::thread_rng()).unwrap().clone();
                let voice = self.voices.choose(&mut rand::thread_rng()).unwrap().clone();
                let mood = self.moods.choose(&mut rand::thread_rng()).unwrap().clone();

                a = HcGreekVerbForm { verb: self.verbs[self.verb_idx].clone(), person, number, tense, voice, mood, gender: None, case: None};
            }
            else {
                a = self.history.last().unwrap().clone();
                a.change_params(2, &self.persons, &self.numbers, &self.tenses, &self.voices, &self.moods);
            }

            if let Ok(_f) = a.get_form(false) {
                self.history.push( a );
                found = true;
                break;
            }
            else {
                //println!("\t\tNope Form: {} {:?} {:?} {:?} {:?} {:?}", a.verb.pps[0], a.person, a.number, a.tense, a.mood, a.voice);
            }
            
        }

        if found {
            //if let Some(f) = self.history.last().unwrap() {
                //println!("\tForm: {} {:?} {:?} {:?} {:?} {:?}", self.history.last().unwrap().verb.pps[0], self.history.last().unwrap().person, self.history.last().unwrap().number, self.history.last().unwrap().tense, self.history.last().unwrap().mood, self.history.last().unwrap().voice);
                return Ok(self.history.last().unwrap().clone());
            //}
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

        for _ in 0..=10016 {
            let mut vf = chooser.next_form().unwrap();
            let mut vfs = vf.get_form(false).unwrap().last().unwrap().form.to_string();
            println!("{:?} {:?} {:?} {:?} {:?} \t\t\t- {}", vf.person, vf.number, vf.tense, vf.mood, vf.voice, vfs);
        }

        assert_eq!(chooser.next_form().unwrap().get_form(false).unwrap().last().unwrap().form.to_string(), String::from("ἔλῡσα"));
        assert_ne!(chooser.next_form().unwrap().get_form(false).unwrap().last().unwrap().form.to_string(), String::from("ἔλῡσα"));
    }
}
