# Mission: Rust Systems Programming

## Why
Acquire working competence in Rust from fundamental mechanics (memory layout, ownership, borrow checker) to systems architecture (async runtimes, multithreading, storage engines, network services).

## Success looks like
- Predict compiler borrow-checker decisions and memory layouts accurately.
- Build multithreaded and asynchronous services using standard idioms (Arc, Mutex, tokio, axum, sqlx).
- Implement systems software from first principles: the Elite 5-Project Systems Stack (a typed arena allocator, a zero-copy streaming log parser, a content-addressed Git object store, a Tokio RESP async gateway, and a distributed Durable Objects engine with S3 CAS leases).
- Handle error boundaries cleanly using thiserror for libraries and anyhow for applications.
- Complete the Rustlings drills, Exercism track, and Rust Book milestone projects without Clippy warnings.

## Constraints
- Schedule: 1 to 2 hours daily Monday to Thursday (reading, notes, targeted quizzes); 3 to 5 hours daily Friday to Sunday (drills, project implementation, test suites).
- Writing: Strictly zero emojis across documentation, code, and commit logs. Follow unslop direct prose guidelines.
- Verification: Daily progress must be verified empirically against git logs and file modification timestamps.

## Out of scope
- GUI frameworks (Iced, Slint, Tauri) until core systems programming is completed.
- Manual pointer manipulation and unsafe blocks prior to completing Chapter 15 and mastering safe abstraction boundaries.
