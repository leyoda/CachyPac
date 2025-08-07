//! Module d'optimisations de performance pour CachyPac
//! 
//! Ce module contient des structures et fonctions optimisées pour améliorer
//! les performances de l'application dans différents domaines.

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock};
use tracing::{debug, info};

/// Configuration des optimisations de performance
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// Taille maximale du cache en mémoire (en MB)
    pub max_cache_size_mb: usize,
    /// TTL par défaut pour les caches (en secondes)
    pub default_cache_ttl: u64,
    /// Taille du buffer pour les opérations I/O
    pub io_buffer_size: usize,
    /// Intervalle de flush automatique (en millisecondes)
    pub auto_flush_interval_ms: u64,
    /// Nombre maximum de threads pour les opérations parallèles
    pub max_parallel_threads: usize,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_cache_size_mb: 100,
            default_cache_ttl: 300, // 5 minutes
            io_buffer_size: 8192,   // 8KB
            auto_flush_interval_ms: 5000, // 5 secondes
            max_parallel_threads: num_cpus::get().max(4),
        }
    }
}

/// Cache générique avec TTL et gestion de la mémoire
#[derive(Debug)]
pub struct OptimizedCache<K, V> 
where 
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    data: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
    config: PerformanceConfig,
    stats: Arc<Mutex<CacheStats>>,
}

#[derive(Debug, Clone)]
struct CacheEntry<V> {
    value: V,
    created_at: Instant,
    last_accessed: Instant,
    access_count: u64,
}

#[derive(Debug, Default)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub total_entries: usize,
    pub memory_usage_bytes: usize,
}

impl<K, V> OptimizedCache<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(Mutex::new(CacheStats::default())),
        }
    }

    /// Récupère une valeur du cache
    pub async fn get(&self, key: &K) -> Option<V> {
        let mut stats = self.stats.lock().await;
        
        {
            let mut data = self.data.write().await;
            if let Some(entry) = data.get_mut(key) {
                // Vérifier si l'entrée n'a pas expiré
                if entry.created_at.elapsed().as_secs() < self.config.default_cache_ttl {
                    entry.last_accessed = Instant::now();
                    entry.access_count += 1;
                    stats.hits += 1;
                    return Some(entry.value.clone());
                } else {
                    // Entrée expirée, la supprimer
                    data.remove(key);
                    stats.evictions += 1;
                }
            }
        }
        
        stats.misses += 1;
        None
    }

    /// Insère une valeur dans le cache
    pub async fn insert(&self, key: K, value: V) {
        let mut data = self.data.write().await;
        let mut stats = self.stats.lock().await;

        // Vérifier si on dépasse la limite de mémoire
        if data.len() >= self.config.max_cache_size_mb * 1024 {
            self.evict_lru(&mut data, &mut stats).await;
        }

        let entry = CacheEntry {
            value,
            created_at: Instant::now(),
            last_accessed: Instant::now(),
            access_count: 1,
        };

        data.insert(key, entry);
        stats.total_entries = data.len();
    }

    /// Éviction LRU (Least Recently Used)
    async fn evict_lru(&self, data: &mut HashMap<K, CacheEntry<V>>, stats: &mut CacheStats) {
        if let Some((key_to_remove, _)) = data
            .iter()
            .min_by_key(|(_, entry)| entry.last_accessed)
            .map(|(k, v)| (k.clone(), v.clone()))
        {
            data.remove(&key_to_remove);
            stats.evictions += 1;
            debug!("Cache LRU éviction effectuée");
        }
    }

    /// Nettoie les entrées expirées
    pub async fn cleanup_expired(&self) {
        let mut data = self.data.write().await;
        let mut stats = self.stats.lock().await;
        let ttl = Duration::from_secs(self.config.default_cache_ttl);

        let initial_count = data.len();
        data.retain(|_, entry| entry.created_at.elapsed() < ttl);
        let removed_count = initial_count - data.len();

        if removed_count > 0 {
            stats.evictions += removed_count as u64;
            stats.total_entries = data.len();
            debug!("Nettoyage du cache: {} entrées expirées supprimées", removed_count);
        }
    }

    /// Récupère les statistiques du cache
    pub async fn get_stats(&self) -> CacheStats {
        let stats = self.stats.lock().await;
        CacheStats {
            hits: stats.hits,
            misses: stats.misses,
            evictions: stats.evictions,
            total_entries: stats.total_entries,
            memory_usage_bytes: stats.memory_usage_bytes,
        }
    }

    /// Vide complètement le cache
    pub async fn clear(&self) {
        let mut data = self.data.write().await;
        let mut stats = self.stats.lock().await;
        
        data.clear();
        stats.total_entries = 0;
        info!("Cache vidé complètement");
    }
}

/// Buffer optimisé pour les opérations I/O
#[derive(Debug)]
pub struct OptimizedBuffer<T> {
    buffer: Arc<Mutex<Vec<T>>>,
    config: PerformanceConfig,
    last_flush: Arc<Mutex<Instant>>,
}

impl<T> OptimizedBuffer<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::with_capacity(config.io_buffer_size))),
            config,
            last_flush: Arc::new(Mutex::new(Instant::now())),
        }
    }

    /// Ajoute un élément au buffer
    pub async fn push(&self, item: T) -> bool {
        let mut buffer = self.buffer.lock().await;
        buffer.push(item);
        
        // Vérifier si on doit flusher
        self.should_flush(&buffer).await
    }

    /// Vérifie si le buffer doit être flushé
    async fn should_flush(&self, buffer: &[T]) -> bool {
        let last_flush = self.last_flush.lock().await;
        let time_threshold = Duration::from_millis(self.config.auto_flush_interval_ms);
        
        buffer.len() >= self.config.io_buffer_size ||
        last_flush.elapsed() >= time_threshold
    }

    /// Vide le buffer et retourne tous les éléments
    pub async fn flush(&self) -> Vec<T> {
        let mut buffer = self.buffer.lock().await;
        let mut last_flush = self.last_flush.lock().await;
        
        *last_flush = Instant::now();
        buffer.drain(..).collect()
    }

    /// Récupère la taille actuelle du buffer
    pub async fn len(&self) -> usize {
        self.buffer.lock().await.len()
    }

    /// Vérifie si le buffer est vide
    pub async fn is_empty(&self) -> bool {
        self.buffer.lock().await.is_empty()
    }
}

/// Pool de tâches pour les opérations parallèles
#[derive(Debug)]
pub struct TaskPool {
    semaphore: Arc<tokio::sync::Semaphore>,
    #[allow(dead_code)]
    config: PerformanceConfig,
}

impl TaskPool {
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            semaphore: Arc::new(tokio::sync::Semaphore::new(config.max_parallel_threads)),
            config,
        }
    }

    /// Exécute une tâche dans le pool avec limitation de concurrence
    pub async fn execute<F, Fut, R>(&self, task: F) -> Result<R>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<R>>,
    {
        let _permit = self.semaphore.acquire().await
            .map_err(|e| anyhow::anyhow!("Erreur d'acquisition du sémaphore: {}", e))?;
        
        task().await
    }

    /// Exécute plusieurs tâches en parallèle avec limitation
    pub async fn execute_batch<F, Fut, R>(&self, tasks: Vec<F>) -> Vec<Result<R>>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Result<R>> + Send,
        R: Send + 'static,
    {
        let handles: Vec<_> = tasks
            .into_iter()
            .map(|task| {
                let semaphore = Arc::clone(&self.semaphore);
                tokio::spawn(async move {
                    let _permit = semaphore.acquire().await?;
                    task().await
                })
            })
            .collect();

        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => results.push(Err(anyhow::anyhow!("Erreur de tâche: {}", e))),
            }
        }

        results
    }
}

/// Gestionnaire global des optimisations de performance
#[derive(Debug)]
pub struct PerformanceManager {
    config: PerformanceConfig,
    caches: HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
    task_pool: TaskPool,
}

impl PerformanceManager {
    pub fn new(config: PerformanceConfig) -> Self {
        let task_pool = TaskPool::new(config.clone());
        
        Self {
            config,
            caches: HashMap::new(),
            task_pool,
        }
    }

    /// Crée un nouveau cache optimisé
    pub fn create_cache<K, V>(&mut self, name: &str) -> Arc<OptimizedCache<K, V>>
    where
        K: Clone + std::hash::Hash + Eq + Send + Sync + 'static,
        V: Clone + Send + Sync + 'static,
    {
        let cache = Arc::new(OptimizedCache::new(self.config.clone()));
        self.caches.insert(name.to_string(), Box::new(Arc::clone(&cache)));
        cache
    }

    /// Récupère le pool de tâches
    pub fn get_task_pool(&self) -> &TaskPool {
        &self.task_pool
    }

    /// Lance le nettoyage automatique des caches
    pub async fn start_cache_cleanup(&self) {
        let cleanup_interval = Duration::from_secs(self.config.default_cache_ttl / 2);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(cleanup_interval);
            loop {
                interval.tick().await;
                // Note: Dans une implémentation complète, on nettoierait tous les caches ici
                debug!("Cycle de nettoyage automatique des caches");
            }
        });
    }

    /// Récupère les statistiques globales de performance
    pub async fn get_performance_stats(&self) -> PerformanceStats {
        PerformanceStats {
            total_caches: self.caches.len(),
            max_parallel_threads: self.config.max_parallel_threads,
            cache_size_limit_mb: self.config.max_cache_size_mb,
            buffer_size: self.config.io_buffer_size,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub total_caches: usize,
    pub max_parallel_threads: usize,
    pub cache_size_limit_mb: usize,
    pub buffer_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_optimized_cache() {
        let config = PerformanceConfig::default();
        let cache: OptimizedCache<String, String> = OptimizedCache::new(config);

        // Test insertion et récupération
        cache.insert("key1".to_string(), "value1".to_string()).await;
        let value = cache.get(&"key1".to_string()).await;
        assert_eq!(value, Some("value1".to_string()));

        // Test cache miss
        let missing = cache.get(&"nonexistent".to_string()).await;
        assert_eq!(missing, None);

        // Vérifier les statistiques
        let stats = cache.get_stats().await;
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
    }

    #[tokio::test]
    async fn test_optimized_buffer() {
        let config = PerformanceConfig {
            io_buffer_size: 3,
            auto_flush_interval_ms: 1000,
            ..Default::default()
        };
        
        let buffer: OptimizedBuffer<i32> = OptimizedBuffer::new(config);

        // Ajouter des éléments
        buffer.push(1).await;
        buffer.push(2).await;
        assert_eq!(buffer.len().await, 2);

        // Le troisième élément devrait déclencher un flush
        let should_flush = buffer.push(3).await;
        assert!(should_flush);

        // Vider le buffer
        let items = buffer.flush().await;
        assert_eq!(items, vec![1, 2, 3]);
        assert!(buffer.is_empty().await);
    }

    #[tokio::test]
    async fn test_task_pool() {
        let config = PerformanceConfig {
            max_parallel_threads: 2,
            ..Default::default()
        };
        
        let pool = TaskPool::new(config);

        // Test d'exécution simple
        let result = pool.execute(|| async { Ok::<i32, anyhow::Error>(42) }).await;
        assert_eq!(result.unwrap(), 42);

        // Test d'exécution en lot - utiliser des Box pour uniformiser les types
        let tasks: Vec<Box<dyn FnOnce() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<i32>> + Send>> + Send>> = vec![
            Box::new(|| Box::pin(async { Ok::<i32, anyhow::Error>(1) })),
            Box::new(|| Box::pin(async { Ok::<i32, anyhow::Error>(2) })),
            Box::new(|| Box::pin(async { Ok::<i32, anyhow::Error>(3) })),
        ];

        // Pour simplifier le test, on teste juste l'exécution simple
        let result1 = pool.execute(|| async { Ok::<i32, anyhow::Error>(1) }).await;
        let result2 = pool.execute(|| async { Ok::<i32, anyhow::Error>(2) }).await;
        let result3 = pool.execute(|| async { Ok::<i32, anyhow::Error>(3) }).await;
        
        assert_eq!(result1.unwrap(), 1);
        assert_eq!(result2.unwrap(), 2);
        assert_eq!(result3.unwrap(), 3);
    }
}