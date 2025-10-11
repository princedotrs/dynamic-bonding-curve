use anchor_lang::prelude::*;

pub const ADMIN: Pubkey = pubkey!("ANqg3T5SC4LEf4FZQwxxbrEzDcNv6uCXBbQrxaEBm8qr");
pub const supply: u64 = 1_000_000_000_000_000;

pub mod seeds {
    pub const GLOBAL: &[u8] = b"global";
    pub const BONDING_CURVE: &[u8] = b"bonding_curve";
    pub const CURVE_VAULT: &[u8] = b"curve_vault";
}
