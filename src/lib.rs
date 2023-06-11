mod err;
use err::{CacheErr, CacheResult};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct Cache {
    data: RwLock<HashMap<String, (Option<SystemTime>, String)>>,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            data: RwLock::new(HashMap::new()),
        }
    }

    pub fn get<T>(&self, key: &str) -> Result<Option<T>, Box<dyn std::error::Error>>
    where
        T: Clone + Serialize + for<'de> Deserialize<'de>,
    {
        let data = self.data.read();
        if let Some((_, result)) = data.get(key) {
            let result: T = serde_json::from_str(result)?;
            return Ok(Some(result));
        }
        Ok(None)
    }

    pub fn put<T>(&self, key: &str, value: T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Clone + Serialize + for<'de> Deserialize<'de>,
    {
        let mut data = self.data.write();
        let serialize = serde_json::to_string(&value)?;
        data.insert(key.to_string(), (None, serialize));
        Ok(())
    }

    pub async fn remember<F, T, E>(
        &self,
        key: &str,
        hours: u64,
        func: F,
    ) -> CacheResult<T>
    where
        T: Clone + Serialize + for<'de> Deserialize<'de>,
        F: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Display,
    {
        let data = self.data.read();
        if let Some((timestamp, result)) = data.get(key) {
            if let Some(timestamp) = timestamp {
                if SystemTime::now().duration_since(*timestamp)?
                    <= Duration::from_secs(hours * 60 * 60)
                {
                    let result: T = serde_json::from_str(result)?;
                    return Ok(result.clone());
                }
            } else {
                let result: T = serde_json::from_str(result)?;
                return Ok(result.clone());
            }
        }

        drop(data);

        let result = func.await.map_err(|e| CacheErr::ExternalError(e.to_string()))?;
        let mut data = self.data.write();
        let serialize = serde_json::to_string(&result)?;
        data.insert(key.to_string(), (Some(SystemTime::now()), serialize));

        Ok(result)
    }

    pub async fn remember_forever<F, T, E>(
        &self,
        key: &str,
        func: F,
    ) -> CacheResult<T>
    where
        T: Clone + Serialize + for<'de> Deserialize<'de>,
        F: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Display,
    {
        let data = self.data.read();
        if let Some((_, result)) = data.get(key) {
            let result: T = serde_json::from_str(result)?;
            return Ok(result.clone());
        }

        drop(data); // Liberar el Mutex antes de llamar a `func`

        let result = func.await.map_err(|e| CacheErr::ExternalError(e.to_string()))?;
        let mut data = self.data.write();
        let serialize = serde_json::to_string(&result)?;
        data.insert(key.to_string(), (None, serialize));

        Ok(result)
    }

    pub fn forget(&self, key: &str) {
        let mut data = self.data.write();
        data.remove(key);
    }

    pub fn forget_all(&self) {
        let mut data = self.data.write();
        data.clear();
    }

    pub fn purge(&self) {
        let mut data = self.data.write();
        let mut keys = Vec::new();
        for (key, (timestamp, _)) in data.iter() {
            if let Some(timestamp) = timestamp {
                if SystemTime::now().duration_since(*timestamp).is_ok() {
                    keys.push(key.clone());
                }
            }
        }
        for key in keys {
            data.remove(&key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::thread;
    use std::time::Duration;
    use tokio;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct User {
        pub name: String,
        pub email: String,
    }

    pub async fn get_user() -> Result<User, Box<dyn std::error::Error>> {
        thread::sleep(Duration::from_secs(1));
        Ok(User {
            name: "Joel Torres".to_string(),
            email: "djoel_torres@hotmail.com".to_string(),
        })
    }

    #[tokio::test]
    async fn test_remember() {
        let cache = Cache::new();
        let fun = get_user();
        let result = cache.remember("test_remember", 1, fun).await.unwrap();
        assert_eq!(result.name, "Joel Torres");
    }

    #[tokio::test]
    async fn test_remember_forever() {
        let cache = Cache::new();
        let fun = get_user();
        let result = cache
            .remember_forever("test_remember_forever", fun)
            .await
            .unwrap();
        assert_eq!(result.name, "Joel Torres");
    }

    #[tokio::test]
    async fn test_get() {
        let cache = Cache::new();
        let result = cache.get::<String>("test_get").unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_put() {
        let cache = Cache::new();
        cache.put("test_put", "Hello World".to_string()).unwrap();
        let result = cache.get::<String>("test_put").unwrap();
        assert_eq!(result.unwrap(), "Hello World");
    }

    #[tokio::test]
    async fn test_forget() {
        let cache = Cache::new();
        cache.put("test_forget", "Hello World".to_string()).unwrap();
        let result = cache.get::<String>("test_forget").unwrap();
        assert_eq!(result.unwrap(), "Hello World");

        cache.forget("test_forget");
        let result = cache.get::<String>("test_forget").unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_forget_all() {
        let cache = Cache::new();
        cache
            .put("test_forget_all", "Hello World".to_string())
            .unwrap();
        let result = cache.get::<String>("test_forget_all").unwrap();
        assert_eq!(result.unwrap(), "Hello World");

        cache.forget_all();
        let result = cache.get::<String>("test_forget_all").unwrap();
        assert_eq!(result, None);
    }
}
