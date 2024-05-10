// Exercice 2.2
// Utilisation du module std::io pour traiter les input et les output.
use std::io;

// Fonction via méthode loop.
pub fn calculer_pages_loop(pages_par_chapitre: u32, chapitres: u32) -> u32 {
    let mut total_pages = 0;
    for _ in 0..chapitres {
        total_pages += pages_par_chapitre;
    }
    total_pages
}

// Fonction via méthode récursive.
pub fn calculer_pages_recursive(pages_par_chapitre: u32, chapitres: u32) -> u32 {
    match chapitres {
        0 => 0,
        _ => pages_par_chapitre + calculer_pages_recursive(pages_par_chapitre, chapitres - 1),
    }
}

// Fonction principale.
pub fn main() {
    let mut pages_par_chapitre = String::new();
    let mut chapitres = String::new();

    println!("Entrez le nombre de pages par chapitre:");
    io::stdin().read_line(&mut pages_par_chapitre).expect("Erreur");    // On gère les erreurs de saisie.
    let pages_par_chapitre: u32 = pages_par_chapitre.trim().parse().expect("Entrez un nombre valide !");    // On gère les erreurs de saisie.

    println!("Entrez le nombre de chapitres:");
    io::stdin().read_line(&mut chapitres).expect("Erreur");     // On gère les erreurs de saisie.
    let chapitres: u32 = chapitres.trim().parse().expect("Entrez un nombre valide !");  // On gère les erreurs de saisie.

    let total_pages_loop = calculer_pages_loop(pages_par_chapitre, chapitres);
    println!("Total de pages avec la methode loop: {}", total_pages_loop);

    let total_pages_recursive = calculer_pages_recursive(pages_par_chapitre, chapitres);
    println!("Total de pages avec la methode recursive: {}", total_pages_recursive);
}