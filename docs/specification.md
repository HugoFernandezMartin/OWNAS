# MVP Specification – Distributed Self‑Hosted Cloud Service
**Author:** Hugo  
**Version:** 1.3 (Refined build strategy and architecture updates)  
**Date:** 2025‑10‑25  

---

## 1. Overview

This MVP defines the first stable iteration of a **self‑hosted, lightweight cloud synchronization and data storage service** written primarily in **Rust**, optimized for **performance, security, and portability across low‑power devices** (e.g., Raspberry Pi 1B+).  
It focuses on offering efficient local storage and a simple synchronization interface, prioritizing minimal resource usage and predictable latency.  

---

## 2. MVP Core Goals

1. **Lightweight Personal Cloud**
   - Provide local file storage accessible via web interface.
   - Operate without external dependencies or cloud APIs.

2. **Efficient Local Server**
   - Written in Rust for maximum control over performance and memory.
   - Minimal runtime footprint, targeting ARMv6 and x86 platforms.

3. **Self‑Contained Deployment**
   - Single binary executable.
   - Uses SQLite as the embedded storage engine (no external DB daemon).
   - Auto‑configuration on first launch.

4. **Web Interface**
   - Simple, responsive dashboard built with pure HTML, CSS, and JS.
   - Displays user files and system status (storage usage, sync state).
   - Modular structure allowing migration to React in future releases.

---

## 3. System Architecture

### **3.1 Overview Diagram**
```
┌──────────────────────────────┐
│        Web Frontend          │
│ HTML / CSS / JS (future: React)│
└──────────────┬───────────────┘
               │ REST API (HTTP / JSON)
┌──────────────┴───────────────┐
│         Rust Backend          │
│ ┌──────────────────────────┐ │
│ │   Core Logic Layer       │ │
│ │   ├─ Auth / Sessions     │ │
│ │   ├─ File Management     │ │
│ │   ├─ Sync Engine         │ │
│ │   └─ Config Manager      │ │
│ └──────────────────────────┘ │
│ ┌──────────────────────────┐ │
│ │     Storage Layer        │ │
│ │   SQLite (rusqlite/sqlx) │ │
│ └──────────────────────────┘ │
└──────────────┬───────────────┘
               │
       Local Filesystem (ext4, FAT, etc.)
```

### **3.2 Core Modules**

| Module | Description | Key Crates |
|--------|--------------|-------------|
| **Core** | Main runtime and configuration handling | `tokio` / `smol`, `serde`, `toml` |
| **API** | HTTP server, REST routing | `axum` / `warp` |
| **Storage** | File index, metadata DB | `rusqlite`, `sqlx` |
| **Sync** | Handles sync requests and conflict resolution | custom module, optional async jobs |
| **Auth** | Lightweight user/session management | `argon2`, `jsonwebtoken` |
| **Metrics (future)** | Collects system usage data | `sysinfo`, `heim` |

---

## 4. Build Strategy

### **4.1 Development Approach**

The project will follow a **two‑stage build strategy** balancing functionality and optimization.

#### **Stage 1 – Full Build (Default)**
- Uses complete versions of runtime and frameworks (`tokio`, `axum`, `sqlx` with full features).  
- Prioritizes developer experience, maintainability, and full functionality.  
- Ideal for development and debugging phases.  
- Enables accurate benchmarking on Raspberry Pi and other hardware before optimization.

#### **Stage 2 – Minimal Build (Optimized)**
- Introduces compile‑time feature flags to reduce footprint.  
- Removes optional dependencies and async overhead when targeting ultra‑low‑power devices.  
- Example `Cargo.toml` configuration:

```toml
[features]
default = ["full"]
minimal = []
full = ["tokio/full", "axum/macros", "sqlx/runtime-tokio"]
```

Conditional compilation example:

```rust
#[cfg(feature = "minimal")]
use smol as runtime;

#[cfg(not(feature = "minimal"))]
use tokio as runtime;
```

This phased approach ensures that the MVP is **developed quickly, tested thoroughly**, and later **optimized intelligently** based on real metrics.

---

## 5. Performance Strategy

- **Low Memory Footprint:** avoid heavy async runtimes in minimal mode, optional compile‑time features.  
- **I/O Efficiency:** buffered file reads/writes, memory‑mapped files if available.  
- **Parallelism:** async task pools only when multicore available.  
- **Caching:** lightweight in‑memory metadata cache.

Target benchmark (Raspberry Pi 1B+):  
- Idle RAM usage < 50 MB  
- CPU < 10% under light I/O load  

---

## 6. Storage Design

- **SQLite database** for metadata, user config, and index of files.  
- **Filesystem storage** for actual file data.  
- Future support for **sync replication** via networked peers (optional).  

Schema (simplified):

```sql
CREATE TABLE files (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    size INTEGER,
    hash TEXT,
    modified_at DATETIME
);

CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    username TEXT UNIQUE,
    password_hash TEXT,
    created_at DATETIME
);
```

---

## 7. Deployment

- **Single binary build:** `cargo build --release`  
- **Cross‑compile targets:** `armv6-unknown-linux-gnueabihf`, `x86_64-unknown-linux-gnu`  
- **Autostart option:** via `systemd` service or `crontab` entry  

---

## 8. Future Expansions

| Feature | Description |
|----------|--------------|
| **Performance Metrics Dashboard** | Real‑time CPU, RAM, and network stats via `/api/system/perf`. |
| **Calendar Integration** | File synchronization scheduler with user rules. |
| **Peer‑to‑Peer Sync** | Optional multi‑device synchronization. |
| **React Frontend Migration** | Rich dashboard experience, live metrics charts. |
| **PostgreSQL Support** | For multi‑user and scalable deployments. |

---

## 9. Roadmap

| Phase | Milestone | Deliverable |
|-------|------------|-------------|
| **Phase 1** | Core Backend + Local Storage | Stable binary with SQLite |
| **Phase 2** | Web UI + REST API | Functional dashboard |
| **Phase 3** | Config + Auth | User profiles & access control |
| **Phase 4** | Metrics & Expansion | System monitoring & extensions |
| **Phase 5** | Minimal Build Release | Optimized variant for low‑power devices |

---

## 10. License and Distribution

- **License:** GPL.  
- **Distribution:** Compiled binaries + open source repository.  
- **Goal:** Empower low‑power users to self‑host efficient private storage solutions.

---
