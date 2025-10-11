use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

use crate::error::DBCErrorCode;

pub fn get_init_param(
    supply: u64,
    total_fund_raising: u64,
    total_sell: u64,
    total_locked_amount: u64,
    migrate_fee: u64,
) -> Result<(u64, u64, u64), DBCErrorCode> {
    if supply <= total_sell {
        return Err(DBCErrorCode::InvalidInput.into());
    };

    let supply_minus_sell_locked = supply
        .checked_sub(total_sell)
        .ok_or(DBCErrorCode::Underflow)?
        .checked_sub(total_locked_amount)
        .ok_or(DBCErrorCode::Underflow)?;

    if supply_minus_sell_locked == 0 {
        return Err(DBCErrorCode::InvalidInput.into());
    };

    let tf_minus_mf = total_fund_raising
        .checked_sub(migrate_fee)
        .ok_or(DBCErrorCode::Underflow)?;

    if tf_minus_mf == 0 {
        return Err(DBCErrorCode::InvalidInput.into());
    };

    let tf_minus_mf_u128 = u128::from(tf_minus_mf);
    let total_sell_u128 = u128::from(total_sell);
    let supply_minus_sell_locked_u128 = u128::from(supply_minus_sell_locked);
    let total_fund_raising_u128 = u128::from(total_fund_raising);

    let intermediate = tf_minus_mf_u128
        .checked_mul(total_sell_u128)
        .ok_or(DBCErrorCode::Overflow)?
        .checked_div(supply_minus_sell_locked_u128)
        .ok_or(DBCErrorCode::Underflow)?;

    let numerator = intermediate
        .checked_mul(total_sell_u128)
        .ok_or(DBCErrorCode::Overflow)?;

    let denominator = tf_minus_mf_u128
        .checked_mul(total_sell_u128)
        .ok_or(DBCErrorCode::Overflow)?
        .checked_div(supply_minus_sell_locked_u128)
        .ok_or(DBCErrorCode::Underflow)?
        .checked_sub(total_fund_raising_u128)
        .ok_or(DBCErrorCode::Underflow)?;

    if denominator == 0 {
        return Err(DBCErrorCode::InvalidCalculation.into());
    };

    let x0_u128 = numerator
        .checked_div(denominator)
        .ok_or(DBCErrorCode::Underflow)?;

    let max_safe_value = 340_282_366_920_938_463_463u128;

    let y0_u128 = if total_fund_raising_u128 <= max_safe_value {
        total_fund_raising_u128
            .checked_mul(total_fund_raising_u128)
            .ok_or(DBCErrorCode::Overflow)?
            .checked_div(denominator)
            .ok_or(DBCErrorCode::Underflow)?
    } else {
        let partial = total_fund_raising_u128
            .checked_div(denominator)
            .ok_or(DBCErrorCode::Underflow)?;
        partial
            .checked_mul(total_fund_raising_u128)
            .ok_or(DBCErrorCode::Overflow)?
    };

    let x0 = u64::try_from(x0_u128).or(Err(DBCErrorCode::Overflow))?;
    let y0 = u64::try_from(y0_u128).or(Err(DBCErrorCode::Overflow))?;

    Ok((x0, y0, total_sell))
}

pub fn compute_s_in(
    v_token_reserve: u64,
    v_sol_reserve: u64,
    t_out: u64,
) -> Result<u64, DBCErrorCode> {
    let v_token_128 = u128::try_from(v_token_reserve).or(Err(DBCErrorCode::Overflow))?;

    let scale_u128 = u128::try_from(1_000_000).or(Err(DBCErrorCode::Overflow))?;

    let t_out_128 = u128::try_from(t_out).or(Err(DBCErrorCode::Overflow))?;

    if t_out_128 > v_token_128 {
        return Err(DBCErrorCode::InvalidCalculation.into());
    }

    let denominator: u128 = v_token_128
        .checked_sub(t_out_128)
        .ok_or(DBCErrorCode::Overflow)?;

    if denominator == 0 {
        return Err(DBCErrorCode::InvalidCalculation.into());
    }

    let q: u128 = u128::try_from(v_token_reserve)
        .or(Err(DBCErrorCode::Overflow))?
        .checked_mul(u128::try_from(v_sol_reserve).or(Err(DBCErrorCode::Overflow))?)
        .ok_or(DBCErrorCode::Overflow)?;

    let constant_product_u128 = u128::try_from(q).or(Err(DBCErrorCode::Overflow))?;

    let lamports_per_sol_u128: u128 =
        u128::try_from(LAMPORTS_PER_SOL).or(Err(DBCErrorCode::Overflow))?;

    let multiplicative_factor: u128 = lamports_per_sol_u128
        .checked_mul(scale_u128)
        .ok_or(DBCErrorCode::Overflow)?;

    let numerator: u128 = constant_product_u128
        .checked_mul(multiplicative_factor)
        .ok_or(DBCErrorCode::Overflow)?;

    let part_1: u128 = numerator
        .checked_div(denominator)
        .ok_or(DBCErrorCode::InvalidCalculation)?;

    let part_2 = u128::try_from(v_sol_reserve).or(Err(DBCErrorCode::Overflow))?;

    let s_in: u128 = part_1.checked_sub(part_2).ok_or(DBCErrorCode::Underflow)?;

    let s_in_u64 = u64::try_from(s_in).or(Err(DBCErrorCode::Overflow))?;

    Ok(s_in_u64)
}

pub fn compute_s_out(
    v_token_reserve: u64,
    v_sol_reserve: u64,
    t_in: u64,
) -> Result<u64, DBCErrorCode> {
    let v_token_128 = u128::try_from(v_token_reserve).or(Err(DBCErrorCode::Overflow))?;

    let t_in_128 = u128::try_from(t_in).or(Err(DBCErrorCode::Overflow))?;

    if t_in_128 > v_token_128 {
        return Err(DBCErrorCode::InvalidCalculation.into());
    }

    let denominator: u128 = v_token_128
        .checked_add(t_in_128)
        .ok_or(DBCErrorCode::Overflow)?;

    if denominator == 0 {
        return Err(DBCErrorCode::InvalidCalculation.into());
    }

    let q: u128 = u128::try_from(v_token_reserve)
        .or(Err(DBCErrorCode::Overflow))?
        .checked_mul(u128::try_from(v_sol_reserve).or(Err(DBCErrorCode::Overflow))?)
        .ok_or(DBCErrorCode::Overflow)?;

    let constant_product_u128 = u128::try_from(q).or(Err(DBCErrorCode::Overflow))?;

    let lamports_per_sol_u128: u128 =
        u128::try_from(LAMPORTS_PER_SOL).or(Err(DBCErrorCode::Overflow))?;

    let scale_u128 = u128::try_from(1_000_000).or(Err(DBCErrorCode::Overflow))?;

    let multiplicative_factor: u128 = lamports_per_sol_u128
        .checked_mul(scale_u128)
        .ok_or(DBCErrorCode::Overflow)?;

    let numerator: u128 = constant_product_u128
        .checked_mul(multiplicative_factor)
        .ok_or(DBCErrorCode::Overflow)?;

    let part_1 = u128::try_from(v_sol_reserve).or(Err(DBCErrorCode::Overflow))?;

    let part_2: u128 = numerator
        .checked_div(denominator)
        .ok_or(DBCErrorCode::InvalidCalculation)?;

    if part_2 > part_1 {
        return Err(DBCErrorCode::Overflow.into());
    }

    let s_out: u128 = part_1.checked_sub(part_2).ok_or(DBCErrorCode::Underflow)?;

    let s_out_u64 = u64::try_from(s_out).or(Err(DBCErrorCode::Overflow))?;

    Ok(s_out_u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_init_param_85_sol() {
        let supply = 1_000_000_000_000_000;
        let total_fund_raising = 85_000_000_000;
        let total_sell = 800_000_000_000_000;
        let total_locked_amount = 0;
        let migrate_fee = 0;

        let (x0, y0, d) = get_init_param(
            supply,
            total_fund_raising,
            total_sell,
            total_locked_amount,
            migrate_fee,
        )
        .expect("get_init_param should succeed");

        println!("85 SOL case - x0: {}, y0: {}, d: {}", x0, y0, d);

        // Expected values
        assert_eq!(
            x0, 1_066_666_666_666_666,
            "virtual_base should be ~1.067 quadrillion"
        );
        assert_eq!(y0, 28_333_333_333, "virtual_quote should be ~28.33 million");
        assert_eq!(d, 800_000_000_000_000, "total_sell should match");
    }
}
