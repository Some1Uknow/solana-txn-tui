# Solana Transaction TUI

A comprehensive Terminal User Interface (TUI) for exploring Solana transactions, accounts, and wallets on Mainnet, Devnet, and Testnet.

## Features

- **Multi-Network Support**: Switch between Mainnet, Devnet, and Testnet
- **Auto-Detection**: Automatically detects if input is a transaction signature or account address
- **Transaction Details**:
  - Signature, slot, timestamp
  - Status (success/failed with error)
  - Fee information
  - Compute Units & Priority Fee
  - Instructions details (Program, type, data)
  - Token Transfers
  - Account list with balance changes
  - Transaction logs
- **Account Details**:
  - SOL balance
  - Account type (system, program, data)
  - Owner information
  - Token accounts with balances
  - Recent transaction history
- **Keyboard Navigation**: Intuitive shortcuts for all actions

## Installation

### Quick Install (Mac & Linux)

The easiest way to install without needing Rust or Homebrew:

```bash
curl -fsSL https://raw.githubusercontent.com/Some1UKnow/solana-txn-tui/main/install.sh | bash
```

### Windows

Download the `.exe` from the [Latest Releases](https://github.com/Some1UKnow/solana-txn-tui/releases) page.

### Using Cargo (Recommended for Rust users)

```bash
cargo install solana-txn-tui
```

### Using Homebrew (macOS/Linux)

```bash
brew tap Some1UKnow/solana-txn-tui
brew install solana-txn-tui
```

### Build from Source

```bash
git clone https://github.com/Some1UKnow/solana-txn-tui.git
cd solana-txn-tui
cargo build --release
```

The binary will be at `target/release/solana-txn-tui`

## Usage

```bash
# If installed via cargo or brew
solana-txn-tui

# If built from source
./target/release/solana-txn-tui
```

### Controls

**Input Screen:**
- Type to enter transaction signature or account address
- `↑/↓` - Change network (Mainnet/Devnet/Testnet)
- `Enter` - Submit query
- `q` or `Esc` - Quit

**Transaction/Account Views:**
- `Tab` / `Shift+Tab` - Switch between tabs (Overview, Accounts, Instructions, etc.)
- `↑/↓` - Scroll up/down
- `PageUp/PageDown` - Scroll faster
- `Home` - Jump to top
- `r` - Return to input screen
- `q` - Quit

**Error Screen:**
- `r` or `Enter` - Return to input
- `q` - Quit

## Example

Test with a real devnet transaction:
```
Transaction: 4AeG6yqyqfRhJzBy2apTcCrVEDsEwqgHWsc8uFvdaKnseuYB8SjWC83KidujaELqe6sqGTUhdkK4eCzgNWWnbv3W
Network: Devnet
```

## Project Structure

```
solana-txn-tui/
├── src/
│   ├── main.rs              # Entry point & terminal setup
│   ├── app.rs               # App state management
│   ├── events.rs            # Keyboard event handling
│   ├── solana/
│   │   ├── mod.rs           # Network enum & exports
│   │   ├── client.rs        # Solana RPC client
│   │   └── types.rs         # Data structures
│   ├── ui/
│   │   ├── mod.rs           # Main UI coordinator
│   │   ├── input_screen.rs  # Input & network selection UI
│   │   ├── transaction_view.rs  # Transaction details display
│   │   ├── account_view.rs  # Account details display
│   │   └── styles.rs        # Theme & colors
│   └── utils/
│       └── validators.rs    # Input validation helpers
├── Cargo.toml
├── README.md
└── LICENSE
```

## Dependencies

- `ratatui` - Modern TUI framework
- `crossterm` - Cross-platform terminal control
- `solana-client` - Solana RPC client
- `solana-sdk` - Solana types and primitives
- `solana-transaction-status` - Transaction parsing
- `solana-account-decoder` - Account data decoding
- `serde` & `serde_json` - Serialization
- `anyhow` - Error handling
- `chrono` - Date/time handling

## Technical Highlights

- **Zero async complexity**: Uses synchronous RPC calls for simplicity
- **Clean architecture**: Separated concerns (app state, events, UI, Solana client)
- **Type-safe**: Leverages Rust's type system for Solana primitives
- **Minimal dependencies**: Only essential crates for functionality
- **Self-documenting code**: Minimal comments, expressive naming

## Testing

```bash
cargo test
```

## Publishing (For Maintainers)

### Publish to crates.io

1. Update version in `Cargo.toml`
2. Run tests: `cargo test`
3. Dry run: `cargo publish --dry-run`
4. Publish: `cargo publish`

### Create Homebrew Formula

1. Create a new GitHub repository: `homebrew-solana-txn-tui`
2. Add the formula file (see below)
3. Users can then: `brew tap Some1UKnow/solana-txn-tui && brew install solana-txn-tui`

**Formula Template:**
```ruby
class SolanaTxnTui < Formula
  desc "Comprehensive TUI for exploring Solana transactions and accounts"
  homepage "https://github.com/Some1UKnow/solana-txn-tui"
  url "https://github.com/Some1UKnow/solana-txn-tui/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "SHA256_OF_RELEASE_TAR_GZ"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "solana-txn-tui", shell_output("#{bin}/solana-txn-tui --version")
  end
end
```

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

If you encounter any issues or have questions, please [open an issue](https://github.com/Some1UKnow/solana-txn-tui/issues).
