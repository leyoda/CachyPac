# CachyPac - Package AUR

Ce répertoire contient les fichiers nécessaires pour créer et maintenir le package AUR de CachyPac.

## Installation via AUR

### Avec un AUR Helper

```bash
# Avec yay
yay -S cachypac

# Avec paru
paru -S cachypac

# Avec trizen
trizen -S cachypac
```

### Installation Manuelle

```bash
# Cloner le repository AUR
git clone https://aur.archlinux.org/cachypac.git
cd cachypac

# Vérifier le PKGBUILD
cat PKGBUILD

# Compiler et installer
makepkg -si
```

## Post-Installation

Après l'installation, le script de configuration automatique sera exécuté. Vous pouvez également l'exécuter manuellement :

```bash
sudo /usr/share/cachypac/post-install.sh
```

### Configuration

1. **Éditer la configuration :**
```bash
sudo nano /etc/cachypac/config.toml
```

2. **Activer le service :**
```bash
sudo systemctl enable cachypac
sudo systemctl start cachypac
```

3. **Vérifier le statut :**
```bash
sudo systemctl status cachypac
```

## Fichiers Installés

### Binaires
- `/usr/bin/cachypac` - Exécutable principal

### Configuration
- `/etc/cachypac/config.toml` - Configuration système
- `/etc/cachypac/config.toml.example` - Exemple de configuration

### Service Systemd
- `/usr/lib/systemd/system/cachypac.service` - Service systemd

### Documentation
- `/usr/share/doc/cachypac/README.md` - Documentation principale
- `/usr/share/doc/cachypac/quick_start.md` - Guide de démarrage rapide
- `/usr/share/doc/cachypac/user_guide.md` - Guide utilisateur complet
- `/usr/share/doc/cachypac/api_documentation.md` - Documentation API
- `/usr/share/doc/cachypac/performance_optimizations.md` - Guide des performances

### Exemples
- `/usr/share/doc/cachypac/examples/simple_intelligence.rs` - Exemple d'intelligence artificielle

### Application Desktop
- `/usr/share/applications/cachypac.desktop` - Fichier desktop pour le menu

### Répertoires de Données
- `/var/lib/cachypac/` - Données de l'application
- `/var/log/cachypac/` - Logs de l'application

## Maintenance du Package

### Mise à Jour du Package

1. **Mettre à jour la version dans PKGBUILD :**
```bash
pkgver=0.2.0
```

2. **Mettre à jour les checksums :**
```bash
updpkgsums
```

3. **Générer .SRCINFO :**
```bash
makepkg --printsrcinfo > .SRCINFO
```

4. **Tester la compilation :**
```bash
makepkg -f
```

5. **Publier sur AUR :**
```bash
git add PKGBUILD .SRCINFO
git commit -m "Update to version 0.2.0"
git push
```

### Vérifications Avant Publication

- [ ] PKGBUILD suit les standards Arch Linux
- [ ] Toutes les dépendances sont correctes
- [ ] Le package se compile sans erreurs
- [ ] Les tests passent
- [ ] La documentation est à jour
- [ ] Le service systemd fonctionne
- [ ] Les permissions sont correctes

## Dépendances

### Dépendances de Compilation
- `rust` - Compilateur Rust
- `cargo` - Gestionnaire de paquets Rust
- `git` - Pour cloner les sources

### Dépendances d'Exécution
- `pacman` - Gestionnaire de paquets Arch Linux
- `systemd` - Système d'init et gestionnaire de services

### Dépendances Optionnelles
- `telegram-desktop` - Pour configurer les notifications Telegram
- `cronie` - Pour la planification avancée des tâches

## Sécurité

Le package AUR configure automatiquement :

- **Utilisateur système** : `cachypac` avec privilèges limités
- **Permissions sudo** : Accès restreint à `pacman` uniquement
- **Isolation systemd** : Service avec restrictions de sécurité
- **Répertoires protégés** : Permissions appropriées pour les données

## Support

- **Issues AUR** : Commentaires sur la page AUR
- **Issues Projet** : [GitHub Issues](https://github.com/leyoda/CachyPac/issues)
- **Documentation** : [Guide Utilisateur](https://github.com/leyoda/CachyPac/blob/main/docs/user_guide.md)

## Licence

Ce package est distribué sous licence Apache 2.0, identique au projet principal.