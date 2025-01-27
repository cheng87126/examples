use std::collections::HashMap;
use actix_identity::Identity;
use actix_web::{
    error, get, post,
    web::{self},
    Either, Error, Responder,
};
use chrono::naive::NaiveDate;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::AppState;

#[derive(Deserialize)]
struct GetFundName {
    code: String,
}
#[derive(Deserialize)]
struct FundQuery {
    id: i32,
}
#[derive(Debug, Clone, Deserialize)]
struct ResDataItem {
    date: String,
    // nav : String,
    // percentage : String,
    value: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ResData {
    // current_page : i32,
    items: Vec<ResDataItem>,
    // size : i32,
    // total_items : i32,
    // total_pages : i32
}
#[derive(Debug, Clone, Deserialize)]
pub struct Res<T> {
    data: T,
    // result_code: i32
}
#[derive(Deserialize)]
struct AddFund {
    pub fund_name: String,
    pub code: String,
    pub buy_date: NaiveDate,
    pub price: Decimal,
    pub amount: Decimal,
    pub tranche: Decimal,
}
#[derive(Deserialize)]
struct UpdateFund {
    pub id: i32,
    pub fund_name: String,
    pub code: String,
    pub buy_date: NaiveDate,
    pub price: Decimal,
    pub amount: Decimal,
    pub tranche: Decimal,
}
#[derive(Debug, FromRow)]
struct Fund {
    pub id: i32,
    pub fund_name: String,
    pub code: String,
    pub buy_date: NaiveDate,
    pub price: Decimal,
    pub amount: Decimal,
    pub tranche: Decimal,
}
impl Fund {
    fn calc(&self,res:&Res<ResData>) -> ResFund {
        let start_time = self.buy_date;
        let end_time: NaiveDate =
            NaiveDate::parse_from_str(&res.data.items[0].date, "%Y-%m-%d").unwrap();
        let diff_day = (end_time - start_time).num_days();
        let new_val = Decimal::from_str(&res.data.items[0].value).unwrap();
        let _old_val = self.price;
        let n = self.tranche;
        let total = new_val * n - self.amount;
        let unit = dec!(10000) / self.amount * total / Decimal::from_i64(diff_day).unwrap();
        let year = unit * dec!(365) / dec!(100);

        return ResFund {
            id: self.id,
            code: self.code.to_owned(),
            name: self.fund_name.to_owned(),
            buy_date: self.buy_date,
            date: end_time,
            amount: self.amount,
            total,
            unit,
            year,
        };
    }
}
#[derive(Debug, FromRow, Serialize)]
struct FundDetail {
    pub id: i32,
    pub fund_name: String,
    pub code: String,
    pub buy_date: NaiveDate,
    pub price: Decimal,
    pub amount: Decimal,
    pub tranche: Decimal,
}
#[derive(Serialize)]
struct ResFund {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub buy_date: NaiveDate,
    date: NaiveDate,
    pub amount: Decimal,
    pub total: Decimal,
    pub unit: Decimal,
    pub year: Decimal,
}
#[derive(Deserialize, Serialize)]
struct FdName {
    fd_name: String,
}
#[derive(Deserialize)]
struct NameRes {
    data: Vec<FdName>,
}

pub fn create_fund_cache()->HashMap<String, Res<ResData>>{
    let cache:HashMap<String, Res<ResData>> = HashMap::new();
    cache
}

#[post("/addFund")]
pub async fn add_fund(
    state: web::Data<AppState>,
    identity: Option<Identity>,
    json: web::Json<AddFund>,
) -> impl Responder {
    if let Some(user) = identity {
        let id = user.id().unwrap();
        sqlx::query("INSERT INTO funds(code,buy_date,price,amount,tranche,user_id,fund_name) VALUES ($1,$2,$3,$4,$5,$6,$7)")
            .bind(&json.code)
            .bind(&json.buy_date)
            .bind(&json.price)
            .bind(&json.amount)
            .bind(&json.tranche)
            .bind(id.parse::<i32>().unwrap())
            .bind(&json.fund_name)
            .execute(&state.pool)
            .await
            .unwrap();
        format!("success {}", user.id().unwrap())
    } else {
        "fail".to_owned()
    }
}
#[get("/getFunds")]
pub async fn get_funds(
    state: web::Data<AppState>,
    identity: Option<Identity>,
) -> Result<web::Json<Vec<ResFund>>, Error> {
    if let Some(user) = identity {
        let id = user.id().unwrap();
        let rows = sqlx::query_as::<_, Fund>("SELECT * FROM funds WHERE user_id = $1")
            .bind(id.parse::<i32>().unwrap())
            .fetch_all(&state.pool)
            .await
            .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
        let mut codes = HashMap::new();
        let mut funds: Vec<ResFund> = Vec::new();
        for r in rows {
            if let Some(val) = codes.get(&r.code){
                funds.push(r.calc(val));
                continue;
            }

            let url = format!(
                "https://danjuanfunds.com/djapi/fund/nav/history/{}?page=1&size=1",
                r.code
            );
            let resp = reqwest::get(url)
                .await
                .unwrap()
                .json::<Res<ResData>>()
                .await
                .unwrap();
            if resp.data.items.len() == 0 {
                continue;
            }
            codes.insert(r.code.to_owned(), resp.clone());
            funds.push(r.calc(&resp));
            /*
            let start_time = r.buy_date;
            let end_time: NaiveDate =
                NaiveDate::parse_from_str(&resp.data.items[0].date, "%Y-%m-%d").unwrap();
            let diff_day = (end_time - start_time).num_days();
            let new_val = Decimal::from_str(&resp.data.items[0].value).unwrap();
            let _old_val = r.price;
            let n = r.tranche;
            let total = new_val * n - r.amount;
            let unit = dec!(10000) / r.amount * total / Decimal::from_i64(diff_day).unwrap(); //(new_val - old_val) / Decimal::from_i64(diff_day).unwrap() * dec!(10000);
            let year = unit * dec!(365) / dec!(100);
            funds.push(ResFund {
                id: r.id,
                code: r.code,
                name: r.fund_name,
                buy_date: r.buy_date,
                date: end_time,
                amount: r.amount,
                total,
                unit,
                year,
            });
            */
        }
        Ok(web::Json(funds))
    } else {
        let v: Vec<ResFund> = Vec::new();
        Ok(web::Json(v))
    }
}
#[post("/updateFund")]
pub async fn update_fund(
    state: web::Data<AppState>,
    identity: Option<Identity>,
    json: web::Json<UpdateFund>,
) -> Result<impl Responder, Error> {
    if let Some(user) = identity {
        let id = user.id().unwrap();
        sqlx::query("UPDATE funds SET code = $1, buy_date = $2, price = $3, amount = $4, tranche = $5, fund_name = $7 WHERE id = $6 AND user_id = $8")
            .bind(&json.code)
            .bind(&json.buy_date)
            .bind(&json.price)
            .bind(&json.amount)
            .bind(&json.tranche)
            .bind(&json.id)
            .bind(&json.fund_name)
            .bind(id.parse::<i32>().unwrap())
            .execute(&state.pool)
            .await
            .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
        Ok("success".to_owned())
    } else {
        Ok("fail".to_owned())
    }
}

type FundResult = Either<web::Json<FundDetail>, &'static str>;
#[get("/getFund")]
pub async fn get_fund(
    state: web::Data<AppState>,
    identity: Option<Identity>,
    query: web::Query<FundQuery>,
) -> Result<FundResult, Error> {
    if let Some(user) = identity {
        let row = sqlx::query_as("SELECT * FROM funds WHERE id = $1 AND user_id = $2")
            .bind(query.id)
            .bind(user.id().unwrap().parse::<i32>().unwrap())
            .fetch_one(&state.pool)
            .await
            .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
        Ok(Either::Left(web::Json(row)))
    } else {
        Ok(Either::Right("fail"))
    }
}
#[get("/getFundName")]
pub async fn get_fund_name(query: web::Query<GetFundName>) -> Either<String, web::Json<FdName>> {
    let name_url = format!(
        "https://fund.xueqiu.com/dj/open/fund/deriveds?codes={}",
        query.code
    );
    let name_res = reqwest::get(name_url)
        .await
        .unwrap()
        .json::<NameRes>()
        .await
        .unwrap();
    if name_res.data.len() == 0 {
        return Either::Left("not found".to_owned());
    }
    Either::Right(web::Json(FdName {
        fd_name: name_res.data[0].fd_name.to_owned(),
    }))
}
