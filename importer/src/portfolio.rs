use std::collections::{HashMap, VecDeque};
use rust_decimal::Decimal;
use serde::Serialize;

use crate::{Transfer, Token};

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct CostBasis {
    datetime: String,
    amount: Decimal,
    usd_value: Decimal,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Sale {
    pub datetime: String,
    pub token: Token,
    /// Amount of token sold (always positive)
    pub amount: Decimal,
    /// USD proceeds for the sold amount (always positive)
    pub proceeds_usd: Decimal,
    /// USD cost basis for the sold amount (always positive)
    pub cost_basis_usd: Decimal,
    /// Realized gain (proceeds - cost_basis)
    pub gain_usd: Decimal,
}

/// Convert a list of `Transfer` records into realized `Sale` events.
///
/// Each purchase (positive value) adds to the FIFO cost basis for the
/// corresponding token. Each sale (negative value) consumes from the
/// earliest cost basis lots and generates one or more `Sale` records.
pub fn transfers_to_sales(transfers: Vec<Transfer>) -> Vec<Sale> {
    let mut basis_map: HashMap<Token, VecDeque<CostBasis>> = HashMap::new();
    let mut sales: Vec<Sale> = Vec::new();

    for transfer in transfers.into_iter() {
        if transfer.value.is_sign_positive() {
            if let Some(usd_value) = transfer.usd_value {
                let entry = basis_map.entry(transfer.token.clone()).or_default();
                entry.push_back(CostBasis {
                    datetime: transfer.datetime.clone(),
                    amount: transfer.value,
                    usd_value,
                });
            }
        } else if transfer.value.is_sign_negative() {
            let token = transfer.token.clone();
            let sale_amount = transfer.value.abs();
            let proceeds_total = transfer.usd_value.unwrap_or_default().abs();

            let price_per_token = if sale_amount.is_zero() {
                Decimal::ZERO
            } else {
                proceeds_total / sale_amount
            };

            let queue = basis_map.entry(token.clone()).or_default();
            let mut remaining = sale_amount;
            while remaining > Decimal::ZERO {
                if queue.is_empty() {
                    // No cost basis available, treat basis as zero.
                    let proceeds = price_per_token * remaining;
                    sales.push(Sale {
                        datetime: transfer.datetime.clone(),
                        token: token.clone(),
                        amount: remaining,
                        proceeds_usd: proceeds,
                        cost_basis_usd: Decimal::ZERO,
                        gain_usd: proceeds,
                    });
                    break;
                }

                let lot = queue.front_mut().expect("queue not empty");
                if lot.amount <= remaining {
                    let amount = lot.amount;
                    let cost_basis = lot.usd_value;
                    let proceeds = price_per_token * amount;
                    sales.push(Sale {
                        datetime: transfer.datetime.clone(),
                        token: token.clone(),
                        amount,
                        proceeds_usd: proceeds,
                        cost_basis_usd: cost_basis,
                        gain_usd: proceeds - cost_basis,
                    });
                    remaining -= amount;
                    queue.pop_front();
                } else {
                    let amount = remaining;
                    let cost_basis = (lot.usd_value * amount) / lot.amount;
                    let proceeds = price_per_token * amount;
                    sales.push(Sale {
                        datetime: transfer.datetime.clone(),
                        token: token.clone(),
                        amount,
                        proceeds_usd: proceeds,
                        cost_basis_usd: cost_basis,
                        gain_usd: proceeds - cost_basis,
                    });

                    // Reduce the lot by the consumed portion
                    lot.amount -= amount;
                    lot.usd_value -= cost_basis;
                    remaining = Decimal::ZERO;
                }
            }
        }
    }

    sales
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn dummy_token() -> Token {
        Token {
            symbol: "ABC".to_string(),
            stable_usd_value: None,
            address: "0x0".to_string(),
            is_usd: false,
        }
    }

    #[test]
    fn single_lot_sale() {
        let token = dummy_token();
        let buy = Transfer {
            transfer_id: "1".into(),
            datetime: "2020-01-01".into(),
            token: token.clone(),
            value: Decimal::from_str("10").unwrap(),
            usd_value: Some(Decimal::from_str("100").unwrap()),
        };
        let sell = Transfer {
            transfer_id: "2".into(),
            datetime: "2020-01-02".into(),
            token: token.clone(),
            value: Decimal::from_str("-4").unwrap(),
            usd_value: Some(Decimal::from_str("-80").unwrap()),
        };

        let sales = transfers_to_sales(vec![buy, sell]);
        assert_eq!(sales.len(), 1);
        let s = &sales[0];
        assert_eq!(s.amount, Decimal::from_str("4").unwrap());
        assert_eq!(s.cost_basis_usd, Decimal::from_str("40").unwrap());
        assert_eq!(s.proceeds_usd, Decimal::from_str("80").unwrap());
        assert_eq!(s.gain_usd, Decimal::from_str("40").unwrap());
    }

    #[test]
    fn multi_lot_sale_split() {
        let token = dummy_token();
        let buy1 = Transfer {
            transfer_id: "1".into(),
            datetime: "2020-01-01".into(),
            token: token.clone(),
            value: Decimal::from_str("4").unwrap(),
            usd_value: Some(Decimal::from_str("40").unwrap()),
        };
        let buy2 = Transfer {
            transfer_id: "2".into(),
            datetime: "2020-01-03".into(),
            token: token.clone(),
            value: Decimal::from_str("6").unwrap(),
            usd_value: Some(Decimal::from_str("90").unwrap()),
        };
        let sell = Transfer {
            transfer_id: "3".into(),
            datetime: "2020-01-04".into(),
            token: token.clone(),
            value: Decimal::from_str("-8").unwrap(),
            usd_value: Some(Decimal::from_str("-160").unwrap()),
        };

        let sales = transfers_to_sales(vec![buy1, buy2, sell]);
        assert_eq!(sales.len(), 2);

        assert_eq!(sales[0].amount, Decimal::from_str("4").unwrap());
        assert_eq!(sales[0].cost_basis_usd, Decimal::from_str("40").unwrap());
        assert_eq!(sales[0].proceeds_usd, Decimal::from_str("80").unwrap());
        assert_eq!(sales[0].gain_usd, Decimal::from_str("40").unwrap());

        assert_eq!(sales[1].amount, Decimal::from_str("4").unwrap());
        assert_eq!(sales[1].cost_basis_usd, Decimal::from_str("60").unwrap());
        assert_eq!(sales[1].proceeds_usd, Decimal::from_str("80").unwrap());
        assert_eq!(sales[1].gain_usd, Decimal::from_str("20").unwrap());
    }
}

