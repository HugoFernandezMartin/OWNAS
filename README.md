# OWNAS

## Overview

OWNAS is a a self‑hosted, lightweight cloud synchronization and data storage service written primarily in Rust, optimized for performance, security, and portability across low‑power devices. It focuses on offering efficient local storage and a simple synchronization interface, prioritizing minimal resource usage and predictable latency.

## Features

- CLI server management
- JSON configuration
- Modular logging system
- Extensible architecture

## Project Structure

The project is organized into several Rust modules to maintain a clean and scalable architecture.  
Each module encapsulates a specific responsibility within the system:

```
src/
├── bin/
│   ├── ownas-cli.rs        # Entry point for the command-line interface (client)
│   └── ownas-daemon.rs     # Entry point for the background daemon (server)
│
├── cli/                    # CLI layer: handles user input and command dispatch
│   ├── client.rs           # Manages IPC communication with the daemon
│   ├── handlers.rs         # Contains command-specific logic for the CLI
│   └── mod.rs
│
├── commands/               # Definition and implementation of CLI commands
│   ├── files.rs            # File-related commands (list, create, delete)
│   ├── run.rs              # Server control commands (start, stop, status, exec)
│   └── mod.rs
│
├── config/                 # Configuration system
│   ├── loader.rs           # Loads configuration from a JSON file
│   ├── types.rs            # Defines configuration data structures
│   └── mod.rs
│
├── core/                   # Core utilities and shared data structures
│   ├── state.rs            # Server runtime state management
│   ├── responses.rs        # Defines response types for IPC communication
│   └── mod.rs
│
├── logging/                # Logging system
│   ├── init.rs             # Initializes and configures the logger
│   └── mod.rs
│
└── server/                 # Daemon core logic
    ├── builder.rs          # Constructs and initializes the server instance
    ├── file_manager.rs     # Handles file operations within the workspace
    ├── ipc_listener.rs     # Listens for and accepts IPC connections
    ├── ipc_handler.rs      # Handles individual IPC client requests
    ├── tcp_listener.rs     # (Reserved for future TCP interface)
    ├── tcp_handler.rs      # (Reserved for future TCP handling)
    └── mod.rs
```

This modular design follows a clear separation of concerns:

- The **CLI** acts as the user-facing interface.
    
- The **daemon** executes and manages tasks in the background.
    
- The **server**, **core**, and **logging** modules handle runtime behavior, state, and observability.
    
- The **config** and **commands** modules define user configuration and interaction logic.

## Getting Started

For executing OWNAS:
```
cargo install --path .
echo 'alias ownas="ownas-cli"' >> ~/.bashrc
source ~/.bashrc
```
## Usage

For using OWNAS:
 1. Start server with `ownas start`. Once running, it listens for incoming commands through IPC (Unix socket).  
Logs are written to the console and managed through the modular logging system.

| Command           | Description                                  |
| ----------------- | -------------------------------------------- |
| `ownas start`     | Starts the server                            |
| `ownas stop`      | Stops the server                             |
| `ownas status`    | Checks whether the server is running         |
| `ownas run <cmd>` | Executes a command inside the server context |
2. Server configuration is defined in a JSON file:
```
{
	"logging": {
		"tracing_level": "DEBUG",
		"logfile_path": "tmp/ownas.log"
	},
	
	"server": {
		"host": "localhost",
		"port": 8080
	}
}
```
3. The CLI also allows simple file management within the server environment:

| Command                           | Description                  |
| --------------------------------- | ---------------------------- |
| `ownas files list`                | Lists files in the workspace |
| `ownas files create <name>`       | Creates a new empty file     |
| `ownas files delete <name>`       | Deletes a file               |
| `ownas files write <name> <text>` | Append text to a file        |
| `ownas files read <name>`         | Show file content            |
## Documentation

See [docs/specification.md](docs/specification.md) for full MVP and system design.

## License

GPL
