# 📋 REQUIREMENTS — Diffie-Hellman Demo RP2026

## Auteur
**KOUADIO KOUASSI HIPOLITE** — UFRMI, Abidjan

## Environnement de développement

| Outil | Version | Rôle |
|---|---|---|
| Rust | 1.95.0 | Langage de programmation principal |
| Cargo | 1.95.0 | Gestionnaire de paquets Rust |
| VSCode | Dernière version | Éditeur de code |
| rust-analyzer | Dernière version | Extension VSCode pour Rust |
| Git | Dernière version | Contrôle de version |

## Système
- **OS testé** : macOS (x86_64-apple-darwin)
- **Architecture** : x86_64

## Dépendances Rust externes
Aucune dépendance externe — la démo utilise uniquement la **bibliothèque standard Rust** (`std`) :

| Module std | Usage |
|---|---|
| `std::io` | Affichage terminal |
| `std::thread` | Pauses animées |
| `std::time` | Mesure du temps d'attaque |

## Installation complète
```bash
# 1. Installer Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Cloner le projet
git clone https://github.com/ichmochtegrosse-cell/diffie_hellman_demo.git
cd diffie_hellman_demo

# 3. Compiler et lancer
cargo run
```

## Dépôt GitHub
https://github.com/ichmochtegrosse-cell/diffie_hellman_demo

## Échéance
11 mai 2026 — soumission par e-mail avec objet [RP2026]