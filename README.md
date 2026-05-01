![Quo Preview](https://cms.protoqol.nl/assets/2ecc5f44-5fe5-4f15-95d6-ba365f4fcd5c)

![Latest Release](https://img.shields.io/github/v/release/Protoqol/Quo?style=flat-square&color=%23ec135b)

Quo is a cross-platform variable dumper designed to make debugging easier. It receives data from your application and
displays it in a clean desktop interface, allowing you to inspect complex values in real-time without cluttering your
terminal or browser console.

## Features

- **Real-time Inspection**: See variables as they are dumped from your code.
- **Cross-platform**: Works on Windows, macOS, and Linux.
- **Multi-language Support**: Official companion packages for Rust, PHP, and JavaScript/TypeScript.
- **Zero-config Fallback**: Macro calls safely no-op if the desktop app is not running.

## Getting started

Integrating Quo into your workflow is a simple two-step process.

1. **Install the Desktop App**: [Download the latest version here](https://quo.protoqol.sh/download?utm_source=github) or via the release page on GitHub for your
   operating system.
2. **Add a Companion Package**: Choose the package for your language below and follow the installation instructions.

---

## Rust [`quo-rust`](https://github.com/Protoqol/Quo-rust)

Use the `quo-rust` crate to send variables with simple macro calls. 

Add `quo` to your `Cargo.toml` under `dependencies`:

```toml
[dependencies]
quo = { version = "0.1", package = "quo-rust" }
```

To enable additional data capture, use feature flags:

```toml
[dependencies]
# Enable specific features
quo = { version = "0.1", package = "quo-rust", features = ["stack-trace", "system-info", "hashing"] }

# Or enable everything
quo = { version = "0.1", package = "quo-rust", features = ["full"] }
```

### Available Features

- `stack-trace`: Captures the call stack and the caller function name.
- `system-info`: Captures current CPU and memory usage of the process.
- `hashing`: Generates a reproducible grouping hash for variables (`var_type:name:origin`), allowing the Quo client to group and diff values over time.
- `full`: Enables all the above features.

Basic info like **Thread ID/Name**, **Runtime Environment** (OS/Arch), and **Memory Address** are included by default.

### Quick Start

Using `quo!()` macro:

```rust
use quo::quo;

#[derive(Debug)]
struct User {
    id: u32,
    username: String,
}

fn main() {
    let user_id = 42;
    let user = User { id: 1, username: "jdoe".to_string() };

    // Dump a single variable
    quo!(user_id);

    // Dump multiple variables at once
    quo!(user_id, user);
}
```

---

## PHP [`quo-php`](https://github.com/Protoqol/Quo-php)

The PHP companion package allows you to dump values from any PHP application.

### Installation

Install the package via Composer:

```bash
composer require protoqol/quo-php
```

### Quick Start

Use the global `quo()` helper function:

```php
<?php

require_once 'vendor/autoload.php';

$userId = 42;
$username = 'dev_user';

quo([$userId, $username]);
```

---

## JavaScript / TypeScript [`quo-ts`](https://github.com/Protoqol/Quo-ts)

Use the JavaScript package in Node.js or browser projects to dump runtime values.

### Installation

Install the package via npm or yarn:

```bash
npm install @protoqol/quo-ts
```

### Quick Start

Import the `quo` function and pass your data:

```javascript
import {quo} from "@protoqol/quo-ts";

const userId = 42;
const username = "dev_user";

quo([userId, username]);
```

## License

Quo is open-source software licensed under the [GPL-3 license](LICENSE).

