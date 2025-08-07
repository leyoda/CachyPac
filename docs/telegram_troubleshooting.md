# 🔧 Guide de Résolution - "Telegram non configuré"

## 🚨 Problème Identifié

Vous voyez le message **"Telegram non configuré"** dans CachyPac ? Ce guide vous aide à résoudre le problème étape par étape.

---

## 🔍 Diagnostic Rapide

### Étape 1: Vérifiez l'Interface CachyPac

1. **Lancez CachyPac** :
   ```bash
   cargo run
   ```

2. **Allez dans l'onglet "⚙️ Paramètres"**

3. **Cliquez sur "🔍 Diagnostic Telegram"** pour voir le diagnostic détaillé

### Étape 2: Analysez le Message d'Erreur

CachyPac affiche maintenant des messages d'erreur précis :

- ❌ **"Token Telegram manquant"** → Vous devez configurer le token bot
- ❌ **"Chat ID Telegram manquant"** → Vous devez configurer le chat ID  
- ❌ **"Telegram désactivé dans la configuration"** → Activez Telegram dans config.toml
- ❌ **"Configuration Telegram invalide"** → Format du token ou chat_id incorrect

---

## ✅ Solutions par Problème

### 🤖 Problème 1: Token Bot Manquant

**Symptôme :** "Token Telegram manquant"

**Solution :**
1. Créez un bot Telegram via [@BotFather](https://t.me/BotFather)
2. Envoyez `/newbot` et suivez les instructions
3. Copiez le token (format: `123456789:ABC-DEF1234ghIkl-zyx57W2v1u123ew11`)
4. Dans CachyPac → Paramètres → Collez le token dans "Token Telegram"
5. Cliquez "💾 Sauvegarder"

### 💬 Problème 2: Chat ID Manquant

**Symptôme :** "Chat ID Telegram manquant"

**Solution :**
1. **Pour un chat privé :**
   - Envoyez un message à [@userinfobot](https://t.me/userinfobot)
   - Copiez votre ID (ex: `123456789`)

2. **Pour un groupe :**
   - Ajoutez [@userinfobot](https://t.me/userinfobot) au groupe
   - Tapez `/start` dans le groupe
   - Copiez l'ID du groupe (ex: `-123456789`)

3. Dans CachyPac → Paramètres → Collez l'ID dans "Chat ID Telegram"
4. Cliquez "💾 Sauvegarder"

### ⚙️ Problème 3: Telegram Désactivé

**Symptôme :** "Telegram désactivé dans la configuration"

**Solution :**
1. Ouvrez le fichier `config.toml`
2. Trouvez la section `[telegram]`
3. Changez `enabled = false` en `enabled = true`
4. Sauvegardez le fichier
5. Redémarrez CachyPac

### 🔧 Problème 4: Format Invalide

**Symptôme :** "Configuration Telegram invalide"

**Solutions :**

**Token invalide :**
- ✅ Format correct : `123456789:ABC-DEF1234ghIkl-zyx57W2v1u123ew11`
- ❌ Format incorrect : `ABC123` ou `token_invalide`
- Le token doit contenir `:` et faire plus de 45 caractères

**Chat ID invalide :**
- ✅ Formats corrects : `123456789`, `-123456789`, `@username`
- ❌ Formats incorrects : `chat123`, `user_name`, `#channel`

---

## 🧪 Test de Configuration

### Test Rapide dans l'Interface

1. Allez dans **Paramètres**
2. Cliquez **"📱 Tester Telegram"**
3. Vérifiez le message de statut :
   - ✅ **"Test Telegram réussi - Message envoyé!"** → Configuration OK
   - ❌ **Erreur spécifique** → Suivez les instructions d'erreur

### Test Avancé avec l'Exemple

Pour un diagnostic complet, utilisez l'exemple dédié :

```bash
# Test avec configuration par défaut
cargo run --example telegram_diagnostic

# Test avec vos vraies credentials
TELEGRAM_BOT_TOKEN="votre_token" TELEGRAM_CHAT_ID="votre_chat_id" \
cargo run --example telegram_diagnostic
```

Cet exemple teste :
- ✅ Connectivité Internet
- ✅ Accès API Telegram  
- ✅ Authentification du bot
- ✅ Permissions du chat
- ✅ Validation des messages
- ✅ Rate limiting

---

## 📋 Configuration Complète

### Fichier config.toml

Exemple de configuration Telegram complète :

```toml
[telegram]
enabled = true
bot_token = "123456789:ABC-DEF1234ghIkl-zyx57W2v1u123ew11"
chat_id = "123456789"
notify_on_updates = true
notify_on_errors = true
notify_on_success = true
message_format = "html"
```

### Variables d'Environnement (Alternative)

```bash
export TELEGRAM_BOT_TOKEN="123456789:ABC-DEF1234ghIkl-zyx57W2v1u123ew11"
export TELEGRAM_CHAT_ID="123456789"
```

---

## 🔍 Diagnostic Avancé

### Logs de Debug

Activez les logs détaillés :

```bash
RUST_LOG=debug cargo run
```

Recherchez les messages :
- `❌ Erreur création notificateur Telegram`
- `❌ Test Telegram impossible`
- `✅ Test Telegram réussi`

### Vérification Manuelle

Testez votre configuration manuellement :

```bash
curl -X POST "https://api.telegram.org/bot<TOKEN>/sendMessage" \
     -H "Content-Type: application/json" \
     -d '{"chat_id": "<CHAT_ID>", "text": "Test CachyPac"}'
```

Remplacez `<TOKEN>` et `<CHAT_ID>` par vos valeurs.

---

## 🆘 Problèmes Persistants

Si le problème persiste après avoir suivi ce guide :

### 1. Vérifiez les Permissions du Bot

- Le bot doit pouvoir envoyer des messages dans le chat
- Pour les groupes : ajoutez le bot comme administrateur
- Pour les canaux : le bot doit être administrateur

### 2. Testez avec un Nouveau Bot

1. Créez un nouveau bot via [@BotFather](https://t.me/BotFather)
2. Utilisez le nouveau token
3. Testez dans un chat privé d'abord

### 3. Vérifiez la Connectivité

```bash
# Test de connectivité API Telegram
curl -I https://api.telegram.org
```

### 4. Consultez les Logs Complets

```bash
RUST_LOG=trace cargo run 2>&1 | grep -i telegram
```

---

## 📞 Support

Si vous avez encore des problèmes :

1. **Consultez les logs** avec `RUST_LOG=debug`
2. **Lancez le diagnostic complet** : `cargo run --example telegram_diagnostic`
3. **Vérifiez la documentation** : [`docs/telegram_diagnostic_guide.md`](telegram_diagnostic_guide.md)
4. **Ouvrez une issue** sur GitHub avec les logs d'erreur

---

## ✅ Résumé des Étapes

1. ✅ **Créer un bot** via [@BotFather](https://t.me/BotFather)
2. ✅ **Obtenir le chat ID** via [@userinfobot](https://t.me/userinfobot)
3. ✅ **Configurer dans CachyPac** → Paramètres
4. ✅ **Activer Telegram** dans config.toml (`enabled = true`)
5. ✅ **Tester** avec le bouton "📱 Tester Telegram"
6. ✅ **Diagnostic** avec le bouton "🔍 Diagnostic Telegram"

**Le message "Telegram non configuré" devrait maintenant être résolu !** 🎉