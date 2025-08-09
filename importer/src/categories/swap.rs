use crate::{Transfer, SwapSubCategory, TwoAssetSwap, TransferDirection,
  DebtSwap, DebtDirection,
};
use itertools::Itertools;

use rust_decimal::Decimal;
use std::str::FromStr;

impl TryFrom<&Vec<Transfer>> for DebtSwap {
    type Error = &'static str;
    fn try_from(transfers: &Vec<Transfer>) -> Result<DebtSwap, &'static str> {
      if transfers.len() != 2 { Err("Nope")? }
      if transfers.iter().map(|x| x.direction.clone()).all_equal() == false {
        Err("Nope")?
      }

      let debt = transfers.iter().find(|x| x.token.is_debt == true).ok_or("Nope")?;
      let token = transfers.iter().find(|x| x.token.is_debt == false).ok_or("Nope")?;
      let debt_value = debt.value;
      let value = token.value;

      Ok(DebtSwap {
        direction: match debt.direction {
          TransferDirection::Incoming => DebtDirection::Borrow,
          TransferDirection::Outgoing => DebtDirection::Repayment,
        },
        debt_token: debt.token.clone(),
        token: token.token.clone(),
        debt_value,
        value,
      })
  }
}

impl TryFrom<&Vec<Transfer>> for TwoAssetSwap {
    type Error = &'static str;
    fn try_from(transfers: &Vec<Transfer>) -> Result<TwoAssetSwap, &'static str> {
      if transfers.len() != 2 { return Err("Nope"); }

      let sold = transfers.iter().find(|x| x.direction == TransferDirection::Outgoing).ok_or("Nope")?;
      let purchased = transfers.iter().find(|x| x.direction == TransferDirection::Incoming).ok_or("Nope")?;
      let value_purchased = purchased.value;
      let value_sold = purchased.value;

      match (sold.usd_value, purchased.usd_value) {
        (Some(value_sold_usd), Some(value_purchased_usd)) => Ok(TwoAssetSwap {
          cost_basis: (value_sold_usd + value_purchased_usd) / Decimal::from_str("2").unwrap(),
          token_purchased: purchased.token.clone(),
          token_sold: sold.token.clone(),
          value_purchased,
          value_sold,
        }),
        (Some(cost_basis), _) if sold.token.is_usd => Ok(TwoAssetSwap {
          cost_basis,
          token_purchased: purchased.token.clone(),
          token_sold: sold.token.clone(),
          value_purchased,
          value_sold,
        }),
        (_, Some(cost_basis)) if purchased.token.is_usd => Ok(TwoAssetSwap {
          cost_basis,
          token_purchased: purchased.token.clone(),
          token_sold: sold.token.clone(),
          value_purchased,
          value_sold,
        }),
        _ => Err("Nope"),
      }
    }
}

impl From<&Vec<Transfer>> for SwapSubCategory {
  fn from(transfers: &Vec<Transfer>) -> Self {
    let two_asset: Result<TwoAssetSwap, _> = transfers.try_into();
    if let Ok(two_asset) = two_asset {
      return SwapSubCategory::TwoAsset(two_asset);
    }

    let debt: Result<DebtSwap, _> = transfers.try_into();
    if let Ok(debt) = debt {
      return SwapSubCategory::Debt(debt);
    }
    Default::default()
  }
}