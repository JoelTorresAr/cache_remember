# CACHE REMEMBER: README.md
**Deprecated Library Notice**

⚠️ **Attention**: This library is deprecated! ⚠️

This is a friendly reminder that the library you are currently using has been marked as deprecated. We strongly recommend transitioning to the stable version of `cacheapp` instead. You can find the stable version at [https://github.com/JoelTorresAr/cacheapp.git](https://github.com/JoelTorresAr/cacheapp.git).

It is important to update your dependencies to ensure compatibility, security, and access to the latest features. By migrating to the stable `cacheapp` version, you will benefit from a more reliable and well-maintained solution.

Please note that the deprecated library might still be functional for the time being, but it will no longer receive updates or support. It is in your best interest to make the switch as soon as possible to avoid potential issues down the line.

Thank you for your understanding and cooperation in keeping your codebase up to date. If you have any questions or need assistance during the migration process, feel free to reach out to the `cacheapp` community or the library's maintainers.

Happy coding with `cacheapp`! 🚀

Best regards,

[Joel Torres]"




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
Changed Box dyn std::error::Error so that it now accepts functions that return any Error as long as it has the Display trait implemented.
Change Mutex to RwLock to allow multiple reads at the same time.
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