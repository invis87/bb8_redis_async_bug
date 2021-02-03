# bb8-redis bug reproduction

---

**No longer reproducible!**

---

When open a lot of connections to service that used bb8-pool for redis it stop responding.

## Steps to reproduce

1. start redis server: `docker run -d --rm -p 6379:6379 --name=redis_server redis redis-server --appendonly yes`
2. start grpc server that use bb8-pool under the hood: `cargo run --bin pool_server`
3. start sending 200 requests with one connection: `cargo run --bin client_ok`
4. start sending 200 requests and create connection for each one: `cargo run --bin client_stuck`

After **4** step server stucked on ~90 request.

---

[link to issue](https://github.com/djc/bb8/issues/95)
