extern crate hoplite_verbs_rs;
use hoplite_verbs_rs::*;
use polytonic_greek::hgk_compare_multiple_forms;
use rand::seq::SliceRandom;
//use rand::Rng;
use rand::thread_rng;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::sync::Arc; //https://stackoverflow.com/questions/41770184/arc-reference-to-member-of-field

pub trait FormChooser {
    fn next_form(&mut self, prev_answer:Option<&str>) -> Result<(HcGreekVerbForm, Option<bool>), &str>;
    fn set_reps_per_verb(&mut self, reps:u8);
}

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
    pub change_verb_incorrect: bool,
}

pub fn init_random_form_chooser(path:&str, unit: u32) -> RandomFormChooser {
    let mut verbs = vec![];
    if let Ok(pp_file) = File::open(path) {
        let pp_reader = BufReader::new(pp_file);
        for (idx, pp_line) in pp_reader.lines().enumerate() {
            if let Ok(line) = pp_line {
                if !line.starts_with('#') { //skip commented lines
                    verbs.push(Arc::new(HcGreekVerb::from_string_with_properties(idx as u32, &line).unwrap()));
                }
                // else {
                //     println!("skip");
                // }
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

    RandomFormChooser {verbs, unit, history: vec![], params_to_change: 2, reps_per_verb: 4,  persons, numbers, tenses, moods, voices, verb_counter: 0, verb_idx: 0, change_verb_incorrect: true }
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
        
        let mut is_correct:Option<bool> = None;

        if self.verbs.is_empty() {
            return Err("no verbs");
        }

        if !self.history.is_empty() {
            if let Some(pa) = prev_answer {
                //check verb
                let prev_form = self.history.last().unwrap();
                let prev_s = prev_form.get_form(false).unwrap().last().unwrap().form.to_string();

                is_correct = Some(hgk_compare_multiple_forms(&prev_s.replace('/', ","), pa));

                if !is_correct.unwrap() && self.change_verb_incorrect {
                    self.verb_counter = self.reps_per_verb;
                }
            }
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
            if self.history.is_empty() || self.verb_counter == 1 {
                //println!("change verb idx: {}", self.verb_idx);
                //println!("\tverb2: {}", self.verbs[self.verb_idx].pps[0]);
                let person = *self.persons.choose(&mut rand::thread_rng()).unwrap();
                let number = *self.numbers.choose(&mut rand::thread_rng()).unwrap();
                let tense = *self.tenses.choose(&mut rand::thread_rng()).unwrap();
                let voice = *self.voices.choose(&mut rand::thread_rng()).unwrap();
                let mood = *self.moods.choose(&mut rand::thread_rng()).unwrap();

                a = HcGreekVerbForm { verb: self.verbs[self.verb_idx].clone(), person, number, tense, voice, mood, gender: None, case: None};
            
                if let Ok(_f) = a.get_form(false) {
                    self.history.push( a );
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
                break; //this breeak and the counter at the start are the only ways out of the loop
            }
            else {
                //println!("\t\tNope Form: {} {:?} {:?} {:?} {:?} {:?}", a.verb.pps[0], a.person, a.number, a.tense, a.mood, a.voice);
                continue;
            }
        }

        Ok((self.history.last().unwrap().clone(), is_correct))
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
            let vf = chooser.next_form(None).unwrap();
            let is_correct = vf.1;
            let vfs = vf.0.get_form(false).unwrap().last().unwrap().form.to_string();
            println!("{:?} {:?} {:?} {:?} {:?} \t\t\t- {} {:?}", vf.0.person, vf.0.number, vf.0.tense, vf.0.mood, vf.0.voice, vfs, is_correct);
        }

        assert_eq!(chooser.next_form(None).unwrap().0.get_form(false).unwrap().last().unwrap().form.to_string(), String::from("ἔλῡσα"));
        assert_ne!(chooser.next_form(None).unwrap().0.get_form(false).unwrap().last().unwrap().form.to_string(), String::from("ἔλῡσα"));
    }
}
