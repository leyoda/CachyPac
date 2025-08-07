# Package AUR CachyRust

## Vue d'ensemble

CachyRust est maintenant prêt pour la publication sur l'Arch User Repository (AUR) avec un package complet et sans régression.

## Fichiers AUR

### PKGBUILD

Le fichier [`PKGBUILD`](../PKGBUILD) contient :

**Métadonnées :**
- **Nom** : `cachy-rust`
- **Version** : `0.1.0`
- **Description** : Bilingue FR/EN
- **Licence** : MIT
- **Architecture** : x86_64

**Dépendances :**
- **Runtime** : `pacman`, `sudo`, `gtk3`, `cairo`, `pango`, `gdk-pixbuf2`, `glib2`
- **Build** : `rust`, `cargo`, `git`, `imagemagick`
- **Optionnelles** : `telegram-desktop`, `systemd`

**Fonctionnalités :**
- Compilation optimisée avec `--release --all-features`
- Génération automatique d'icônes multi-tailles
- Tests unitaires intégrés
- Installation complète (binaires, services, configuration)

### .SRCINFO

Le fichier [`.SRCINFO`](../.SRCINFO) est généré automatiquement et contient les métadonnées parsées du PKGBUILD.

## Installation AUR

### Méthodes d'installation

**1. Avec un helper AUR (recommandé) :**
```bash
# Avec yay
yay -S cachy-rust

# Avec paru
paru -S cachy-rust

# Avec trizen
trizen -S cachy-rust
```

**2. Installation manuelle :**
```bash
git clone https://aur.archlinux.org/cachy-rust.git
cd cachy-rust
makepkg -si
```

### Vérification avant installation

```bash
# Vérifier le PKGBUILD
cat PKGBUILD

# Vérifier les checksums
makepkg --verifysource

# Build test sans installation
makepkg --noextract --syncdeps
```

## Structure d'installation

### Fichiers installés

**Binaires :**
- `/usr/bin/cachy-rust` - Exécutable principal
- `/usr/bin/cachy-rust-launcher` - Script de lancement (optionnel)
- `/usr/bin/cachy-rust-tray` - Gestion du system tray

**Configuration :**
- `/etc/cachy-rust/config.toml` - Configuration système par défaut
- `/etc/skel/.config/cachy-rust/` - Template utilisateur

**Services systemd :**
- `/usr/lib/systemd/user/cachy-rust-tray.service` - Service utilisateur

**Interface graphique :**
- `/usr/share/applications/cachy-rust.desktop` - Entrée de menu
- `/etc/xdg/autostart/cachy-rust-autostart.desktop` - Autostart (optionnel)

**Icônes :**
- `/usr/share/icons/hicolor/{16,32,48,64,128,256}x{16,32,48,64,128,256}/apps/cachy-rust.png`
- `/usr/share/pixmaps/cachy-rust.png` - Icône de fallback

**Documentation :**
- `/usr/share/doc/cachy-rust/README.md`
- `/usr/share/doc/cachy-rust/INTERNATIONALIZATION.md`
- `/usr/share/licenses/cachy-rust/LICENSE`

### Post-installation

Le package exécute automatiquement :
- Mise à jour du cache des icônes GTK
- Mise à jour de la base de données des applications
- Affichage des instructions de configuration

## Fonctionnalités du package

### Compilation optimisée

```bash
# Variables d'environnement
export RUSTUP_TOOLCHAIN=stable
export CARGO_TARGET_DIR=target

# Compilation avec toutes les fonctionnalités
cargo build --frozen --release --all-features
```

### Génération d'icônes

Si ImageMagick est disponible :
```bash
# Génération automatique depuis le SVG
for size in 16 32 48 64 128 256; do
    convert resources/cachy-rust.svg -resize ${size}x${size} icons/cachy-rust-${size}.png
done
```

### Tests intégrés

```bash
# Tests unitaires lors du build
cargo test --frozen --release
```

### Gestion des services

**Post-installation :**
- Instructions d'utilisation affichées
- Services systemd disponibles mais non activés
- Configuration sudo optionnelle

**Pré-suppression :**
- Arrêt automatique des services actifs
- Nettoyage des processus en cours

**Post-suppression :**
- Préservation des configurations utilisateur
- Instructions de nettoyage complet
- Mise à jour des caches système

## Configuration sudo optionnelle

Le package inclut un exemple de configuration sudo :

```bash
# Installation optionnelle pour éviter les mots de passe
sudo cp /usr/share/doc/cachy-rust/cachy-rust.sudoers.example /etc/sudoers.d/cachy-rust
sudo visudo -c  # Vérifier la syntaxe
```

Contenu du fichier :
```
# Optional: Allow cachy-rust to run pacman without password
# Uncomment the following line to enable:
# %wheel ALL=(ALL) NOPASSWD: /usr/bin/pacman
```

## Maintenance du package

### Mise à jour de version

1. **Modifier le PKGBUILD :**
```bash
pkgver=0.2.0
pkgrel=1
```

2. **Mettre à jour les checksums :**
```bash
updpkgsums
```

3. **Régénérer .SRCINFO :**
```bash
makepkg --printsrcinfo > .SRCINFO
```

4. **Tester la compilation :**
```bash
makepkg -f
```

### Validation du package

**Vérifications automatiques :**
- `namcap PKGBUILD` - Analyse du PKGBUILD
- `namcap *.pkg.tar.zst` - Analyse du package généré
- `makepkg --check` - Tests unitaires

**Tests manuels :**
- Installation dans un environnement propre
- Vérification des fonctionnalités principales
- Test de désinstallation

## Publication sur AUR

### Prérequis

1. **Compte AUR** : Créer un compte sur https://aur.archlinux.org
2. **Clé SSH** : Configurer l'authentification SSH
3. **Git** : Installer git pour la gestion des versions

### Processus de publication

**1. Clone initial :**
```bash
git clone ssh://aur@aur.archlinux.org/cachy-rust.git
cd cachy-rust
```

**2. Ajouter les fichiers :**
```bash
cp ../PKGBUILD .
cp ../.SRCINFO .
git add PKGBUILD .SRCINFO
```

**3. Commit et push :**
```bash
git commit -m "Initial import of cachy-rust v0.1.0"
git push origin master
```

### Mises à jour ultérieures

```bash
# Modifier PKGBUILD et .SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Update to v0.2.0 - Add new features"
git push origin master
```

## Avantages du package AUR

### Pour les utilisateurs

**Installation simplifiée :**
- Une seule commande avec les helpers AUR
- Gestion automatique des dépendances
- Intégration système complète

**Mises à jour automatiques :**
- Détection des nouvelles versions
- Mise à jour avec le système
- Préservation de la configuration

**Sécurité :**
- Code source vérifiable
- Compilation locale
- Pas de binaires pré-compilés

### Pour le projet

**Adoption communautaire :**
- Visibilité dans l'écosystème Arch
- Feedback de la communauté
- Contributions potentielles

**Distribution standardisée :**
- Installation cohérente
- Gestion des dépendances
- Intégration système

**Maintenance collaborative :**
- Possibilité de co-mainteneurs
- Signalement de bugs via AUR
- Amélioration continue

## Compatibilité et tests

### Distributions testées

- **Arch Linux** : Support principal
- **Manjaro** : Compatible
- **EndeavourOS** : Compatible
- **CachyOS** : Compatible (distribution cible)

### Environnements de bureau

- **KDE Plasma** : Support complet
- **GNOME** : Compatible
- **XFCE** : Compatible
- **i3/sway** : Compatible (mode minimal)

### Architectures

- **x86_64** : Support principal
- **aarch64** : Possible (non testé)

## Roadmap AUR

### Version 0.1.0 (Actuelle)
- [x] PKGBUILD complet
- [x] Installation système
- [x] Services systemd
- [x] Documentation

### Version 0.2.0 (Prévue)
- [ ] Support multi-architecture
- [ ] Package -git pour les développeurs
- [ ] Optimisations de compilation
- [ ] Tests d'intégration étendus

### Améliorations futures
- [ ] Package binaire pour les releases
- [ ] Support des distributions dérivées
- [ ] Intégration avec les outils de build automatisés
- [ ] Métriques d'utilisation (optionnelles)

## Conclusion

Le package AUR CachyRust est :

✅ **Production-ready** : Tests complets, installation robuste
✅ **Sans régression** : Préserve toutes les fonctionnalités existantes
✅ **Communautaire** : Prêt pour l'adoption par la communauté Arch
✅ **Maintenable** : Structure claire, documentation complète
✅ **Extensible** : Base solide pour les futures améliorations

Le package respecte toutes les bonnes pratiques AUR et peut être publié immédiatement pour permettre l'adoption communautaire de CachyRust.