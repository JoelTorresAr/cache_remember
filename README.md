# CACHE REMEMBER: README.md

| Resource          | Link                                                                                                                              |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------------|
| Crate version     | [![Crates.io](https://img.shields.io/crates/v/wkhtmlapp?color=warning&style=plastic)](https://crates.io/crates/cache_remember)    |
| Documentation     | [Cargo docs](https://github.com/JoelTorresAr/cache_remember.git)                                                                  |



## [0.1.0] - 2023-06-11
Cache Remember is a simple caching library for rust that allows you to cache the result of a function call for a given amount of time.
Inspired in laravel's cache remember.

The remember function uses an async function as one of its parameters, which function must return a value that has Deserialize implemented, 
Serialize for serde. If it has a cached value, it returns the value without executing the function, otherwise it will execute the function 
and store the result in cache for future queries.

## [0.1.2] - 2023-06-11
REMOVE INNECESARY ASYNC AND RETURN RESULT IN forget(), forget_all() and purge() functions.

## [0.1.3] - 2023-06-11
The function that was passed as a parameter was changed to be strictly Result<T, Box<dyn std::error::Error>>, 
now it accepts functions that return an Error that has the Display trait implemented
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

            //purge expired records in cache
            cache.purge();
    }
```