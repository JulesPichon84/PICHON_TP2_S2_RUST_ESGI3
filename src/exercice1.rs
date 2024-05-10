// Exercice 2.1
// Utilisation du module std::io pour traiter les input et les output.
use std::io;


pub fn afficher_genre_par_titre(titres: &Vec<(&str, &str)>, titre: &str) {
    match genre_par_titre(titres, titre) {
        Some(genre) => println!("Le genre du livre \"{}\" est : {}", titre, genre),
        None => println!("Le livre \"{}\" n'existe pas dans la liste.", titre),
    }
}

pub fn genre_par_titre<'a>(titres: &'a Vec<(&str, &str)>, titre: &str) -> Option<&'a str> {
    for (titre_livre, genre) in titres {
        if *titre_livre == titre {
            return Some(genre);
        }
    }
    None // Aucun genre trouvé pour le titre donné.
}

pub fn main() {
    // On initialise de la liste de livres.
    let livres = vec![
        ("L'Étranger", "Roman"),
        ("1984", "Science-fiction"),
        ("Le Petit Prince", "Conte"),
        ("Odyssée", "Poésie"),
        ("Gaston Lagaffe", "Bande Dessinée")
    ];

    // On affiche de la liste des livres disponibles.
    println!("Liste des livres disponibles :");
    for (titre, _) in &livres {
        println!("{}", titre);
    }

    // On demande à l'utilisateur de choisir un livre.
    println!("Veuillez choisir un livre parmi la liste ci-dessus :");

    // Lecture de l'entrée utilisateur pour le titre du livre.
    let mut choix_livre = String::new();
    io::stdin().read_line(&mut choix_livre)
        .expect("Erreur lors de la lecture de l'entrée utilisateur.");
    let choix_livre = choix_livre.trim(); // On supprime les éventuels espaces ou sauts de ligne.

    // On affiche le genre du livre choisi.
    afficher_genre_par_titre(&livres, choix_livre);
}