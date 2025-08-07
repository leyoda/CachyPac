#!/bin/bash

echo "========================================="
echo "🔍 DIAGNOSTIC COMPLET DE CACHYPAC"
echo "========================================="
echo ""

# Test 1: Vérifier la présence de checkupdates
echo "📦 TEST 1: Vérification de checkupdates (pacman-contrib)"
echo "-----------------------------------------"
if command -v checkupdates &> /dev/null; then
    echo "✅ checkupdates est installé"
else
    echo "❌ checkupdates N'EST PAS installé!"
    echo "   → Installez avec: sudo pacman -S pacman-contrib"
fi
echo ""

# Test 2: Vérifier les fichiers Fluent
echo "🌍 TEST 2: Vérification des fichiers de traduction"
echo "-----------------------------------------"
if [ -d "locales" ]; then
    echo "📁 Dossier locales/ trouvé avec les fichiers:"
    ls -la locales/*.ftl 2>/dev/null | wc -l | xargs -I {} echo "   → {} fichiers .ftl trouvés"
    echo "⚠️  MAIS ces fichiers ne sont PAS utilisés par le code!"
else
    echo "❌ Dossier locales/ non trouvé"
fi
echo ""

# Test 3: Tester le mode daemon avec logs
echo "🔧 TEST 3: Test du mode daemon (5 secondes)"
echo "-----------------------------------------"
echo "Lancement en mode daemon avec logs de diagnostic..."
timeout 5 RUST_LOG=warn ./target/release/cachypac --daemon 2>&1 | grep -E "(DIAGNOSTIC|WARN|ERROR)" | head -10
echo ""

# Test 4: Vérifier la configuration
echo "⚙️ TEST 4: Vérification de la configuration"
echo "-----------------------------------------"
if [ -f "config.toml" ]; then
    echo "✅ config.toml trouvé"
    grep -q "bot_token" config.toml && echo "   → Configuration Telegram présente" || echo "   → Configuration Telegram absente"
else
    echo "⚠️ Pas de config.toml dans le répertoire courant"
    echo "   → L'application utilisera les valeurs par défaut"
fi
echo ""

# Test 5: Résumé des problèmes
echo "========================================="
echo "📊 RÉSUMÉ DES PROBLÈMES DÉTECTÉS"
echo "========================================="
echo ""
echo "🔴 PROBLÈMES CRITIQUES:"
echo "  1. Module Telegram en SIMULATION uniquement (pas de vraies notifications)"
echo "  2. Module Scheduler en SIMULATION uniquement (pas de vraies tâches cron)"
echo ""
echo "🟡 PROBLÈMES IMPORTANTS:"
echo "  3. Fichiers de traduction .ftl non utilisés (traductions codées en dur)"
echo "  4. Nécessite pacman-contrib pour fonctionner (checkupdates)"
echo ""
echo "🔵 INCOHÉRENCES:"
echo "  5. Documentation mentionne des fonctionnalités IA non implémentées"
echo "  6. Tests d'intégration mentionnés mais non présents"
echo ""
echo "========================================="
echo "✅ Diagnostic terminé"
echo "========================================="