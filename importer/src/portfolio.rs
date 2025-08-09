use crate::{Transaction, PortfolioItem, Token, Transfer, TransactionCategory, SwapSubCategory};
use std::collections::HashMap;
use itertools::Itertools;


pub trait ToPortfolio {
    fn to_portfolio(self) -> Vec<PortfolioItem>;
}

impl ToPortfolio for Vec<Transaction> {
    fn to_portfolio(self) -> Vec<PortfolioItem> {
      let portfolio: Vec<PortfolioItem> = vec![];

      let asset_map = self.iter().fold(HashMap::new(), |mut map: HashMap<Token, PortfolioItem>, transaction| {
        transaction.net_transfers.iter().for_each(|transfer|{
          let item = map.entry(transfer.token.clone());
          match &transaction.category {
            TransactionCategory::Swap(SwapSubCategory::TwoAsset(data)) => {
            },
            TransactionCategory::Swap(SwapSubCategory::Debt(data)) => {
            },
            _ => {
              /*let (value, cost_basis) match transfer.direction {
                TransferDirection::Incoming => 
              }*/

            },
          };
        });
        map
      });
      /*flat_map(|transaction| transaction.net_transfers.iter().map(|transfer| {
        (transfer.token.clone(), transfer.clone(), transaction.clone())
      }))
      .fold(HashMap::new(), |map, (token, transaction)|)
      ;*/

        //.collect::<HashMap<Token, Vec<Transaction, Transfer>>();
        
      panic!();
    }
}
