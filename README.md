# CACHE REMEMBER: README.md

## [1.0.0] - 2020-01-01

Cache Remember is a simple caching library for rust that allows you to cache the result of a function call for a given amount of time.
Inspired in laravel's cache remember.

The remember function uses an async function as one of its parameters, which function must return a value that has Deserialize implemented, 
Serialize for serde. If it has a cached value, it returns the value without executing the function, otherwise it will execute the function 
and store the result in cache for future queries.

| Resource          | Link                                                                                                                      |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------- |
| Crate             | [![Crates.io](https://img.shields.io/crates/v/wkhtmlapp?color=warning&style=plastic)]                                     |
| Documentation     | [Cargo docs](https://github.com/JoelTorresAr/cache_remember.git)                                                          |
| Cache remember Version | [cache_remember_0.1.0](https://github.com/wkhtmltopdf/packaging/releases)                                                   |

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