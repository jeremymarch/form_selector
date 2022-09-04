extern crate hoplite_verbs_rs;
use hoplite_verbs_rs::*;
use rustunicodetests::hgk_compare;
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
    fn next_form(&mut self, prev_answer:Option<&str>) -> Result<(HcGreekVerbForm, Option<bool>), &str>;
    fn set_reps_per_verb(&mut self, reps:u8);
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
    pub verb_counter: u8,
    verb_idx: usize,
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
    // for (idx,a) in verbs.iter().enumerate() {
    //     println!("v{}: {}", idx, a.pps[0]);
    // }

    //default params available, these can be set
    let persons = vec![HcPerson::First, HcPerson::Second, HcPerson::Third];
    let numbers = vec![HcNumber::Singular, HcNumber::Plural];
    let tenses = vec![HcTense::Present, HcTense::Imperfect, HcTense::Future, HcTense::Aorist, HcTense::Perfect, HcTense::Pluperfect];
    let moods = vec![HcMood::Indicative, HcMood::Subjunctive, HcMood::Optative, HcMood::Imperative];
    let voices = vec![HcVoice::Active, HcVoice::Middle, HcVoice::Passive];

    RandomFormChooser {verbs, unit, history: vec![], params_to_change: 2, reps_per_verb: 4,  persons, numbers, tenses, moods, voices, verb_counter: 0, verb_idx: 0 }
}

impl FormChooser for RandomFormChooser {
    fn set_reps_per_verb(&mut self, reps:u8) {
        if reps > 0 && reps < 10 {
            self.reps_per_verb = reps;
        }
        else {
            self.reps_per_verb = 4;
        }
    }

    fn next_form(&mut self, prev_answer:Option<&str>) -> Result<(HcGreekVerbForm, Option<bool>), &str> {
        let mut count = 0;
        let mut a:HcGreekVerbForm;
        let mut found = false;
        let mut is_correct:Option<bool> = None;

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

        if self.history.len() > 0 && prev_answer.is_some() {
            //check verb
            let prev_form = self.history.last().unwrap();
            let prev_s = prev_form.get_form(false).unwrap().last().unwrap().form.to_string();

            if hgk_compare(&prev_s, prev_answer.unwrap(), 0) == 0 {
                //println!("correct");
                is_correct = Some(true);
            }
            else {
                //println!("incorrect");
                is_correct = Some(false);
            }
        }

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
            
                if let Ok(_f) = a.get_form(false) {
                    self.history.push( a );
                    found = true;
                    //break;
                }
                else {
                    //println!("\t\tNope Form: {} {:?} {:?} {:?} {:?} {:?}", a.verb.pps[0], a.person, a.number, a.tense, a.mood, a.voice);
                    continue;
                }
            }

            a = self.history.last().unwrap().clone();
            a.change_params(2, &self.persons, &self.numbers, &self.tenses, &self.voices, &self.moods);

            if let Ok(_f) = a.get_form(false) {
                self.history.push( a );
                found = true;
                break;
            }
            else {
                //println!("\t\tNope Form: {} {:?} {:?} {:?} {:?} {:?}", a.verb.pps[0], a.person, a.number, a.tense, a.mood, a.voice);
                continue;
            }
        }

        if found {
            //if let Some(f) = self.history.last().unwrap() {
                //println!("\tForm: {} {:?} {:?} {:?} {:?} {:?}", self.history.last().unwrap().verb.pps[0], self.history.last().unwrap().person, self.history.last().unwrap().number, self.history.last().unwrap().tense, self.history.last().unwrap().mood, self.history.last().unwrap().voice);
                return Ok((self.history.last().unwrap().clone(), is_correct));
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
        chooser.set_reps_per_verb(6);
        // chooser.persons = vec![HcPerson::First];
        // chooser.numbers = vec![HcNumber::Singular];

        for _ in 0..=10016 {
            let mut vf = chooser.next_form(None).unwrap();
            let is_correct = vf.1;
            let mut vfs = vf.0.get_form(false).unwrap().last().unwrap().form.to_string();
            println!("{:?} {:?} {:?} {:?} {:?} \t\t\t- {} {:?}", vf.0.person, vf.0.number, vf.0.tense, vf.0.mood, vf.0.voice, vfs, is_correct);
        }

        assert_eq!(chooser.next_form(None).unwrap().0.get_form(false).unwrap().last().unwrap().form.to_string(), String::from("ἔλῡσα"));
        assert_ne!(chooser.next_form(None).unwrap().0.get_form(false).unwrap().last().unwrap().form.to_string(), String::from("ἔλῡσα"));
    }
}
