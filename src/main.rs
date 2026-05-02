// ============================================================
// PROJET : Réseaux et Protocoles - RP2026
// SUJET  : Démonstration du protocole Diffie-Hellman
// AUTEUR : KOUADIO KOUASSI HIPOLITE
// DATE   : Mai 2026
// ============================================================

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

// --- FONCTIONS UTILITAIRES ---

/// Affiche un message lettre par lettre pour l'effet "machine à écrire"
fn typewriter(text: &str, delay_ms: u64) {
    for c in text.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(delay_ms));
    }
    println!();
}

/// Pause entre les étapes
fn pause(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

/// Séparateur visuel
fn separateur() {
    println!("\n{}\n", "=".repeat(60));
}

/// Calcule (base ^ exposant) mod modulus — algorithme d'exponentiation rapide
/// C'est le coeur mathématique de Diffie-Hellman
fn puissance_mod(base: u64, exposant: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result: u64 = 1;
    let mut b = base % modulus;
    let mut e = exposant;
    while e > 0 {
        if e % 2 == 1 {
            result = result * b % modulus;
        }
        e /= 2;
        b = b * b % modulus;
    }
    result
}

/// Vérifie si un nombre est premier (test de primalité simple)
fn est_premier(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 { return false; }
        i += 2;
    }
    true
}

// --- SCÈNE 1 : Introduction ---
fn introduction() {
    separateur();
    typewriter("  DÉMONSTRATION DU PROTOCOLE DIFFIE-HELLMAN", 30);
    typewriter("  Projet Réseaux et Protocoles — RP2026", 25);
    typewriter("  Lycée Sainte-Marie de Cocody, Abidjan", 25);
    separateur();
    pause(800);

    typewriter("Imaginez trois personnes :", 20);
    pause(400);
    typewriter("  👤 LOUIS   — veut envoyer un message secret à Charles", 20);
    pause(400);
    typewriter("  👤 CHARLES — veut recevoir ce message secret", 20);
    pause(400);
    typewriter("  👀 ISMAËL  — écoute TOUT ce qui circule sur le réseau", 20);
    pause(600);

    typewriter("\nLe défi : Louis et Charles doivent se mettre d'accord", 20);
    typewriter("sur un SECRET COMMUN sans jamais l'envoyer sur le réseau.", 20);
    typewriter("Même Ismaël, qui écoute tout, ne doit pas pouvoir le trouver.", 20);
    pause(800);
}

// --- SCÈNE 2 : Paramètres publics ---
fn parametres_publics(p: u64, g: u64) {
    separateur();
    typewriter("ÉTAPE 1 — Choix des paramètres PUBLICS", 25);
    typewriter("(Tout le monde les connaît, même Ismaël)", 20);
    pause(500);

    println!("\n  p = {} (nombre premier — le modulus)", p);
    println!("  g = {} (la base — générateur)", g);
    pause(400);

    if est_premier(p) {
        typewriter("\n  ✅ p est bien un nombre premier.", 20);
    } else {
        typewriter("\n  ⚠️  Attention : p n'est PAS premier ! (démonstration uniquement)", 20);
    }
    pause(600);
}

// --- SCÈNE 3 : Clés privées ---
fn cles_privees(a: u64, b: u64) {
    separateur();
    typewriter("ÉTAPE 2 — Chaque personne choisit une clé PRIVÉE secrète", 25);
    typewriter("(Jamais partagée, jamais envoyée sur le réseau !)", 20);
    pause(500);

    println!("\n  🔒 Clé privée de LOUIS   : a = {}", a);
    println!("  🔒 Clé privée de CHARLES : b = {}", b);
    pause(400);
    typewriter("\n  👀 Ismaël ne voit rien de tout ça.", 20);
    pause(600);
}

// --- SCÈNE 4 : Clés publiques ---
fn cles_publiques(p: u64, g: u64, a: u64, b: u64) -> (u64, u64) {
    separateur();
    typewriter("ÉTAPE 3 — Calcul et échange des clés PUBLIQUES", 25);
    typewriter("(Envoyées sur le réseau — Ismaël peut les voir !)", 20);
    pause(500);

    let a_pub = puissance_mod(g, a, p);
    let b_pub = puissance_mod(g, b, p);

    println!("\n  LOUIS calcule   : A = g^a mod p = {}^{} mod {} = {}", g, a, p, a_pub);
    pause(400);
    println!("  CHARLES calcule : B = g^b mod p = {}^{} mod {} = {}", g, b, p, b_pub);
    pause(400);

    typewriter("\n  📡 Louis envoie A à Charles. Charles envoie B à Louis.", 20);
    typewriter("  👀 Ismaël voit : A et B. Mais il ne connaît pas a ni b !", 20);
    pause(600);

    (a_pub, b_pub)
}

// --- SCÈNE 5 : Secret partagé ---
fn secret_partage(p: u64, a: u64, b: u64, a_pub: u64, b_pub: u64) {
    separateur();
    typewriter("ÉTAPE 4 — Calcul du SECRET PARTAGÉ", 25);
    pause(500);

    let secret_louis   = puissance_mod(b_pub, a, p);
    let secret_charles = puissance_mod(a_pub, b, p);

    println!("\n  LOUIS calcule   : S = B^a mod p = {}^{} mod {} = {}", b_pub, a, p, secret_louis);
    pause(400);
    println!("  CHARLES calcule : S = A^b mod p = {}^{} mod {} = {}", a_pub, b, p, secret_charles);
    pause(600);

    if secret_louis == secret_charles {
        typewriter("\n  ✅ Les deux secrets sont IDENTIQUES !", 20);
        println!("  🔑 SECRET PARTAGÉ = {}", secret_louis);
        pause(400);
        typewriter("  Louis et Charles ont un secret commun sans jamais l'avoir envoyé !", 20);
    } else {
        typewriter("\n  ❌ Erreur dans le calcul.", 20);
    }
    pause(800);
}

// --- SCÈNE 6 : Attaque d'Ismaël ---
fn attaque_ismael(p: u64, g: u64, a_pub: u64, b_pub: u64, secret: u64) {
    separateur();
    typewriter("ÉTAPE 5 — Ismaël essaie de trouver le secret !", 25);
    typewriter("Il connaît : p, g, A (clé pub Louis), B (clé pub Charles)", 20);
    pause(500);

    typewriter("\nIsmaël doit résoudre : g^x mod p = A  (logarithme discret)", 20);
    typewriter("Il essaie toutes les valeurs possibles de x...", 20);
    pause(400);

    let mut trouve = false;
    for x in 1..p {
        if puissance_mod(g, x, p) == a_pub {
            println!("\n  👀 Ismaël trouve a = {} après {} essais", x, x);
            let secret_ismael = puissance_mod(b_pub, x, p);
            println!("  👀 Ismaël calcule le secret = {}", secret_ismael);
            if secret_ismael == secret {
                typewriter("  ⚠️  Ismaël a RÉUSSI à trouver le secret !", 20);
                typewriter("  → Avec un PETIT nombre premier, c'est facile !", 20);
            }
            trouve = true;
            break;
        }
    }
    if !trouve {
        typewriter("  Ismaël n'a pas trouvé dans la plage testée.", 20);
    }
    pause(800);
}

// --- SCÈNE 7 : Réponse à Bobo ---
fn reponse_bobo() {
    separateur();
    typewriter("RÉPONSE À LA QUESTION DE BOBO :", 30);
    pause(300);
    typewriter("« Quelle est la frontière objective de non-efficacité", 22);
    typewriter("  du protocole Diffie-Hellman face à Ismaël,", 22);
    typewriter("  quant au choix des nombres premiers primordiaux ? »", 22);
    separateur();
    pause(600);

    typewriter("DÉMONSTRATION COMPARATIVE :", 25);
    pause(400);

    // Test avec petit nombre premier
    println!("\n  --- CAS 1 : Petit nombre premier (p = 23) ---");
    let p_petit: u64 = 23;
    let g: u64 = 5;
    let a: u64 = 6;
    let b: u64 = 15;
    let a_pub_p = puissance_mod(g, a, p_petit);
    let b_pub_p = puissance_mod(g, b, p_petit);
    let secret_p = puissance_mod(b_pub_p, a, p_petit);

    println!("  p={}, g={}, a={}, b={}", p_petit, g, a, b);
    println!("  Clé publique Louis A = {}", a_pub_p);
    println!("  Secret partagé S = {}", secret_p);

    // Ismaël attaque
    let debut = std::time::Instant::now();
    for x in 1..p_petit {
        if puissance_mod(g, x, p_petit) == a_pub_p {
            println!("  👀 Ismaël casse en {} essais — temps : {:?}", x, debut.elapsed());
            break;
        }
    }
    pause(600);

    // Test avec grand nombre premier
    println!("\n  --- CAS 2 : Grand nombre premier (p = 7919) ---");
    let p_grand: u64 = 7919;
    let g2: u64 = 5;
    let a2: u64 = 300;
    let b2: u64 = 450;
    let a_pub_g = puissance_mod(g2, a2, p_grand);
    let b_pub_g = puissance_mod(g2, b2, p_grand);
    let secret_g = puissance_mod(b_pub_g, a2, p_grand);

    println!("  p={}, g={}, a={}, b={}", p_grand, g2, a2, b2);
    println!("  Clé publique Louis A = {}", a_pub_g);
    println!("  Secret partagé S = {}", secret_g);

    let debut2 = std::time::Instant::now();
    let mut essais: u64 = 0;
    let mut casse = false;
    for x in 1..p_grand {
        essais += 1;
        if puissance_mod(g2, x, p_grand) == a_pub_g {
            println!("  👀 Ismaël casse en {} essais — temps : {:?}", essais, debut2.elapsed());
            casse = true;
            break;
        }
    }
    if !casse {
        println!("  ✅ Ismaël n'a pas pu casser en {} essais !", essais);
    }
    pause(600);

    separateur();
    typewriter("CONCLUSION POUR BOBO :", 25);
    pause(300);
    typewriter("La frontière d'efficacité dépend de la taille de p :", 20);
    pause(300);
    println!();
    println!("  • p < 100       → ❌ Très dangereux  (Ismaël casse en millisecondes)");
    println!("  • p ~ 1 000     → ⚠️  Insuffisant     (quelques secondes)");
    println!("  • p ~ 1 000 000 → 🟡 Moyen           (minutes à heures)");
    println!("  • p > 2^2048    → ✅ Sécurisé        (millions d'années)");
    pause(400);
    println!();
    typewriter("  En pratique (SSH, VPN) : p fait 2048 à 4096 bits !", 20);
    typewriter("  C'est pour ça que le protocole SSH est un 'miracle mathématique'.", 20);
    pause(800);
}

// --- MAIN ---
fn main() {
    // Paramètres de la démo principale
    let p: u64 = 23;   // nombre premier public
    let g: u64 = 5;    // générateur public
    let a: u64 = 6;    // clé privée de Louis
    let b: u64 = 15;   // clé privée de Charles

    introduction();
    parametres_publics(p, g);
    cles_privees(a, b);
    let (a_pub, b_pub) = cles_publiques(p, g, a, b);
    secret_partage(p, a, b, a_pub, b_pub);

    let secret = puissance_mod(b_pub, a, p);
    attaque_ismael(p, g, a_pub, b_pub, secret);
    reponse_bobo();

    separateur();
    typewriter("  FIN DE LA DÉMONSTRATION — Projet RP2026", 25);
    typewriter("  Merci pour votre attention !", 25);
    separateur();
}