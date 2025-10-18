//use rand::seq::SliceRandom;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use chrono::Utc;
use rand::prelude::IndexedRandom;

#[derive(Debug, Deserialize)]
struct VerbConjugations(HashMap<String, HashMap<String, String>>);

fn main() {

    println!("Jeu-questionnaire en français");
    // Start
    let start_datetime = Utc::now();
    println!(
        "Date et heure de début (UTC): {}",
        start_datetime.format("%Y-%m-%d %H:%M:%S")
    );

    let file_content = fs::read_to_string("inputs/verbs.json")
        .expect("Échec de la lecture du fichier d'entrée (failed to read file) verbs.json");
    let verb_data: VerbConjugations = serde_json::from_str(&file_content)
        .expect("Échec de l'analyse du JSON (could not parse JSON)");

    let verbs: Vec<&String> = verb_data.0.keys().collect();
    let subjects = vec!["je", "tu", "il", "elle", "on", "nous", "vous", "ils", "elles"];

    println!("Bienvenue au jeu de conjugaison française !");
    println!("Tapez la conjugaison correcte du verbe donné au présent.");

    loop {
        let verb = verbs.choose(&mut rand::rng()).unwrap();
        let subject = subjects.choose(&mut rand::rng()).unwrap();

        print!("Conjuguez '{}' avec le sujet '{}': ", verb, subject);
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let user_input = user_input.trim();

        let correct_answer = verb_data.0.get(*verb).and_then(|v| v.get(*subject));

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
