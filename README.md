# dynamic-bonding-curve: a token-launch bonding curve on Solana, built with Anchor

A Pump.fun-style launch program written in Rust with Anchor. Anyone can create a token that trades against SOL on a constant-product curve backed by virtual reserves. Instead of hand-picking those virtual reserves, the creator says how much SOL they want to raise and how many tokens to sell, and the program works out a curve that hits both targets.

What you'll find here:

- deriving virtual reserves from fundraising targets, in checked `u128` math
- buy and sell along `x · y = k` with a platform fee and slippage limits
- creating an SPL mint with Metaplex metadata from a program, then revoking the mint authority
- holding SOL in a PDA vault and signing for it with seeds
- marking a curve complete once it has raised its target

## What the program does

| Instruction | Who calls it | What it does |
| --- | --- | --- |
| `initialize()` | admin | Creates the `Global` config: fee recipient, a 1% default fee and the default token supply |
| `set_params(admin, fee_recipient, fee_basis_points, new_supply)` | admin | Updates the global config |
| `create(token_args, dbc_curve_args)` | creator | Creates the mint and metadata, mints the whole supply to the curve, revokes the mint authority and sets up the curve |
| `buy(amount, max_sol_cost)` | trader | Buys `amount` tokens, failing if SOL in plus fee would exceed `max_sol_cost` |
| `sell(amount, min_sol_output)` | trader | Sells `amount` tokens, failing if SOL out after the fee is below `min_sol_output` |

## Accounts

```
Global PDA          seeds = ["global"]                  admin, fee_recipient, fee_basis_points, supply
BondingCurve PDA    seeds = ["bonding_curve", mint]     virtual and real reserves, completion target, complete flag
 ├─ vault PDA       seeds = ["curve_vault", mint]       holds the SOL raised
 └─ curve ATA       ATA(mint, owner = BondingCurve)     holds the unsold tokens
```

## How the curve works

Trades use the constant-product formula on **virtual** reserves `x` (tokens) and `y` (SOL):

```
buy  t tokens:  SOL in  = x · y / (x − t) − y
sell t tokens:  SOL out = y − x · y / (x + t)
```

`create` takes four numbers: `total_fund_raising` (SOL to raise), `total_sell` (tokens sold on the curve), `total_locked_amount` and `migrate_fee`. From those, `get_init_param` in `utils.rs` solves for the starting virtual reserves `x0` and `y0` so that:

- buying all `total_sell` tokens raises exactly `total_fund_raising`, and
- the price at the end of the curve equals the SOL raised (minus the migration fee) divided by the tokens left over, so that liquidity could be moved into an AMM pool at the same price.

For example, a 1,000,000,000-token supply (6 decimals) selling 800,000,000 tokens to raise 85 SOL starts at `x0 ≈ 1.067B` tokens and `y0 ≈ 28.33` SOL. This case is covered by a Rust unit test.

`create` also checks that the target is at least 40 SOL and that tokens sold plus tokens locked make up at least 80% of supply.

Each buy and sell charges `fee_basis_points` (1% by default) to the fee recipient. Once the SOL raised passes the completion target, the curve is marked `complete` and further buys and sells are rejected. Moving the liquidity to an AMM after completion is not implemented here.

## Run it

Prerequisites: Rust, the Solana CLI, Anchor, Node.js + Yarn, and [Surfpool](https://surfpool.run).

```bash
git clone https://github.com/princedotrs/dynamic-bonding-curve.git
cd dynamic-bonding-curve
yarn install
anchor build

# curve math unit tests
cargo test -p dynamic-bonding-curve

# terminal 1: local validator
surfpool start

# terminal 2: integration tests
anchor test --skip-local-validator
```

The integration tests (`tests/dynamic-bonding-curve.ts`) initialize the global config, create a token with a curve, then buy and sell against it.

## Project layout

```
programs/dynamic-bonding-curve/src/
├── lib.rs               # instruction entry points
├── constants.rs         # admin key, default supply, PDA seeds
├── utils.rs             # curve math: get_init_param, compute_s_in, compute_s_out (+ unit tests)
├── error.rs
├── state/               # Global, BondingCurve
└── instructions/
    ├── admin/           # initialize, set_params
    └── user/            # create, buy, sell
target/idl, target/types # IDL and TypeScript types for clients
tests/                   # end-to-end tests
```

## Ideas to extend it

- Migrate the raised SOL and remaining tokens into an AMM pool when the curve completes
- Add an admin instruction to pause and unpause trading (the `paused` flag is already checked)
- Vest the locked tokens to the creator over time

## License

MIT
