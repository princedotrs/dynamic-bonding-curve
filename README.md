# Dynamic Bonding Curve

A Solana-based program implementing a dynamic bonding curve mechanism for token launch and distribution. This project provides an advanced alternative to traditional bonding curves with enhanced flexibility and improved tokenomics.

## 🚀 Overview

The Dynamic Bonding Curve program enables the creation and management of tokens with sophisticated pricing mechanisms that adapt to market conditions. Unlike traditional bonding curves, this implementation offers dynamic parameters that can be adjusted to optimize token distribution and price discovery.

## 📊 What is a Dynamic Bonding Curve?

### Traditional Bonding Curves
Traditional bonding curves use a fixed mathematical formula (typically `price = k * supply^n`) where:
- Price increases predictably with supply
- Parameters are set at deployment and cannot be changed
- Limited flexibility for different market conditions
- Simple but rigid pricing mechanism

### Dynamic Bonding Curves
Dynamic bonding curves offer several enhancements:
- **Adjustable Parameters**: Curve parameters can be modified based on market conditions
- **Virtual Reserves**: Separate virtual and real reserves for more sophisticated price discovery
- **Completion Mechanics**: Built-in graduation to full market trading
- **Fee Structures**: Integrated platform fees and migration costs
- **Locked Amounts**: Support for token locking mechanisms

## 🎯 Benefits of Dynamic Bonding Curves

### 1. **Enhanced Price Discovery**
- More realistic price movements through virtual reserves
- Better initial price stability
- Reduced manipulation opportunities

### 2. **Flexible Tokenomics**
- Configurable total supply allocation
- Adjustable locked amounts for team/treasury
- Dynamic fee structures

### 3. **Market Graduation**
- Automatic transition to full market trading
- Configurable completion thresholds
- Built-in liquidity migration

### 4. **Risk Management**
- Slippage protection for buyers and sellers
- Maximum cost limits for transactions
- Minimum output guarantees

### 5. **Administrative Control**
- Pausable contracts for security
- Admin parameter updates
- Fee recipient management

## 🏗️ Architecture

### Core Components

#### 1. **BondingCurve State**
```rust
pub struct BondingCurve {
    pub mint: Pubkey,                    // Token mint address
    pub completion_lamports: u64,        // SOL needed for completion
    pub virtual_token_reserve: u64,      // Virtual token reserves
    pub virtual_sol_reserve: u64,        // Virtual SOL reserves
    pub real_token_reserve: u64,         // Actual token reserves
    pub real_sol_reserve: u64,           // Actual SOL reserves
    pub token_total_supply: u64,         // Total token supply
    pub complete: bool,                  // Graduation status
    pub initializer: Pubkey,             // Creator address
    pub bump: u8,                        // PDA bump
    pub vault_bump: u8,                  // Vault PDA bump
}
```

#### 2. **Global Configuration**
```rust
pub struct Global {
    pub admin: Pubkey,              // Admin authority
    pub fee_recipient: Pubkey,      // Platform fee recipient
    pub fee_basis_points: u64,      // Platform fee (basis points)
    pub supply: u64,                // Default token supply
    pub paused: bool,               // Emergency pause state
    pub bump: u8,                   // PDA bump
}
```

### 3. **Core Instructions**

#### Administrative Instructions
- `initialize()` - Initialize global configuration
- `set_params()` - Update global parameters

#### User Instructions
- `create()` - Create new token with bonding curve
- `buy()` - Purchase tokens from the curve
- `sell()` - Sell tokens back to the curve

## 🔧 Installation & Setup

### Prerequisites
- Rust 1.70+
- Solana CLI 1.16+
- Anchor Framework 0.31+
- Node.js 18+
- Yarn

### Installation

1. **Clone the repository**
```bash
git clone https://github.com/prince981620/dynamic-bonding-curve.git
cd dynamic-bonding-curve
```

2. **Install dependencies**
```bash
# Install Rust dependencies
cargo build

# Install Node.js dependencies
yarn install
```

3. **Setup Solana environment**
```bash
# Set to localnet for development
solana config set --url localhost

# Generate a new keypair if needed
solana-keygen new
```

4. **Build the program**
```bash
anchor build
```

## 🧪 Testing

### Run Tests
```bash
# Start surfpool
surfpool start --watch

# Run all tests
anchor test

# Run specific test
anchor test --skip-local-validator
```

### Test Coverage
The test suite covers:
- Global initialization
- Token creation with bonding curves
- Buy/sell operations
- Parameter updates
- Error conditions

## 📖 Usage Examples

### 1. Initialize Global Configuration
```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DynamicBondingCurve } from "../target/types/dynamic_bonding_curve";

const program = anchor.workspace.dynamicBondingCurve as Program<DynamicBondingCurve>;

await program.methods
  .initialize()
  .rpc();
```

### 2. Create a New Token with Bonding Curve
```typescript
const tokenArgs = {
  name: "Test Token",
  symbol: "TEST",
  uri: "https://example.com/metadata.json"
};

const dbcCurveArgs = {
  totalFundRaising: new anchor.BN(100 * anchor.web3.LAMPORTS_PER_SOL), // 100 SOL
  totalSell: new anchor.BN(800_000_000_000_000), // 80% of supply
  totalLockedAmount: new anchor.BN(100_000_000_000_000), // 10% locked
  migrateFee: new anchor.BN(5 * anchor.web3.LAMPORTS_PER_SOL) // 5 SOL
};

await program.methods
  .create(tokenArgs, dbcCurveArgs)
  .accounts({
    // ... account configuration
  })
  .rpc();
```

### 3. Buy Tokens
```typescript
const amount = new anchor.BN(1_000_000_000); // 1,000 tokens
const maxSolCost = new anchor.BN(anchor.web3.LAMPORTS_PER_SOL); // Max 1 SOL

await program.methods
  .buy(amount, maxSolCost)
  .accounts({
    // ... account configuration
  })
  .rpc();
```

### 4. Sell Tokens
```typescript
const amount = new anchor.BN(500_000_000); // 500 tokens
const minSolOutput = new anchor.BN(0.4 * anchor.web3.LAMPORTS_PER_SOL); // Min 0.4 SOL

await program.methods
  .sell(amount, minSolOutput)
  .accounts({
    // ... account configuration
  })
  .rpc();
```

## 🔒 Security Features

### Access Control
- Admin-only functions for critical parameters
- Signer validation for all user actions
- PDA-based account derivation

### Safety Mechanisms
- Overflow/underflow protection
- Slippage protection for trades
- Emergency pause functionality
- Input validation and sanitization

### Economic Security
- Virtual reserves prevent manipulation
- Fee structures aligned with platform incentives
- Graduation mechanics ensure proper market transition

## 📁 Project Structure

```
dynamic-bonding-curve/
├── programs/
│   └── dynamic-bonding-curve/
│       └── src/
│           ├── lib.rs              # Main program module
│           ├── constants.rs        # Program constants
│           ├── error.rs           # Error definitions
│           ├── utils.rs           # Utility functions
│           ├── instructions/      # Instruction implementations
│           │   ├── admin/         # Admin instructions
│           │   └── user/          # User instructions
│           └── state/             # State definitions
├── tests/                         # Test files
├── migrations/                    # Deployment scripts
├── runbooks/                      # Deployment runbooks
├── target/                        # Build outputs
├── Anchor.toml                    # Anchor configuration
├── Cargo.toml                     # Rust dependencies
├── package.json                   # Node.js dependencies
└── README.md                      # This file
```

## 🛠️ Configuration

### Anchor.toml
```toml
[programs.localnet]
dynamic_bonding_curve = "BnHQoRBC1kedRoiFfrXvxzRDUPM64R7wxaPT2PsDndKw"

[registry]
url = "https://api.apr.dev"

[provider]
cluster = "localnet"
wallet = "~/.config/solana/id.json"
```

### Environment Variables
- `ANCHOR_PROVIDER_URL` - Solana RPC endpoint
- `ANCHOR_WALLET` - Path to wallet keypair

## 🚀 Deployment

### Local Deployment
```bash
# Start local validator
anchor localnet

# Deploy to localnet
anchor deploy
```

### Devnet Deployment
```bash
# Switch to devnet
solana config set --url devnet

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

### Mainnet Deployment
```bash
# Switch to mainnet
solana config set --url mainnet-beta

# Deploy to mainnet (use with caution)
anchor deploy --provider.cluster mainnet-beta
```

## 📋 Constants & Parameters

### Default Configuration
- **Total Supply**: 1,000,000,000,000,000 tokens (1 quadrillion)
- **Decimals**: 6
- **Admin**: ANqg3T5SC4LEf4FZQwxxbrEzDcNv6uCXBbQrxaEBm8qr

### Fee Structure
- Platform fees calculated in basis points (1% = 100 basis points)
- Configurable fee recipient
- Migration fees for curve completion

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the ISC License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Anchor Framework](https://anchor-lang.com/)
- [Solana Documentation](https://docs.solana.com/)
- [Metaplex Token Metadata](https://github.com/metaplex-foundation/metaplex-program-library)

## ⚠️ Disclaimer

This software is provided "as is" without warranty of any kind. Use at your own risk. Always audit smart contracts before deploying to mainnet.

---

**Built with ❤️ on Solana**