# Quantum Exegol

**Environmental Cybersecurity Framework** - Une alternative Rust à Exegol pour les opérations de sécurité offensive.

![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)
![License](https://img.shields.io/badge/License-GPLv3-blue.svg)

## À propos

Quantum Exegol est un framework de cybersécurité modulaire développé en Rust, conçu comme une alternative moderne à Exegol. Il permet de gérer des environnements de penetration testing, red team et CTF via des conteneurs Docker.

## Fonctionnalités

- 🔧 **Installation d'images** - Télécharge et configure des environnements de sécurité
- 🚀 **Gestion de conteneurs** - Démarre, arrête et exécute des commandes dans les conteneurs
- 📦 **Gestion d'images** - Liste, construit et supprime des images Docker
- ⚙️ **Configuration flexible** - Personnalise le comportement selon tes besoins
- 🔄 **Mise à jour** - Met à jour les images et le wrapper automatiquement
- 🔐 **Cryptographie intégrée** - Utilise les bibliothèques du projet RUST MATH EXPERIMENTAL

## Installation

```bash
# Clonez le projet
cd APPLICATION RUST QUANTIQUE/QUANTUM_EXEGOL

# Compilez le projet
cargo build --release

# Lancez l'aide
cargo run -- --help
```

## Utilisation

```bash
# Voir la version
quantum-exegol version

# Installer une image
quantum-exegol install --name quantum/security --tag latest

# Démarrer un conteneur
quantum-exegol start --name mon-container

# Lister les images
quantum-exegol images

# Lister les conteneurs
quantum-exegol ps

# Exécuter une commande
quantum-exegol exec --name mon-container nmap -sV 192.168.1.1

# Arrêter un conteneur
quantum-exegol stop --name mon-container

# Mettre à jour
quantum-exegol update
```

## Architecture

```
QUANTUM_EXEGOL/
├── src/
│   ├── main.rs       # Point d'entrée CLI
│   ├── cli.rs        # Gestion des commandes
│   ├── container.rs  # Gestion des conteneurs
│   ├── image.rs      # Gestion des images
│   ├── config.rs     # Configuration
│   ├── manager.rs    # Gestionnaire central
│   └── utils.rs      # Utilitaires
├── Cargo.toml        # Dépendances
└── README.md         # Documentation
```

## Intégration RUST MATH EXPERIMENTAL

Ce projet intègre les bibliothèques mathématiques et quantiques développées dans RUST MATH EXPERIMENTAL :

- **math_core** - Fonctions mathématiques avancées
- **quantum** - Bibliothèques de calcul quantique
- **crypto** - Outils cryptographiques
- **nanotech** - Simulation nanotechnology

## Technologies

- **Rust** - Langage de programmation
- **Docker** - Conteneurisation
- **bollard** - API Docker pour Rust
- **clap** - Parsing CLI
- **ndarray** - Calcul matriciel
- **num-complex** - Nombres complexes
- **rustfft** - Transformée de Fourier rapide

## License

GPLv3 - Voir LICENSE pour plus de détails.

## Contribution

Les contributions sont les bienvenues ! Veuillez lire CONTRIBUTING.md pour plus d'informations.

---

**Note** : Ce projet est une preuve de concept (POC) et une alternative académique à Exegol. Il n'est pas destiné à remplacer Exegol dans un environnement de production sans tests approfondis.
