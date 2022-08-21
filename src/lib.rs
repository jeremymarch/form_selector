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
    pub history: Vec<HcGreekVerbForm>
}

pub fn init_random_form_chooser(path:&str, unit: u32) -> RandomFormChooser {
    let mut verbs = vec![];
    if let Ok(pp_file) = File::open(path) {
        let pp_reader = BufReader::new(pp_file);
        for (idx, pp_line) in pp_reader.lines().enumerate() {
            if let Ok(line) = pp_line {
                //verbs.push(Arc::new(Box::new(HcGreekVerb::from_string_with_properties(idx as u32, &line).unwrap())));
                verbs.push(Arc::new(HcGreekVerb::from_string_with_properties(idx as u32, &line).unwrap()));
            }
        }
    }
    RandomFormChooser {verbs, unit, history: vec![] }
}

impl FormChooser for RandomFormChooser {
    fn next_form(&mut self) -> Result<String, &str> {
        let persons = [HcPerson::First, HcPerson::Second, HcPerson::Third];
        let numbers = [HcNumber::Singular, HcNumber::Plural];
        let tenses = [HcTense::Present, HcTense::Imperfect, HcTense::Future, HcTense::Aorist, HcTense::Perfect, HcTense::Pluperfect];
        let moods = [HcMood::Indicative, HcMood::Subjunctive,HcMood::Optative,HcMood::Imperative];
        let voices = [HcVoice::Active,HcVoice::Middle,HcVoice::Passive];
    
    
        //let luw = "λῡ́ω, λῡ́σω, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην";
        //let a = HcGreekVerb::from_string(1, luw, REGULAR).unwrap();
        let mut count = 0;
        loop {
            count += 1;
            if count > 1000 {
                return Err("overflow");
            }

            if self.history.len() < 1 {
                //let l = self.verbs.len();
                let v = self.verbs.choose(&mut rand::thread_rng()).unwrap();
                //let v = rand::thread_rng().gen_range(0..self.verbs.len());
                let person = persons.choose(&mut rand::thread_rng()).unwrap().clone();
                let number = numbers.choose(&mut rand::thread_rng()).unwrap().clone();
                let tense = tenses.choose(&mut rand::thread_rng()).unwrap().clone();
                let voice = voices.choose(&mut rand::thread_rng()).unwrap().clone();
                let mood = moods.choose(&mut rand::thread_rng()).unwrap().clone();

                //self.history.push( SmallGreekVerbForm {verb:v, person, number, tense, voice, mood, gender:None, case:None});
                self.history.push( HcGreekVerbForm {verb:v.clone(), person, number, tense, voice, mood, gender:None, case:None});
            }

            //println!("Form: {} {:?} {:?} {:?} {:?} {:?}", v.pps[0], p, n, t, m, vo);

            if let Some(b) = self.history.last() {
                let a = HcGreekVerbForm { verb:b.verb.clone(), person:b.person, number:b.number, tense:b.tense, voice:b.voice,mood:b.mood,gender:None, case:None};
                if let Ok(f) = a.get_form(false) {
                    //self.form = Some(b.clone());
                    return Ok(f.last().unwrap().form.to_string());
                }
            }
            // }
            // else {
            //     continue;
            // }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {        
        let mut chooser = init_random_form_chooser("../hoplite_verbs_rs/testdata/pp.txt", 20);

        assert_eq!(chooser.next_form(), Ok(String::from("ἔλῡσα")));
        assert_ne!(chooser.next_form(), Ok(String::from("ἔλῡσfα")));
    }
}