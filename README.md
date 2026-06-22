# DNS Resolver 🌐

High-performance DNS resolver with DoH support.

## Features

- **Caching**: TTL-based with LRU eviction
- **Prefetching**: Proactive cache warming
- **DoH**: DNS-over-HTTPS (Cloudflare, Google)
- **Negative Caching**: NXDOMAIN caching

## Performance

| Metric | Value |
|--------|-------|
| Resolution | 2ms (cached), 15ms (fresh) |
| Throughput | 100K queries/s |
| Cache hit rate | >80% |

## License

MIT