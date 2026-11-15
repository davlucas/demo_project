# Horloge Digitale Multiplateforme (Rust + eframe/egui + chrono-tz)

Application graphique haute performance en Rust affichant l'heure digitale en temps réel, avec l'heure locale, l'heure UTC, la sélection multi-fuseaux horaires et l'internationalisation complète.

## Fonctionnalités

- **Internationalisation (i18n) complète en 5 langues** :
  - 🇫🇷 **Français**
  - 🇬🇧 **English** (Anglais)
  - 🇯🇵 **日本語** (Japonais)
  - 🇨🇳 **中文** (Chinois)
  - 🇪🇸 **Español** (Espagnol)
  - Changement instantané de langue en direct via un menu déroulant.
  - La langue sélectionnée est automatiquement mémorisée dans le fichier de configuration.
  - **Support des glyphes CJK (Japonais / Chinois)** avec détection et chargement automatique des polices système multiplateformes (Linux Noto Sans CJK / WQY, Windows YaHei / Meiryo, macOS PingFang / Hiragino).

- **Multiplateforme** : Compatible nativement Linux (Wayland & X11), Windows et macOS (Binaire universel Apple Silicon + Intel).

- **Horloges de référence** :
  - **Heure Locale** (décalage UTC et date complète).
  - **Heure UTC** (Temps Universel Coordonné de référence).

- **Gestion dynamique de fuseaux horaires** :
  - **Tri hiérarchique par Continent** traduit dans la langue sélectionnée.
  - **Tri alphabétique des Pays / Villes** pour chaque continent.
  - Recherche textuelle rapide pour filtrer instantanément une ville.
  - Bouton d'ajout instantané à l'affichage principal.
  - Bouton de suppression (`✖`) pour chaque horloge personnalisée.

- **Persistance automatique (`USERDIR/.digital_clock`)** :
  - Les fuseaux et la langue sélectionnée sont sauvegardés automatiquement dans :
    - **Linux / macOS** : `$HOME/.digital_clock`
    - **Windows** : `%USERPROFILE%\.digital_clock`
  - Restaurés automatiquement au démarrage.

- **Contrainte stricte de dimensions d'écran (Multi-colonnes réactif)** :
  - Détection automatique de la résolution d'écran de l'utilisateur.
  - **Chaque colonne est strictement limitée en hauteur** (`max_height <= résolution écran`), empêchant tout dépassement vertical.
  - Répartition dynamique sur plusieurs colonnes avec défilement horizontal fluide si de nombreuses horloges sont ajoutées.

---

## Compilation et Lancement

### Lancement direct :
```bash
cd /home/david/digital_clock
cargo run --release
```

### Compilation du binaire de production :
```bash
cargo build --release
```
L'exécutable standalone est généré dans `target/release/digital_clock`.
