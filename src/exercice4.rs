// Exercice 2.4
// Utilisation du module std::io pour traiter les input et les output.
use std::io;

// Fonction principale.
pub fn main() {
    loop {
        println!("Choisissez une option :");
        println!("1. Carré");
        println!("2. Rectangle");
        println!("3. Cercle");
        println!("4. Quitter");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Erreur de lecture.");

        // On vérifie que le nombre entré par l'utilisateur est valide.
        let choix: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Erreur, la valeur doit etre un nombre entier.");
                continue;
            }
        };

        // On initialise la variable surface comme résultat. f64 pour plus de précision.
        let mut surface = 0.0;

        // On associe le nombre entré par l'utilisateur au calculs associés.
        match choix {

            // Carré
            1 => {
                println!("Entrez le côté du carré :");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Erreur de lecture.");
                let cote: f64 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Erreur, la valeur doit etre un nombre.");
                        continue;
                    }
                };
                surface = cote * cote;
            }

            // Rectangle
            2 => {
                println!("Entrez la longueur du rectangle :");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Erreur de lecture.");
                let longueur: f64 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Erreur, la valeur doit etre un nombre.");
                        continue;
                    }
                };
                println!("Entrez la largeur du rectangle :");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Erreur de lecture.");
                let largeur: f64 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Erreur, la valeur doit etre un nombre.");
                        continue;
                    }
                };
                surface = longueur * largeur;
            }

            // Cercle
            3 => {
                println!("Entrez le rayon du cercle :");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Erreur de lecture.");
                let rayon: f64 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Erreur, la valeur doit etre un nombre.");
                        continue;
                    }
                };
                surface = 3.14 * rayon * rayon;
            }

            // Sortie
            4 => {
                println!("Fin du programme");
                break;
            }

            // Défaut
            _ => {
                println!("Erreur, choix invalide !");
                continue;
            }
        }

        println!("La surface est : {:.2}", surface);
    }
}