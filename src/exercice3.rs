// Exercice 2.3
// Utilisation du module std::io pour traiter les input et les output.
use std::io;

pub fn main() {
    loop {
        println!("Entrez le nombre de terme de la suite à calculer n avec n > 0 (0 pour terminer) :");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Erreur");

        let n: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {     // On gère les erreurs de saisie.
                println!("Erreur, la valeur doit etre un nombre entier.");
                continue;
            }
        };

        // Sortie de la boucle si n <= 0.
        if n <= 0 {
            println!("Fin du programme");
            break;
        }

        let mut u = 1.0;
        for i in 2..=n {
            u += 1.0 / i as f64;
        }

        println!("U{} est : {:.4}", n, u);
    }
}