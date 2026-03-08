![Quo Preview](https://cms.protoqol.nl/assets/2ecc5f44-5fe5-4f15-95d6-ba365f4fcd5c)

Quo is a cross-platform variable dumper designed to make debugging easier. It receives data from your application and
displays it in a clean desktop interface, allowing you to inspect complex values in real-time without cluttering your
terminal or browser console.

> **Note**: Quo is currently undergoing a significant rebuild, transitioning from Electron to Tauri for better
> performance and a smaller footprint.

## Features

- **Real-time Inspection**: See variables as they are dumped from your code.
- **Cross-platform**: Works on Windows, macOS, and Linux.
- **Multi-language Support**: Official companion packages for Rust, PHP, and JavaScript/TypeScript.
- **Zero-config Fallback**: Macro calls safely no-op if the desktop app is not running.

## Getting started

Integrating Quo into your workflow is a simple two-step process.

1. **Install the Desktop App**: [Download the latest version here](/download) or via the release page on GitHub for your
   operating system.
2. **Add a Companion Package**: Choose the package for your language below and follow the installation instructions.

---

## Rust (work in progress)

Use the `quo-rust` crate to send variables with simple macro calls.

### Installation

Add `quo-rust` to your `Cargo.toml`:

```toml
[dev-dependencies]
quo = { version = "0.1.7", package = "quo-rust" }
```

### Quick Start

Import the macro and pass variables to inspect:

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

## PHP (work in progress)

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

## JavaScript / TypeScript (work in progress)

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

Quo is open-source software licensed under the [GPL-3 license](.github/LICENSE).

