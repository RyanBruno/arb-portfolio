use crate::{Transfer, SwapSubCategory, SimpleSwap, TwoAssetSwap, TransferDirection};
use rust_decimal::Decimal;
use std::str::FromStr;

/// Determines whether a set of transfers represents a simple two-token swap.
fn is_simple_swap(transfers: &[Transfer]) -> bool {
  transfers.len() == 2 && transfers.iter().filter(|x| x.token.stable_usd_value.is_some()).count() == 1
}

impl TryFrom<&Vec<Transfer>> for SimpleSwap {
    type Error = &'static str;
    fn try_from(transfers: &Vec<Transfer>) -> Result<SimpleSwap, &'static str> {
      if !is_simple_swap(&transfers) { return Err("Nope"); }

      let usd_transfer = transfers.iter().find(|x| x.token.is_usd);
      let asset_transfer = transfers.iter().find(|x| x.token.is_usd);

      match (usd_transfer, asset_transfer) {
        (Some(usd_transfer), Some(asset_transfer)) => match (usd_transfer.value, asset_transfer.value) {
          (Some(cost_basis), Some(value)) => Ok(SimpleSwap {
            cost_basis,
            direction: asset_transfer.direction.clone().into(),
            token: asset_transfer.token.clone(),
            value,
          }),
          _ => Err("Nope"),
        },
        _ => Err("Nope"),
      }
    }
}

impl TryFrom<&Vec<Transfer>> for TwoAssetSwap {
    type Error = &'static str;
    fn try_from(transfers: &Vec<Transfer>) -> Result<TwoAssetSwap, &'static str> {
      if transfers.len() != 2 { return Err("Nope"); }

      let sold = transfers.iter().find(|x| x.direction == TransferDirection::Outgoing);
      let purchased = transfers.iter().find(|x| x.direction == TransferDirection::Incoming);

      match (sold, purchased) {
        (Some(sold), Some(purchased)) => match (sold.value, purchased.value) {
          (Some(value_sold), Some(value_purchased)) => match (sold.usd_value, purchased.usd_value) {
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
          },

          _ => Err("Nope"),
        },
        _ => Err("Nope"),
      }
    }
}

impl From<&Vec<Transfer>> for SwapSubCategory {
  fn from(transfers: &Vec<Transfer>) -> Self {
    /*let simple: Result<SimpleSwap, _> = transfers.try_into();
    if let Ok(simple) = simple {
      return SwapSubCategory::Simple(simple);
    }*/
    let two_asset: Result<TwoAssetSwap, _> = transfers.try_into();
    if let Ok(two_asset) = two_asset {
      return SwapSubCategory::TwoAsset(two_asset);
    }
    Default::default()
  }
}