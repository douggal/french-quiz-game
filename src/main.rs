//use rand::seq::SliceRandom;
use chrono::Utc;
use rand::prelude::IndexedRandom;
use serde::{Deserialize};
use serde_json::Value;
use std::collections::HashMap;
use std::{fs, process};
use std::io::{self, Write};

#[derive(Debug, Deserialize)]
struct VerbConjugations(HashMap<String, HashMap<String, HashMap<String, String>>>);

fn validate_json(json_str: &str) -> Result<Value, serde_json::Error> {
    serde_json::from_str(json_str)
}

fn main() {

    println!("Jeu-questionnaire en français");
    // Start
    let start_datetime = Utc::now();
    println!(
        "Date et heure de début (UTC): {}",
        start_datetime.format("%Y-%m-%d %H:%M:%S")
    );

    let verbs_input_file = "inputs/conjugations.json";

    let file_content = fs::read_to_string(verbs_input_file)
        .expect("Échec de la lecture du fichier d'entrée (failed to read file) verbs.json");

   let validated_input = match validate_json(&file_content) {
        Ok(file_contents) => {
            println!("Felicitations, ce JSON c'est correct !\n\n");
            file_contents.to_string()
        }
        Err(err) => {
            print!("Échec de l'analyse du JSON (could not parse JSON).\n\n");
            eprintln!("Error: {}", err); // Print error message to stderr
            process::exit(1) // Exit with a non-zero status code
        }
   };

    let verb_data: VerbConjugations = serde_json::from_str(&validated_input)
        .expect("Échec de l'analyse du JSON (could not parse JSON)");

    let verbs: Vec<String> = verb_data.0.keys().cloned().collect();
    let tense = vec!["present","passe_compose","futur"];
    let subjects = vec!["je", "tu", "il", "elle", "on", "nous", "vous", "ils", "elles"];

    println!("Bienvenue au jeu de conjugaison française !");
    println!("Tapez la conjugaison correcte du verbe donné au présent.");

    loop {
        let verb = verbs.choose(&mut rand::rng()).unwrap();
        let tense  = tense.choose(&mut rand::rng()).unwrap();
        let subject = subjects.choose(&mut rand::rng()).unwrap();

        print!("Conjuguez '{}' in the {} tense avec le sujet '{}': ", verb, tense, subject);
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let user_input = user_input.trim();

        let correct_answer = verb_data.0.get(verb)
            .and_then(|v| v.get(*tense)
            .and_then(|v| v.get(*subject)));

        match correct_answer {
            Some(answer) if answer == user_input => {
                println!("✅ Correct!");
            }
            Some(answer) => {
                println!("❌ Incorrect. La bonne réponse est: '{}'", answer);
            }
            None => {
                println!("⚠️ Erreur: conjugaison introuvable.");
            }
        }

        println!("Voulez-vous continuer ? (o/n)");
        let mut continue_input = String::new();
        io::stdin().read_line(&mut continue_input).unwrap();
        if continue_input.trim().to_lowercase() != "o" {
            break;
        }
    }

    println!("Merci d'avoir joué !");
}
