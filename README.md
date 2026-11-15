# Horloge Digitale Multiplateforme (Rust + eframe/egui)

Application graphique légère affichant l'heure en temps réel au format digital, avec l'heure locale et l'heure UTC en simultané.

## Fonctionnalités

- **Multiplateforme** : Fonctionne de manière native sur Linux (Wayland et X11), Windows et macOS.
- **Affichage simultané** :
  - **Heure Locale** : Heure précise, fuseau/décalage horaire (ex: UTC+01:00) et date complète.
  - **Heure UTC** : Temps Universel Coordonné et date UTC.
- **Options interactives** :
  - Bascule entre format 24h et format 12h (AM/PM).
  - Activation / désactivation de l'affichage des secondes.
  - Activation / désactivation de l'affichage de la date.
- **Thème sombre "digital"** : Polices monospace à haute lisibilité avec codes couleurs distincts (vert digital pour le local, cyan pour UTC).
- **Ressources légères** : Rafraîchissement ciblé toutes les 200ms pour garantir la fluidité des secondes sans saturer le CPU/GPU.

---

## Structure du projet

```
digital_clock/
├── Cargo.toml
├── README.md
└── src/
    └── main.rs
```

---

## Instructions de compilation et d'exécution

### 1. Prérequis système

#### Sur Linux (Debian / Ubuntu) :
```bash
sudo apt update
sudo apt install build-essential libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
```

#### Sur Linux (Arch Linux / Manjaro) :
```bash
sudo pacman -S base-devel libxkbcommon
```

#### Sur Windows / macOS :
Aucune dépendance système externe particulière requise (le compilateur Rust standard `rustc` et `cargo` suffisent).

---

### 2. Compilation et lancement en mode développement

Pour compiler et lancer immédiatement l'application :

```bash
cd digital_clock
cargo run
```

---

### 3. Compilation en mode Release (Production)

Pour produire un binaire optimisé et compact :

```bash
cargo build --release
```

Le binaire exécutable sera disponible dans :
- **Linux / macOS** : `target/release/digital_clock`
- **Windows** : `target/release/digital_clock.exe`
