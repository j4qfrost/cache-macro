use cache_macro::cache;
use lru::LruCache;

#[test]
fn multiple_caches() {
    use std::f64;
    #[cache(LruCache : LruCache::new(20))]
    fn cached_sqrt(x: u64) -> f64 {
        f64::sqrt(x as f64)
    }
    #[cache(LruCache : LruCache::new(20))]
    fn cached_log(x: u64) -> f64 {
        f64::ln(x as f64)
    }

    assert_eq!(cached_sqrt(9), f64::sqrt(9.0));
    assert_eq!(cached_log(9), f64::ln(9.0));
}
