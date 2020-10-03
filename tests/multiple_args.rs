use cache_macro::cache;
use lru::LruCache;

#[test]
fn multiple_args() {
    #[cache(LruCache : LruCache::new(20))]
    #[inline]
    fn ackermann(m: u64, n: u64) -> u64 {
        if m == 0 {
            n + 1
        } else if m > 0 && n == 0 {
            ackermann(m - 1, 1)
        } else {
            ackermann(m - 1, ackermann(m, n - 1))
        }
    }

    assert_eq!(ackermann(0, 0), 1);
    assert_eq!(ackermann(1, 0), 2);
    assert_eq!(ackermann(1, 1), 3);
    assert_eq!(ackermann(3, 2), 29);
}
