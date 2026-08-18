# 🦀 Rust Mastery Curriculum: Milestone Projects & Portfolio Roadmap

> **Author / Mentor:** Staff Rust Systems Engineer  
> **Target Audience:** Engineers with foundational knowledge (Rust Book Chapters 1–3) seeking industry-grade systems mastery and production readiness.  
> **Scope:** 5 Progressive Stages, 13 Rigorous Projects, Architectural Blueprints, and Production Quality Standards.

---

## 🧭 Table of Contents

1. [Executive Summary & The Philosophy of Rust Mastery](#-executive-summary--the-philosophy-of-rust-mastery)
2. [Curriculum Architecture & Roadmap Matrix](#-curriculum-architecture--roadmap-matrix)
3. [Stage 1: Memory & Ownership Mastery (Chapters 4–9)](#-stage-1-memory--ownership-mastery)
   - [Project 1.1: `slice-tool` — High-Performance Zero-Copy Text Utility](#project-11-slice-tool--high-performance-zero-copy-text-utility)
   - [Project 1.2: `oxstore` — In-Memory Key-Value Store with Typed Values & TTL](#project-12-oxstore--in-memory-key-value-store-with-typed-values--ttl)
   - [Project 1.3: `custom-collections` — Vec & Singly Linked List with `Drop` & Iterators](#project-13-custom-collections--vec--singly-linked-list-with-drop--iterators)
4. [Stage 2: Traits, Generics, and System I/O (Chapters 10–14 + Ciulla 6–10)](#-stage-2-traits-generics-and-system-io)
   - [Project 2.1: `minigrep-pro` — Regex Engine, Buffered Streamer & Colored CLI](#project-21-minigrep-pro--regex-engine-buffered-streamer--colored-cli)
   - [Project 2.2: `micro-json` — Custom JSON Parser with Parser Combinators](#project-22-micro-json--custom-json-parser-with-parser-combinators)
   - [Project 2.3: `flow-engine` — Extensible Data Pipeline via Trait Objects (`dyn Trait`)](#project-23-flow-engine--extensible-data-pipeline-via-trait-objects-dyn-trait)
5. [Stage 3: Concurrency, Multithreading & Networking (Chapters 15–21 + Ciulla 11–13)](#-stage-3-concurrency-multithreading--networking)
   - [Project 3.1: `ferris-server` — Multithreaded TCP HTTP/1.1 Server with Threadpool](#project-31-ferris-server--multithreaded-tcp-http11-server-with-threadpool)
   - [Project 3.2: `spider-rs` — High-Throughput Concurrent Web Crawler](#project-32-spider-rs--high-throughput-concurrent-web-crawler)
   - [Project 3.3: `rusty-redis` — Redis Lite Engine with RESP Parser & Striped Locks](#project-33-rusty-redis--redis-lite-engine-with-resp-parser--striped-locks)
6. [Stage 4: Production Async Web Backend & Systems (Ciulla 14–16 + Zero to Production)](#-stage-4-production-async-web-backend--systems)
   - [Project 4.1: `nexus-api` — Enterprise Async Microservice (Axum + SQLx + JWT + Tracing)](#project-41-nexus-api--enterprise-async-microservice)
7. [Stage 5: Advanced & Capstone Mastery (Unsafe, Systems, Distributed)](#-stage-5-advanced--capstone-mastery)
   - [Capstone Option A: `raft-rs` — Distributed Consensus & Replicated KV Engine](#capstone-option-a-raft-rs--distributed-consensus--replicated-kv-engine)
   - [Capstone Option B: `pebble-db` — LSM-Tree & WAL Embedded Storage Engine](#capstone-option-b-pebble-db--lsm-tree--wal-embedded-storage-engine)
   - [Capstone Option C: `sys-observer` — Linux eBPF Kernel Observer & Telemetry Daemon](#capstone-option-c-sys-observer--linux-ebpf-kernel-observer--telemetry-daemon)
   - [Capstone Option D: `vanguard-vm` — Bytecode VM & WASM Web Emulator](#capstone-option-d-vanguard-vm--bytecode-vm--wasm-web-emulator)
8. [Staff Engineering Production Standards & Portfolio Checklist](#-staff-engineering-production-standards--portfolio-checklist)
9. [Appendix: Learning Resources, Books & Mentorship FAQ](#-appendix-learning-resources-books--mentorship-faq)

---

## 💡 Executive Summary & The Philosophy of Rust Mastery

Transitioning from beginner syntax to staff-level fluency in Rust requires abandoning dynamic/garbage-collected habits and adopting **data-oriented systems thinking**.

```
                           THE RUST MENTALITY SHIFT
┌──────────────────────────────────────────────┐    ┌──────────────────────────────────────────────┐
│       GARBAGE COLLECTED / DYNAMIC            │    │            RUST SYSTEMS PARADIGM             │
├──────────────────────────────────────────────┤    ├──────────────────────────────────────────────┤
│ • Everything is a pointer/reference          │    │ • Values are concrete, stack-first, sized    │
│ • Runtimes manage memory in background       │ -> │ • Lifetimes enforce single owners & aliasing │
│ • Exceptions throw arbitrarily at runtime    │    │ • Errors are explicit enums in return types  │
│ • Data races caught late in production       │    │ • "Fearless Concurrency" proven at compile   │
│ • Heavy heap allocations everywhere          │    │ • Zero-cost abstractions & cache locality    │
└──────────────────────────────────────────────┘    └──────────────────────────────────────────────┘
```

### The 4 Core Tenets of this Curriculum
1. **Never Outsource Understanding Too Early:** Before using `serde`, write a parser. Before using `tokio::spawn`, write a native `std::sync::mpsc` threadpool. Before using `actix` or `axum`, build a raw `TcpListener` request parser.
2. **Compile-Time Guarantees over Runtime Checks:** Use the type system (Newtypes, Typestate pattern, PhantomData) to make invalid states unrepresentable.
3. **Zero Allocations Where Possible:** Prioritize `&str` over `String`, `&[T]` over `Vec<T>`, custom stack-allocated buffers, and `Cow<'a, T>` for lazy clones.
4. **Verifiable Engineering:** Every project must include integration tests, property-based tests (`proptest`), benchmarks (`criterion`), and sanitizer checks (`miri`, `clippy --pedantic`).

---

## 📊 Curriculum Architecture & Roadmap Matrix

| Stage | Focus Area | Book Mapping | Key Milestones | Real-World Competency |
| :--- | :--- | :--- | :--- | :--- |
| **Stage 1** | **Memory & Ownership** | Ch 4–9 | CLI Text Tool, In-Memory KV Store, Custom Vec / Linked List | Pointer semantics, stack vs heap, borrow checker, custom `Drop`, `Option`/`Result` handling |
| **Stage 2** | **Traits, Generics & I/O** | Ch 10–14 + Ciulla 6–10 | `minigrep-pro`, `micro-json` Parser, Dynamic Plugin Engine | Dynamic vs Static dispatch, associated types, parser combinators, zero-copy deserialization |
| **Stage 3** | **Concurrency & Network** | Ch 15–21 + Ciulla 11–13 | Threadpool TCP Server, Multi-threaded Crawler, Redis Lite (RESP) | `Send`/`Sync`, `Arc<Mutex<T>>`, lock-free channels, socket protocols, graceful degradation |
| **Stage 4** | **Async Web & Backend** | Zero to Production + Ciulla 14–16 | Enterprise REST/GraphQL Engine, PostgreSQL (SQLx), Tracing, Docker | Tokio runtime, middleware pipelines, database connection pools, observability, CI/CD |
| **Stage 5** | **Capstone & Low-Level** | Rustonomicon, RFCs | Raft Consensus / LSM-Tree DB / eBPF Kernel Tracer / Bytecode VM | `unsafe` validation, FFI, cache-friendly data layouts, distributed consensus, Miri |

---

## 🧱 Stage 1: Memory & Ownership Mastery

**Target Chapters:** The Rust Book (Ch 4: Ownership & Lifetimes, Ch 5: Structs & Methods, Ch 6: Enums & Pattern Matching, Ch 7: Crates & Modules, Ch 8: Collections, Ch 9: Error Handling).

```
                      Stage 1 Memory Architecture Flow
                 ┌──────────────────────────────────────┐
                 │          Stack Frame (Fast)          │
                 │  - Local variables & primitives      │
                 │  - Pointer to heap (ptr, cap, len)   │
                 └──────────────────┬───────────────────┘
                                    │ points to
                                    ▼
                 ┌──────────────────────────────────────┐
                 │             Heap Memory              │
                 │  - Dynamic arrays (Vec<T>)           │
                 │  - Dynamic text (String)             │
                 │  - Boxed nodes (Box<Node>)           │
                 └──────────────────────────────────────┘
```

---

### Project 1.1: `slice-tool` — High-Performance Zero-Copy Text Utility

#### Description
A command-line tool mimicking core functionalities of `wc`, `head`, `tail`, `dedup`, and word frequency counters. It processes multi-gigabyte text files without loading entire files into memory, utilizing string slices (`&str`), byte slices (`&[u8]`), and streaming buffers.

#### Key Rust Concepts Solidified
- Borrowing rules: exclusive mutable reference (`&mut T`) vs multiple immutable references (`&T`).
- Slices (`&str`, `&[u8]`) and lifetime annotations (`'a`).
- Efficient file I/O using `std::io::{BufRead, BufReader, Read, Write}`.
- Structs with lifetime parameters (`struct ChunkReader<'a>`).
- Custom Error Enums with `std::fmt::Display` and `std::error::Error`.

#### Crates & Tooling
- **Standard Library Only** (`std::io`, `std::env`, `std::fs`, `std::collections`).
- Optional for CLI argument parsing: manual parsing via `std::env::args()` first, then refactor with `clap` (derive mode).

#### Architecture & Key Type Signatures
```rust
pub struct LineStats<'a> {
    pub line_count: usize,
    pub word_count: usize,
    pub byte_count: usize,
    pub longest_line: &'a str,
}

pub enum ToolError {
    Io(std::io::Error),
    InvalidUtf8 { line_number: usize },
    InvalidArguments(String),
}

impl std::error::Error for ToolError {}
```

#### Stretch Goals & Edge Cases
- Handle non-UTF8 byte streams gracefully using `lossy` conversions or byte-mode parsing (`&[u8]`).
- Process memory-mapped files using `memmap2` crate for files > 2 GB.
- Implement an in-place deduplication algorithm with $O(1)$ extra heap allocation using a sliding window of references.

#### Verifiable Success Criteria
- [ ] Process a 1 GB log file in under 1.5 seconds with peak memory usage below 15 MB (`/usr/bin/time -v slice-tool log.txt`).
- [ ] 100% test coverage on UTF-8 edge cases (multi-byte emojis, carriage returns `\r\n`, trailing newlines, empty files).
- [ ] Zero compiler warnings with `#![warn(clippy::all, clippy::pedantic)]`.

---

### Project 1.2: `oxstore` — In-Memory Key-Value Store with Typed Values & TTL

#### Description
An embedded, in-memory key-value database engine that supports typed values (Strings, Integers, Floats, Lists, and Binary Blobs), expiration timestamps (TTL), and namespace isolation.

#### Key Rust Concepts Solidified
- Enums with rich data payloads (Algebraic Data Types).
- Pattern matching with `match`, `if let`, `while let`, and `matches!`.
- Ownership transitions in standard collections (`std::collections::HashMap`).
- Working with time (`std::time::{Instant, Duration, SystemTime}`).
- Encapsulation using Rust module hierarchy (`pub`, `pub(crate)`, `super`).

#### Architecture & Key Type Signatures
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Text(String),
    Integer(i64),
    Float(f64),
    List(Vec<Value>),
    Blob(Vec<u8>),
}

pub struct Entry {
    pub value: Value,
    pub created_at: std::time::Instant,
    pub expires_at: Option<std::time::Instant>,
}

pub struct OxStore {
    storage: std::collections::HashMap<String, Entry>,
}

impl OxStore {
    pub fn new() -> Self;
    pub fn set(&mut self, key: String, value: Value, ttl: Option<std::time::Duration>) -> Option<Value>;
    pub fn get(&mut self, key: &str) -> Option<&Value>;
    pub fn delete(&mut self, key: &str) -> bool;
    pub fn cleanup_expired(&mut self) -> usize;
}
```

#### Stretch Goals & Edge Cases
- Lazy TTL expiration on `get()` combined with an active background sweeping strategy using a min-heap (`std::collections::BinaryHeap`) based on expiry timestamps.
- Implement atomic increments (`incr_by`) on `Value::Integer` without cloning the key or reallocation.
- Implement persistent snapshot save/restore to disk using raw binary serialization.

#### Verifiable Success Criteria
- [ ] Active and passive TTL eviction passes all unit tests without memory leaks.
- [ ] Benchmarked lookup performance: $> 5,000,000$ operations/second for pure in-memory reads.
- [ ] All methods return clean `Result<T, StoreError>` or `Option<T>` with zero `panic!`, `unwrap()`, or `expect()`.

---

### Project 1.3: `custom-collections` — Vec & Singly Linked List with `Drop` & Iterators

#### Description
Implement two foundational data structures from scratch: `MyVec<T>` (a dynamically resizing heap-allocated array) and `PersistentStack<T>` (an immutable singly linked list). Manually manage capacity growth, pointer arithmetic, custom drop cleanups, and the `Iterator` trait.

#### Key Rust Concepts Solidified
- Manual heap allocation mental model (`std::alloc::{alloc, dealloc, realloc, Layout}`).
- RAII and implementing the `Drop` trait to avoid stack overflows on deeply nested recursive structures.
- Implementing custom `Iterator`, `IntoIterator`, and `DoubleEndedIterator`.
- The difference between `T`, `&T`, and `&mut T` during iteration.

#### Architecture & Key Type Signatures
```rust
// Part A: Custom Vector
pub struct MyVec<T> {
    ptr: *mut T,
    cap: usize,
    len: usize,
}

// Part B: Persistent Stack (Functional Linked List)
pub struct Stack<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T>(Stack<T>);
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
```

#### Stretch Goals & Edge Cases
- Write an iterative `impl<T> Drop for Stack<T>` that prevents recursion-induced stack overflow when dropping a 1,000,000-element list.
- Implement `MyVec::drain` and `MyVec::retain` with in-place element shifting.
- Validate memory safety with **Miri** (`cargo miri test`).

#### Verifiable Success Criteria
- [ ] Clean run of `cargo miri test` with zero undefined behaviors, memory leaks, or invalid pointer alignments.
- [ ] Custom `MyVec` reallocates exponentially ($2\times$) and matches standard vector behavior across 50 property tests.
- [ ] Successfully implement `Iterator` producing `&T`, `&mut T`, and `T` values.

---

## 🔬 Stage 2: Traits, Generics, and System I/O

**Target Chapters:** The Rust Book (Ch 10: Generic Types, Traits, and Lifetimes, Ch 11: Automated Tests, Ch 12: An I/O Project, Ch 13: Functional Features: Iterators & Closures, Ch 14: More About Cargo) + Ciulla (Ch 6: Traits, Ch 7: Generics, Ch 8: Closures, Ch 9: Smart Pointers, Ch 10: Error Handling).

```
                      Dynamic vs Static Dispatch
      Static Dispatch (Monomorphization):
      fn render<T: Renderable>(item: T)  --->  render_button(b), render_text(t)
      (Zero runtime cost, inlined, larger binary size)

      Dynamic Dispatch (Trait Objects):
      fn render(item: &dyn Renderable)   --->  vtable lookup at runtime
      (Pointer + VTable pointer, flexible plugin systems)
```

---

### Project 2.1: `minigrep-pro` — Regex Engine, Buffered Streamer & Colored CLI

#### Description
An industrial-grade replacement for grep that extends the standard Rust Book tutorial with recursive directory traversal, regular expression matching, multi-pattern search, colored terminal output, context lines (`-A`, `-B`, `-C`), and structured JSON output.

#### Key Rust Concepts Solidified
- Traits as interfaces and bounds (`T: AsRef<Path>`, `T: std::io::Write`).
- Higher-order functions, custom iterators, and closure combinators (`map`, `filter_map`, `flat_map`, `fold`).
- Clean separation of CLI parsing logic, business domain, and standard output rendering.
- Unit testing with mocks and integration testing with `assert_cmd` and `predicates`.

#### Crates & Tooling
- `regex` (or implement a basic NFA regex engine from scratch for maximum learning), `colored`, `walkdir`, `clap` (v4), `serde_json`.

#### Architecture & Key Type Signatures
```rust
pub trait Matcher {
    fn is_match(&self, line: &str) -> bool;
    fn find_matches<'a>(&self, line: &'a str) -> Vec<(usize, usize)>; // byte ranges
}

pub struct SearchConfig {
    pub query: String,
    pub case_insensitive: bool,
    pub before_context: usize,
    pub after_context: usize,
    pub colored: bool,
}

pub struct SearchEngine<M: Matcher, W: std::io::Write> {
    matcher: M,
    writer: W,
    config: SearchConfig,
}
```

#### Stretch Goals & Edge Cases
- Implement parallel directory traversal using `rayon` without locking output streams (use buffered channel writers).
- Support searching inside `.gz` compressed files transparently via dynamic decompression streams.
- Add `--replace <STRING>` flag supporting regex capture group expansions (`$1`, `$2`).

#### Verifiable Success Criteria
- [ ] Match performance within $2\times$ of GNU `grep` on 500 MB log benchmarks.
- [ ] Comprehensive integration tests covering edge cases: binary file detection, symlink loops, permission denied folders.
- [ ] Fully document every public function and type with `///` doc-tests verified via `cargo test --doc`.

---

### Project 2.2: `micro-json` — Custom JSON Parser with Parser Combinators

#### Description
Build a full JSON parser from the ground up that transforms raw strings into an AST (`JsonValue`) without external parser generators, and then build a second version using `nom` or `pest`. Implement custom serialization/deserialization traits similar to `serde`.

#### Key Rust Concepts Solidified
- Recursive data structures and indirect types using `Box<T>`.
- Lifetimes in zero-copy parsing (`JsonValue<'a>` referencing the input buffer).
- Parser combinator architecture (functions returning closures of type `impl Fn(Input) -> Result<(Input, Output), Error>`).
- Custom trait definition for serialization: `ToMyJson` and `FromMyJson`.

#### Crates & Tooling
- Version A: Pure `std` (Handwritten Recursive Descent Parser).
- Version B: `nom` or `pest`.
- Benchmark against `serde_json`.

#### Architecture & Key Type Signatures
```rust
#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue<'a> {
    Null,
    Bool(bool),
    Number(f64),
    String(&'a str),
    Array(Vec<JsonValue<'a>>),
    Object(std::collections::HashMap<&'a str, JsonValue<'a>>),
}

pub trait Deserializable<'a>: Sized {
    fn deserialize(json: &'a JsonValue<'a>) -> Result<Self, ParseError>;
}
```

#### Stretch Goals & Edge Cases
- Support Unicode escape sequences (`\uXXXX`) including surrogate pairs without allocating extra heap memory unless escaping occurs (using `std::borrow::Cow<'a, str>`).
- Handle nested objects/arrays up to 1,000 levels deep without blowing the call stack (convert recursive descent to explicit state machine with loop).
- Implement a derive procedural macro (`#[derive(MySerialize)]`) in a separate workspace sub-crate.

#### Verifiable Success Criteria
- [ ] Passes 100% of the [JSONTestSuite](https://github.com/nst/JSONTestSuite) test cases (parsing valid files, rejecting invalid files).
- [ ] Zero allocations when parsing JSON strings containing no escape characters (100% `&str` references).
- [ ] Benchmark against `serde_json` showing less than $3\times$ latency differential.

---

### Project 2.3: `flow-engine` — Extensible Data Pipeline via Trait Objects (`dyn Trait`)

#### Description
A modular, plugin-driven data transformation pipeline. Users can chain filter, transform, aggregate, and export steps defined dynamically via trait objects, configuration files (YAML/JSON), and dynamic runtime registration.

#### Key Rust Concepts Solidified
- Dynamic dispatch with trait objects (`Box<dyn Step>`, `&mut dyn Sink`).
- Object safety rules (why some traits cannot be made into `dyn Trait`).
- Generic type erasure and the `Any` trait for downcasting.
- Smart pointers: `Rc<T>`, `Arc<T>`, `RefCell<T>`, and interior mutability.

#### Architecture & Key Type Signatures
```rust
pub trait PipelineStep: Send + Sync {
    fn name(&self) -> &str;
    fn process(&self, record: Record) -> Result<Option<Record>, PipelineError>;
}

pub struct Pipeline {
    steps: Vec<Box<dyn PipelineStep>>,
    sink: Box<dyn Sink>,
}

impl Pipeline {
    pub fn add_step(&mut self, step: Box<dyn PipelineStep>) -> &mut Self;
    pub fn execute(&self, source: Box<dyn Source>) -> Result<ExecutionMetrics, PipelineError>;
}
```

#### Stretch Goals & Edge Cases
- Dynamic shared library loading (`.so` / `.dylib` / `.dll`) using `libloading` allowing runtime plugin extension without recompilation.
- Implement an event listener / interceptor pattern with interior mutability (`RefCell` / `RwLock`) to collect metrics on every step.

#### Verifiable Success Criteria
- [ ] Cleanly assemble and run pipelines from JSON definitions at runtime.
- [ ] Zero data races and zero memory leaks under prolonged high-throughput executions.
- [ ] Clear compilation failure demonstrations with explanations of object-safety violations.

---

## ⚡ Stage 3: Concurrency, Multithreading & Networking

**Target Chapters:** The Rust Book (Ch 15: Smart Pointers, Ch 16: Fearless Concurrency, Ch 17: OOP Features, Ch 18: Patterns, Ch 19: Advanced Features, Ch 20/21: Multithreaded Web Server) + Ciulla (Ch 11: Smart Pointers In-Depth, Ch 12: Concurrency & Channels, Ch 13: Networking Fundamentals).

```
                 Concurrent Architecture (Worker Pool & Channels)
     Incoming TCP Conns
             │
             ▼
     ┌───────────────┐        mpsc / crossbeam channel
     │  Dispatcher   │ ─────────────────────────────────┐
     └───────────────┘                                  │
                                                        ▼
                                           ┌─────────────────────────┐
                                           │      Job Queue          │
                                           └─────────────────────────┘
                                                        │
                      ┌─────────────────────────────────┼─────────────────────────────────┐
                      ▼                                 ▼                                 ▼
             ┌─────────────────┐               ┌─────────────────┐               ┌─────────────────┐
             │    Worker 1     │               │    Worker 2     │               │    Worker N     │
             │ (Arc<Mutex<..>) │               │ (Arc<Mutex<..>) │               │ (Arc<Mutex<..>) │
             └─────────────────┘               └─────────────────┘               └─────────────────┘
```

---

### Project 3.1: `ferris-server` — Multithreaded TCP HTTP/1.1 Server with Threadpool

#### Description
A production-hardened multi-threaded HTTP/1.1 web server built directly on top of `std::net::TcpListener`. Includes a custom bounded threadpool, atomic queue metrics, static file streaming, keep-alive connections, and graceful shutdown on OS signals (`SIGINT`, `SIGTERM`).

#### Key Rust Concepts Solidified
- Raw socket networking (`TcpListener`, `TcpStream`).
- Thread synchronization primitives: `std::thread`, `Arc<T>`, `Mutex<T>`, `Condvar`, `atomic::{AtomicUsize, AtomicBool, Ordering}`.
- Message passing using `std::sync::mpsc` or `crossbeam-channel`.
- Signal handling and resource cleanup on shutdown.

#### Crates & Tooling
- Standard library (`std::net`, `std::sync`, `std::thread`).
- `ctrlc` or `signal-hook` for signal interception.

#### Architecture & Key Type Signatures
```rust
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<std::sync::mpsc::Sender<Job>>,
}

pub struct Worker {
    id: usize,
    thread: Option<std::thread::JoinHandle<()>>,
}

pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body: Vec<u8>,
}
```

#### Stretch Goals & Edge Cases
- Support `HTTP/1.1 Keep-Alive` by reusing streams with socket read timeouts (`set_read_timeout`).
- Implement Chunked Transfer Encoding for streaming large files without loading them entirely into memory.
- Bounded worker queue: if queue is full, return `503 Service Unavailable` immediately rather than unbounded memory growth.

#### Verifiable Success Criteria
- [ ] Sustain $> 35,000$ requests/second on `wrk` benchmark (`wrk -t12 -c400 -d30s http://127.0.0.1:8080/`).
- [ ] Flawless graceful shutdown: during an active load test, `Ctrl+C` drains in-flight requests and exits with 0 dropped sockets.
- [ ] Zero memory leaks or dangling threads verified via `valgrind` or OS process inspection.

---

### Project 3.2: `spider-rs` — High-Throughput Concurrent Web Crawler

#### Description
A distributed-style multi-threaded web scraper that crawls a targeted domain, parses HTML links, respects `robots.txt`, enforces per-host rate limits, and exports a clean sitemap graph to JSON or SQLite.

#### Key Rust Concepts Solidified
- Concurrent data structures (`dashmap::DashMap` or `Arc<RwLock<HashSet<String>>>`).
- Worker-manager concurrency topology with barrier synchronization (`std::sync::Barrier`).
- Rate limiting using token bucket algorithms with atomic counters.
- Channel fan-out / fan-in patterns.

#### Crates & Tooling
- `reqwest` (blocking mode for Stage 3, or `ureq`), `scraper` / `lol_html`, `crossbeam`, `url`, `dashmap`.

#### Architecture & Key Type Signatures
```rust
pub struct CrawlerConfig {
    pub root_url: url::Url,
    pub max_depth: usize,
    pub max_concurrent_workers: usize,
    pub delay_per_host: std::time::Duration,
}

pub struct Crawler {
    visited: std::sync::Arc<dashmap::DashSet<String>>,
    queue_tx: crossbeam_channel::Sender<CrawlTask>,
    queue_rx: crossbeam_channel::Receiver<CrawlTask>,
    active_workers: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}
```

#### Stretch Goals & Edge Cases
- Prevent spider traps (infinite calendar URLs, query param mutations) by implementing URL canonicalization and bloom filters.
- Persist crawl state periodically so interrupted crawls can resume without re-fetching visited pages.

#### Verifiable Success Criteria
- [ ] Crawl 10,000 pages within a test domain in $< 60$ seconds with bounded memory ($< 100$ MB).
- [ ] Proper error handling: handle timeouts, DNS resolution failures, 404/500 HTTP codes without worker thread panics.

---

### Project 3.3: `rusty-redis` — Redis Lite Engine with RESP Parser & Striped Locks

#### Description
A standalone clone of Redis server supporting the official RESP2 (Redis Serialization Protocol) protocol. Compatible with official Redis clients (`redis-cli`, Python/Node redis libraries), supporting `PING`, `SET`, `GET`, `DEL`, `EXPIRE`, `INCR`, `MGET`, `PUBLISH`, and `SUBSCRIBE`.

#### Key Rust Concepts Solidified
- Binary protocol wire parsing and serializing.
- High-concurrency memory storage using lock striping (partitioned `Arc<RwLock<HashMap<K, V>>>` to minimize lock contention).
- Publish-Subscribe pattern using broadcast channels.
- Custom state machine design for socket stream state.

#### Crates & Tooling
- `nom` or manual byte parser for RESP protocol, `crossbeam-channel`, `parking_lot` for high-performance synchronization primitives.

#### Architecture & Key Type Signatures
```rust
#[derive(Debug, PartialEq, Clone)]
pub enum RespFrame {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Option<Vec<u8>>),
    Array(Option<Vec<RespFrame>>),
}

pub struct StripedStore {
    num_shards: usize,
    shards: Vec<parking_lot::RwLock<std::collections::HashMap<String, StoreValue>>>,
}
```

#### Stretch Goals & Edge Cases
- Support Append-Only File (`AOF`) logging on every write with background `fsync` worker.
- Handle pipeline commands (`redis-cli --pipe`) by reading and executing multiple framed requests in a single TCP read buffer.

#### Verifiable Success Criteria
- [ ] Connect seamlessly using official `redis-cli` (`redis-cli -p 6379 set foo bar`, `get foo`).
- [ ] Pass `redis-benchmark` with $> 50,000$ ops/sec on single-node setup.
- [ ] Zero deadlocks under heavy concurrent multi-client read/write workloads.

---

## 🚀 Stage 4: Production Async Web Backend & Systems

**Target Frameworks & Tooling:** Ciulla (Ch 14: Async Rust, Ch 15: Web Applications, Ch 16: Microservices) + Luca Palmieri's *Zero to Production in Rust*.

```
                      Stage 4 Production Service Topology
     Client (HTTP/HTTPS)
             │
             ▼
     ┌────────────────────────────────────────────────────────┐
     │                      Axum Router                       │
     │  - Tower Layers: Tracing, CORS, Auth, Compression, Rate│
     └───────────────────────────┬────────────────────────────┘
                                 │
                 ┌───────────────┴───────────────┐
                 ▼                               ▼
     ┌───────────────────────┐       ┌───────────────────────┐
     │   Handlers / Domain   │       │   Security / OAuth    │
     │   - Type-safe inputs  │       │   - Argon2id Password │
     │   - Validation Rules  │       │   - JWT / Refresh Tok │
     └───────────┬───────────┘       └───────────────────────┘
                 │
                 ▼
     ┌────────────────────────────────────────────────────────┐
     │                SQLx Asynchronous Pool                  │
     │  - Compile-time verified queries against PostgreSQL    │
     │  - Automatic migrations & connection health checks     │
     └────────────────────────────────────────────────────────┘
```

---

### Project 4.1: `nexus-api` — Enterprise Async Microservice

#### Description
A production-grade, enterprise-ready asynchronous backend service (e.g., a Team Collaboration / Issue Tracker / Subscription Billing Engine) featuring Axum, SQLx with PostgreSQL, Redis caching, JWT & Argon2id authentication, structured telemetry via `tracing` and Prometheus metrics, containerized with multi-stage Docker builds and automated CI pipelines.

#### Key Rust Concepts Solidified
- Asynchronous runtime internals (`tokio`, async tasks, cooperative scheduling).
- The `Future` and `Pin` mental model; avoiding common async pitfalls (cancellation safety, holding locks across `.await` points).
- Compile-time checked SQL queries using `sqlx::query!` and `sqlx::query_as!`.
- Tower middleware stack (`Service`, `Layer`, `Extractors`).
- Domain-Driven Design (DDD): separating Domain, Application, and Infrastructure layers with strict type safety.
- Hierarchical configuration management using `config` and environment overrides.

#### Crates & Dependencies
- **Core Async:** `tokio` (features = ["full"]), `axum` (v0.7+), `tower`, `tower-http`.
- **Database & Cache:** `sqlx` (features = ["postgres", "runtime-tokio-rustls", "uuid", "chrono"]), `bb8-redis` / `deadpool-redis`.
- **Security:** `argon2`, `jsonwebtoken`, `secrecy` (to prevent accidental logging of credentials).
- **Observability:** `tracing`, `tracing-subscriber`, `tracing-bunyan-formatter`, `metrics`, `metrics-exporter-prometheus`.
- **Validation & Serde:** `serde`, `validator`, `uuid`, `chrono`.
- **Testing:** `reqwest`, `testcontainers`, `fake`, `wiremock`.

#### Enterprise Directory Layout
```
nexus-api/
├── Cargo.toml
├── Dockerfile
├── .github/workflows/ci.yml
├── migrations/
│   └── 20260101_init.sql
├── src/
│   ├── main.rs              # Application entrypoint & startup
│   ├── lib.rs               # Library root (enables integration tests)
│   ├── configuration.rs     # Environment & settings loader
│   ├── domain/              # Pure domain models, value objects, invariants
│   │   ├── mod.rs
│   │   ├── user_email.rs    # Validated Email newtype
│   │   ├── user_password.rs # Hashed password wrapper
│   │   └── issue.rs
│   ├── routes/              # HTTP Handlers (Extractors & Responses)
│   │   ├── mod.rs
│   │   ├── auth/
│   │   ├── issues/
│   │   └── health.rs
│   ├── services/            # Business orchestration & external APIs
│   ├── repository/          # Database persistence implementations
│   ├── middleware/          # JWT auth guards, request ID, metrics
│   └── telemetry.rs         # Tracing subscriber initialization
└── tests/
    ├── api/                 # Black-box API integration tests
    │   ├── helpers.rs       # Test app spawner with isolated DB
    │   ├── auth_tests.rs
    │   └── issue_tests.rs
```

#### Production Features to Implement
1. **Typestate & Newtypes:** Never pass raw strings for IDs or unvalidated emails:
   ```rust
   #[derive(Clone, Debug, PartialEq)]
   pub struct UserEmail(String);

   impl UserEmail {
       pub fn parse(s: String) -> Result<Self, DomainError> {
           if validator::validate_email(&s) {
               Ok(Self(s))
           } else {
               Err(DomainError::InvalidEmail)
           }
       }
   }
   ```
2. **Zero-Secret Leakage:** Wrap all sensitive credentials in `secrecy::Secret<String>` so they are masked automatically in logs and `Debug` printers.
3. **Transactional Outbox Pattern:** Ensure database writes and message broker events are committed within a single PostgreSQL transaction.
4. **Structured Tracing with Correlation IDs:** Every incoming HTTP request generates a `x-request-id` header which flows across all tracing spans and database queries.
5. **Automated Ephemeral Integration Tests:** Each integration test run uses `testcontainers` or a random PostgreSQL logical schema to ensure 100% test isolation and parallel execution.

#### Verifiable Success Criteria
- [ ] 100% compile-time SQL verification without active DB connection during build via `cargo sqlx prepare --check`.
- [ ] Passing integration test suite running completely automated via GitHub Actions CI in under 3 minutes.
- [ ] Multi-stage Docker image resulting in a minimal scratch or distroless container size $< 25$ MB.
- [ ] Full `/metrics` Prometheus endpoint and health check `/healthz` returning database and cache health status.

---

## 🏆 Stage 5: Advanced & Capstone Mastery

Choose **one** of the four capstone projects below. Each option represents a masterclass in a distinct domain of systems programming and serves as the centerpiece of a Staff-level Rust portfolio.

```
                            CAPSTONE TRACKS
┌─────────────────────────┐                     ┌─────────────────────────┐
│     OPTION A: DISTRIBUTED│                     │  OPTION B: STORAGE ENGINE│
│  Raft Consensus / P2P   │                     │  LSM-Tree DB / WAL      │
├─────────────────────────┤                     ├─────────────────────────┤
│ • Leader Election, RPC  │                     │ • MemTable, SSTables    │
│ • Log Replication       │                     │ • Bloom Filters, Compaction
└─────────────────────────┘                     └─────────────────────────┘
             │                                               │
             ├───────────────────────┬───────────────────────┤
             │                       │                       │
┌─────────────────────────┐                     ┌─────────────────────────┐
│   OPTION C: LINUX / eBPF│                     │ OPTION D: COMPILER / VM │
│  Kernel Tracing & FFI   │                     │ Bytecode VM / WASM      │
├─────────────────────────┤                     ├─────────────────────────┤
│ • aya / libbpf-rs       │                     │ • Stack/Register Engine │
│ • Raw Ring Buffers      │                     │ • Garbage Collector, JIT│
└─────────────────────────┘                     └─────────────────────────┘
```

---

### Capstone Option A: `raft-rs` — Distributed Consensus & Replicated KV Engine

#### Description
Build a fully compliant implementation of the **Raft Distributed Consensus Algorithm** (Ongaro & Ousterhout) from scratch. Build an active distributed cluster of nodes that negotiate leader election, replicate state machine logs, survive network partitions, handle dynamic node membership changes, and expose a linearizable distributed key-value store.

#### Key Rust Concepts Solidified
- Complex asynchronous state machines and actor pattern.
- Network RPC serialization (using `tonic` / gRPC or custom binary frames).
- Deterministic simulation testing and chaos testing (simulating packet drops, delays, and node crashes).
- Persistent Write-Ahead Log storage with fsync guarantees.

#### Architecture Blueprint
```rust
pub enum NodeRole {
    Follower,
    Candidate,
    Leader,
}

pub struct RaftNode<S: StateMachine> {
    id: NodeId,
    peers: Vec<NodeId>,
    current_term: u64,
    voted_for: Option<NodeId>,
    log: Vec<LogEntry>,
    commit_index: usize,
    last_applied: usize,
    state_machine: S,
    role: NodeRole,
}
```

#### Verifiable Success Criteria
- [ ] Pass the complete Jepsen-style linearizability tests under simulated network partitions (split-brain resistance).
- [ ] Automatic leader election within $< 300$ ms of leader termination.
- [ ] Zero state corruption after arbitrary SIGKILL and restart cycles.

---

### Capstone Option B: `pebble-db` — LSM-Tree & WAL Embedded Storage Engine

#### Description
An embedded, high-performance key-value storage engine modeled after LevelDB / RocksDB using a **Log-Structured Merge-Tree (LSM-tree)**. Features an active in-memory `MemTable` (backed by a SkipList), disk Write-Ahead Logging (`WAL`), immutable `SSTables` on disk with Block Bloom Filters, sparse indexes, and background tiered / leveled compaction.

#### Key Rust Concepts Solidified
- Unsafe / Low-level pointer manipulation and binary layout optimization (`repr(C)`, byte alignment, memory mapping via `memmap2`).
- Cache-efficient data structures (SkipList, Block-based SSTable format).
- Multi-threaded background compaction workers coordinating with active writers using atomic swaps and read-copy-update (RCU) semantics.
- Binary encoding formats (Varint, CRC32 checksums).

#### Storage Engine Architecture
```
Write Path:
Key/Value ──► [ WAL (Append-Only Disk) ] ──► [ MemTable (In-Memory SkipList) ]
                                                        │
                                                        ▼ (When full)
                                             [ Immutable MemTable ]
                                                        │
                                                        ▼ (Background Flush)
Disk Storage:                                [ Level 0 SSTable ]
                                                        │
                                                        ▼ (Compaction)
                                             [ Level 1 SSTables ]
```

#### Verifiable Success Criteria
- [ ] 100% crash consistency: recover complete state from WAL after simulated sudden process abort.
- [ ] Read performance optimized with Bloom filters: $> 98\%$ of non-existent key lookups avoided disk I/O.
- [ ] Benchmark comparison against `sled` and `rocksdb` showing sustained write throughput $> 100,000$ writes/sec.

---

### Capstone Option C: `sys-observer` — Linux eBPF Kernel Observer & Telemetry Daemon

#### Description
A real-time Linux kernel monitoring and security observability tool written in pure Rust using **eBPF** (`aya` or `libbpf-rs`). Attaches probes (`kprobes`, `tracepoints`, `XDP`, `socket filters`) to capture process executions (`execve`), network packet throughput, file opens, and TCP lifecycle events directly in kernel space and passes them to a userspace async daemon via lockless ring buffers.

#### Key Rust Concepts Solidified
- Writing `#![no_std]` Rust code compiled to eBPF bytecode (`bpfel-unknown-none`).
- Safe userspace abstractions over raw kernel pointers and C FFI structures.
- High-performance lockless communications using kernel Ring Buffers (`BPF_MAP_TYPE_RINGBUF`).
- Linux systems programming: Unix domain sockets, signals, system capabilities (`CAP_BPF`, `CAP_NET_ADMIN`).

#### Architecture Blueprint
```
Kernel Space (eBPF #![no_std] Rust):
[ sys_enter_execve ] ──► Parse Arguments ──► Push to BPF RingBuffer
                                                        │
                                                        ▼ (Zero-Copy Transfer)
Userspace Daemon (Tokio + Aya):
[ RingBuffer Reader ] ──► Event Filter ──► Structured Tracing ──► Prometheus/OTel
```

#### Verifiable Success Criteria
- [ ] Zero memory allocations in kernel probe execution path.
- [ ] Handle $> 500,000$ events/second with $< 1\%$ CPU overhead on the monitored host.
- [ ] Verify safety through the Linux Kernel BPF In-Kernel Verifier without rejects.

---

### Capstone Option D: `vanguard-vm` — Bytecode VM & WASM Web Emulator

#### Description
A fast bytecode compiler and stack-based virtual machine for a custom dynamic programming language, complete with a bytecode assembler, disassembler, register allocator, Cheney/Mark-Sweep garbage collector, and WebAssembly compilation target running in the browser via Canvas/WebGL.

#### Key Rust Concepts Solidified
- Compiler frontend design: Lexer, Recursive Descent AST Parser, Bytecode Emitter.
- Virtual Machine dispatch loops (Direct Threaded Code vs Match loops).
- Unsafe memory management: Implementing a custom Mark-Sweep or Tracing Garbage Collector in Rust.
- Compiling Rust to WebAssembly (`wasm32-unknown-unknown`) and interfacing with JavaScript via `wasm-bindgen` and `web-sys`.

#### VM Instruction Pipeline
```
Source Code (*.vg)
       │
       ▼ (Lexer & Parser)
Abstract Syntax Tree (AST)
       │
       ▼ (Bytecode Compiler)
Bytecode Chunk (Opcodes + Constant Pool)
       │
       ▼ (VM Execution Loop)
┌────────────────────────────────────────────────────────┐
│  Vanguard VM Runtime                                   │
│  - Call Frames & Value Stack                           │
│  - Object Heap & Mark-Sweep Garbage Collector          │
│  - Native Function FFI Hooks                           │
└────────────────────────────────────────────────────────┘
```

#### Verifiable Success Criteria
- [ ] Execute recursive Fibonacci (`fib(35)`) and array sorting benchmarks matching or exceeding Python bytecode interpreter speeds.
- [ ] Custom Garbage Collector safely sweeps all unreferenced objects without memory leaks, validated via Miri.
- [ ] Compile VM to WebAssembly and interactively execute scripts in a live web-based terminal.

---

## 🎖️ Staff Engineering Production Standards & Portfolio Checklist

To present your Rust repositories at a Staff / Senior Principal level, every repository must adhere to the following strict criteria:

### 1. Zero-Tolerance Compiler & Linter Configuration
Every project root `Cargo.toml` or `lib.rs` must enforce strict linting:
```toml
# In Cargo.toml
[lints.rust]
unsafe_code = "forbid" # Unless building Stage 5 Unsafe/FFI capstone
missing_docs = "warn"

[lints.clippy]
all = "warn"
pedantic = "warn"
nursery = "warn"
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
```

### 2. Comprehensive Documentation & Architecture Decision Records (ADR)
- **Crate-Level Docs:** Every public module, struct, enum, and function must have doc-comments with runnable examples tested via `cargo test --doc`.
- **ADR Folder (`/docs/adr/`):** Document key architectural choices (e.g., *Why we chose DashMap over RwLock<HashMap>*, *Why we selected Axum over Actix-Web*).

### 3. Systematic Testing Pyramid
```
                   TESTING PYRAMID
                ┌──────────────────┐
                │   Fuzz Testing   │  cargo-fuzz / libFuzzer (Crash discovery)
                ├──────────────────┤
                │  Property Tests  │  proptest (Invariants across 10,000 cases)
                ├──────────────────┤
                │ Integration Tests│  tests/*.rs (Black-box API & Network)
                ├──────────────────┤
                │    Unit Tests    │  src/**/*.rs (Pure function validation)
                └──────────────────┘
```

### 4. Continuous Integration (CI) Standard Workflow
Every repository must feature a `.github/workflows/ci.yml` running:
1. **Formatting:** `cargo fmt -- --check`
2. **Lints:** `cargo clippy --all-targets --all-features -- -D warnings`
3. **Tests:** `cargo test --all-targets --all-features`
4. **Miri (where applicable):** `cargo miri test`
5. **Security Audit:** `cargo audit` and `cargo deny check`
6. **Code Coverage:** `cargo llvm-cov --lcov --output-path lcov.info` (enforce $> 85\%$ coverage).

---

## 📚 Appendix: Learning Resources, Books & Mentorship FAQ

### Essential Literature Progression
1. **Foundations (Stages 1–2):**
   - *The Rust Programming Language* (Steve Klabnik, Carol Nichols)
   - *The Rust Programming Handbook* (Francesco Ciulla)
   - *Programming Rust, 2nd Edition* (Jim Blandy, Jason Orendorff, Leonora Tindall)
2. **Systems, Concurrency & Idioms (Stage 3):**
   - *Rust Atomics and Locks* (Mara Bos) — **Mandatory for multithreading**
   - *Effective Rust* (David Drysdale)
3. **Production Web & Architecture (Stage 4):**
   - *Zero to Production in Rust* (Luca Palmieri) — **The definitive guide for backend Rust**
4. **Advanced & Low-Level (Stage 5):**
   - *The Rustonomicon* (The Dark Arts of Unsafe Rust)
   - *Database Internals* (Alex Petrov) — For LSM-Tree / Storage Capstone
   - *Crafting Interpreters* (Robert Nystrom) — For Bytecode VM Capstone

---

### Mentorship FAQ

#### Q: When should I use `clone()` vs redesigning my data model?
> **Staff Mentor Rule:** During rapid prototyping, `clone()` is acceptable to get the compiler satisfied. However, before finalizing any module, inspect every `.clone()`. If you are cloning large buffers or collections to satisfy the borrow checker, ask:
> 1. Can the function take a reference `&T` instead of an owned `T`?
> 2. Can I use `std::borrow::Cow` (Clone-on-Write) to avoid allocations for unmodified data?
> 3. Does my struct have multiple entities wanting ownership? If yes, consider `Arc<T>` for immutability or a centralized Arena / Index-based ID allocator.

#### Q: How do I know when to use dynamic dispatch (`dyn Trait`) vs static dispatch (`impl Trait` / Generics)?
> **Staff Mentor Rule:** Default to **static dispatch (`impl Trait` / generics)**. It allows compiler inlining, produces zero runtime overhead, and catches errors early. Switch to **dynamic dispatch (`Box<dyn Trait>` / `&dyn Trait`)** only when:
> 1. You need heterogeneous collections (e.g., `Vec<Box<dyn PipelineStep>>` containing different structs).
> 2. You are building a dynamic plugin system loaded at runtime.
> 3. Binary code bloat from excessive monomorphization is an actual measured problem.

#### Q: When is `unsafe` acceptable in production Rust?
> **Staff Mentor Rule:** There are only three valid reasons to write `unsafe`:
> 1. Interfacing with foreign C/C++ libraries or operating system syscalls (FFI).
> 2. Implementing foundational data structures where the compiler cannot verify pointer safety (e.g., custom lock-free queues, ring buffers).
> 3. Measured, profile-proven hot loops requiring SIMD or eliding bounds checks where the optimizer failed.
> *Every `unsafe` block must be accompanied by a `// SAFETY:` comment rigorously proving why undefined behavior is impossible.*

---

## 🎯 Next Steps for the Student

1. **Create the Project Directory:** Initialize `01_slice_tool` inside your workspace.
2. **Set up Strict Linting:** Add `#![warn(clippy::all, clippy::pedantic)]` to your `main.rs`.
3. **Begin Stage 1 Project 1.1:** Build the zero-copy file streaming utility and benchmark against standard UNIX tools.
4. **Commit Every Milestone:** Maintain a clean Git history with conventional commits (`feat:`, `fix:`, `perf:`, `test:`, `docs:`).

*Happy Hacking! May the Borrow Checker be ever in your favor.* 🦀
