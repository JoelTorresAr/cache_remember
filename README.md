# CACHE REMEMBER: README.md

## [1.0.0] - 2020-01-01

Cache Remember is a simple caching library for rust that allows you to cache the result of a function call for a given amount of time.
Inspired in laravel's cache remember.


## EXAMPLE

```rust
    use cache_remember::CacheRemember;
    use std::thread;
    use std::time::Duration;
    use serde::{Deserialize, Serialize};
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

    fn main() {
            // Create a new cache instance
            let cache = Cache::new();
            // Get the result of the function call
            let fun = get_user();
            let hours : u64 = 1;
            let result = cache.remember("test_remember", hours, fun).await.unwrap();
            println!("{:?}", result);

            //forget the cache
            cache.forget("test_remember");

            // remember forever
            let fun = get_user();
            let result = cache.remember_forever("test_remember", fun).await.unwrap();
            println!("{:?}", result);

            //forget all cache
            cache.forget_all();
            println!("{:?}", cache);
    }
```