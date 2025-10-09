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
