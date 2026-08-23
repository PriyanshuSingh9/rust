# The Curated 5-Project Systems Stack (Elite Track)

> **Philosophy:** High Signal, Zero Redundancy.  
> Out of all possible projects, these 5 deliver the highest density of Rust systems learning while building the exact primitives dominating modern infrastructure (Cloudflare Durable Objects, Deno celld, Ingestion, Storage, and Concurrency).

```
┌──────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                 THE ELITE 5-PROJECT SYSTEMS STACK                                │
├──────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 1. arena        │ Typed Arena Allocator & Graph Memory Model (Lifetimes + Handles + Unsafe)     │
│ 2. stream-slice │ Zero-Copy Streaming Ingestion Engine (mmap + &[u8] + Peak RAM < 10MB)          │
│ 3. mini-git     │ Content-Addressed Object Store & Packfile Engine (Enums + Hashing + DAGs)      │
│ 4. mini-redis   │ High-Concurrency Async TCP Gateway & Actor Store (Tokio + RESP + Backpressure) │
│ 5. celld-rs     │ Distributed Durable Objects Engine (S3 CAS Leases + SQLite LTX + RPO=0)        │
└──────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## Architectural Progression by Systems Layer

```
                  Memory Layer        -->  1. arena
                  Ingestion Layer     -->  2. stream-slice
                  Storage Layer       -->  3. mini-git
                  Networking Layer    -->  4. mini-redis
                  Distributed State   -->  5. celld-rs (Signature Capstone)
```

---

## Detailed Project Specifications

### 1. `arena` - Typed Memory Arena & Graph Allocator
- **The Core Rust Lesson:** Lifetimes (`'a`), reference graphs, handles vs references (`NodeId(usize)`), `NonNull<T>`, safe encapsulation of `unsafe`.
- **Why It Matters:** Bypasses beginner reference struggles by teaching how production compilers, game engines, and network parsers manage memory locality and cyclic graphs without garbage collection.
- **Architecture:**
  ```text
  arena
  ├── Safe handle-based allocator (NodeId(usize) generational indexing)
  ├── Cyclic graph construction (parent/child trees without Rc/RefCell overhead)
  ├── Lifetime-bounded raw memory chunk arena
  └── Unsafe NonNull<T> block allocation with Miri UB verification
  ```
- **Deliverable:** A zero-fragmentation typed arena library benchmarked against standard heap allocation (`Box`/`Vec`).

---

### 2. `stream-slice` - Zero-Copy Streaming Ingestion Engine
- **The Core Rust Lesson:** Zero-copy slicing (`&str`, `&[u8]`), memory-mapped I/O (`memmap2`), allocation profiling, custom iterators.
- **Why It Matters:** Direct implementation of [Yapbook Feature P2 & P3](file:///home/bhondu/coding/projects/yapbook). Teaches the zero-allocation packet inspection patterns used in Cloudflare WAF and line-rate proxies.
- **Architecture:**
  ```text
  126k-message dataset / packet stream
                  │
                  ▼
          mmap / BufReader
                  │
                  ▼
                &[u8]
                  │
                  ├── zero-copy tokenizer
                  ├── parser combinator
                  └── borrowed message representation
                           │
                           ▼
                  single-pass O(N) metrics
  ```
- **Deliverable:** Parses 126k WhatsApp messages with zero heap allocations in the hot loop, maintaining peak RAM $< 10	ext{ MB}$.

---

### 3. `mini-git` - Content-Addressed Object Store & Packfile Engine
- **The Core Rust Lesson:** Algebraic data types, traits, binary serialization, filesystem DAG traversals, delta compression.
- **Why It Matters:** Foundational storage primitive for modern version control, Cursor Continuity, and Cloudflare Artifacts.
- **Architecture:**
  ```text
  mini-git
  ├── Object DAG (blobs, trees, commits, tags)
  ├── SHA-1 and SHA-256 content-addressing
  ├── Staging index parser (.git/index binary format)
  ├── Refs resolution (symbolic refs, HEAD, branches)
  ├── Tree diffing and checkout engine
  └── Packfile builder & delta-compression reader
  ```
- **Deliverable:** A functional Git plumbing CLI capable of committing, hashing, and producing valid packfiles readable by standard `git`.

---

### 4. `mini-redis` - High-Concurrency Async TCP Gateway & Actor Store
- **The Core Rust Lesson:** Concurrency, Tokio event loops, RESP wire framing, lock-striping, actor message channels.
- **Why It Matters:** Combines TCP proxying (Pingora style) with actor-based database isolation ([Yapbook Feature P4 SQLite Actor](file:///home/bhondu/coding/projects/yapbook)).
- **Architecture:**
  ```text
  TCP Connections --> Non-Blocking Tokio Listener
                           │
                           ▼
                  RESP Frame Decoder
                           │
                           ▼
          Striped RwLock State / mpsc Single Writer Actor
                           │
                           ▼
                 Response Serializer
  ```
- **Deliverable:** High-throughput async server supporting `GET`, `SET`, `DEL`, `EXPIRE`, `TTL`, `INCR`, pipelining, and connection rate limiting, sustaining 50k concurrent requests.

---

### 5. `celld-rs` - Distributed Durable Objects Engine (Signature Capstone)
- **The Core Rust Lesson:** Distributed state machines, Compare-and-Swap (CAS) lease coordination, sharded SQLite replication, Tokio RPC routing, sub-millisecond hibernation.
- **Why It Matters:** Direct implementation of the [Deno celld](https://celld.dev) and [Cloudflare Durable Objects](https://developers.cloudflare.com/durable-objects/) architecture - the premier stateful distributed systems primitive for edge infrastructure.

#### High-Impact Engineering Differentiators
To elevate `celld-rs` from a basic clone to a staff-grade systems project, the implementation incorporates five production innovations:

1. **Dual-Engine Runtime (WASI 0.2 WebAssembly + V8 JavaScript):**
   - Implements both a V8 isolate runtime for standard JS/TS Cloudflare Workers bundles and a `Wasmtime` WASI 0.2 component host.
   - Allows writing Durable Objects in **Rust, Go, Zig, or C++**, cutting cell wake latency to $< 1	ext{ ms}$ with sub-$200	ext{ KB}$ resident memory and deterministic instruction fuel metering.

2. **Deterministic Jepsen Chaos Simulation Suite (`turmoil`):**
   - Embeds a deterministic network simulation test harness using Tokio's `turmoil`.
   - Formally verifies S3 CAS lease fencing and LTX transaction replication under simulated split-brain partitions, bucket throttling, packet loss, and sudden `SIGKILL` power loss, proving $RPO=0$ and linearizable read consistency.

3. **In-Cell Vector Search & Semantic Memory (`sqlite-vec`):**
   - Embeds vector indexing (`sqlite-vec` extension or pure-Rust HNSW) directly inside each cell's SQLite instance.
   - Turns every Durable Object into an autonomous stateful AI agent with local transactional memory, vector embeddings, and S3-replicated state.

4. **Zero-Copy Intra-Node IPC (Lock-Free Ring Buffers):**
   - Employs `bytes::Bytes` and lock-free ring buffers for cross-cell messaging on the same physical node.
   - Routes intra-node messages in $< 5\ \mu	ext{s}$ via zero-copy in-memory pointers, avoiding loopback HTTP socket overhead.

5. **Group Commit Pipelining & S3 Express Acceleration:**
   - Batches multiple SQLite WAL page flushes into pipelined LTX segment PUTs on S3 Express One Zone / Cloudflare R2.
   - Slashes durable write acknowledgement latency from ~90ms down to sub-10ms.

#### Architectural Blueprint
```text
┌───────────────────────────────────────────────────────────────────────────────────┐
│                            CELLD-RS DISTRIBUTED ENGINE                            │
├───────────────────────────────────────────────────────────────────────────────────┤
│ 1. INGRESS & PROXY LAYER                                                          │
│    • Tokio async router mapping incoming requests to owning cell instances        │
│    • Zero-copy local IPC (< 5 µs) vs transparent HTTP/2 peer node forwarding      │
├───────────────────────────────────────────────────────────────────────────────────┤
│ 2. STATELESS WORKER NODES (~2,500 cells / 8 GB node)                              │
│    • Sharded SQLite: Each addressable object is an isolated SQLite database       │
│    • Dual Runtime: V8 (JS/TS) + WASI 0.2 Components (< 1 ms wake, < 200 KB RAM)  │
│    • In-Cell Vector Search: Embedded sqlite-vec for autonomous AI agent memory    │
│    • Epoch Fencing: Ownership is an atomic Compare-And-Swap (CAS) lease in S3     │
├───────────────────────────────────────────────────────────────────────────────────┤
│ 3. S3 BUCKET AS COORDINATOR (Zero Consensus Overhead)                             │
│    • Eliminates Raft/Paxos clusters: The bucket is the single source of truth     │
│    • Pipelined LTX Streaming: Continuous log shipping with RPO = 0 on SIGKILL     │
└───────────────────────────────────────────────────────────────────────────────────┘
```

#### Verification & Deliverables
- **Benchmark Suite:** Empirical latency distributions (p50, p99, p99.9) for cold wakes, resident reads, and durable writes.
- **Chaos Invariant Proof:** Automated Turmoil test run in CI asserting zero data loss across simulated split-brain and node crash scenarios.
- **Live Terminal TUI / Dashboard:** Real-time monitor using `ratatui` tracking active vs hibernated cells, lease renewal clocks, and LTX replication throughput.
