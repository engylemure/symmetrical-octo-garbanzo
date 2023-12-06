use actix_web::{get, post, web, HttpResponse, Responder};
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, TransactionTrait,
};
use serde::Deserialize;

use crate::entities::NumberEntity;

use crate::entities::number::{
    ActiveModel as NumberEntityActiveModel, Column as NumberEntityColumn,
    Model as NumberEntityModel,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/numbers").service(index).service(create));
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetNumbersQueryParams {
    is_prime: Option<bool>,
    limit: Option<u64>,
}

#[get("")]
async fn index(
    db_conn: web::Data<DatabaseConnection>,
    params: web::Query<GetNumbersQueryParams>,
) -> impl Responder {
    let db_conn = db_conn.as_ref();
    match NumberEntity::find()
        .apply_if(params.is_prime, |query, is_prime| {
            query.filter(NumberEntityColumn::IsPrime.eq(is_prime))
        })
        .limit(params.limit)
        .all(db_conn)
        .await
    {
        Ok(numbers) => HttpResponse::Ok().json(numbers),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize)]
struct PostNumbersPayload {
    value: u32,
}

#[post("")]
async fn create(
    db_conn: web::Data<DatabaseConnection>,
    payload: web::Json<PostNumbersPayload>,
) -> impl Responder {
    match create_numbers_with_sieve_of_eratosthenes(db_conn.as_ref(), payload.value).await {
        Ok(number) => HttpResponse::Ok().json(number),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

impl From<(usize, bool)> for NumberEntityActiveModel {
    fn from((value, is_prime): (usize, bool)) -> Self {
        NumberEntityActiveModel {
            is_prime: ActiveValue::Set(is_prime),
            value: ActiveValue::Set(value as i32),
            ..Default::default()
        }
    }
}

async fn create_numbers_with_sieve_of_eratosthenes(
    db_conn: &DatabaseConnection,
    value: u32,
) -> Result<NumberEntityModel, sea_orm::DbErr> {
    let trx = db_conn.begin().await?;
    match NumberEntity::find()
        .filter(NumberEntityColumn::Value.eq(value))
        .one(&trx)
        .await?
    {
        Some(number) => Ok(number),
        None => {
            let last_number = NumberEntity::find()
                .order_by_desc(NumberEntityColumn::Value)
                .one(&trx)
                .await?;
            let mut to_be_inserted = vec![vec![]];
            for data in sieve_of_eratosthenes(value).into_iter().enumerate() {
                if (to_be_inserted.last().unwrap().len() + 1) * 2 > (u16::MAX as usize) {
                    to_be_inserted.push(vec![]);
                }
                if let Some(last_number) = &last_number {
                    if data.0 > last_number.value as usize {
                        to_be_inserted
                            .last_mut()
                            .unwrap()
                            .push(NumberEntityActiveModel::from(data))
                    }
                } else {
                    to_be_inserted
                        .last_mut()
                        .unwrap()
                        .push(NumberEntityActiveModel::from(data))
                }
            }
            for chunk in to_be_inserted.into_iter() {
                NumberEntity::insert_many(chunk).exec(&trx).await?;
            }
            trx.commit().await?;
            Ok(NumberEntity::find()
                .filter(NumberEntityColumn::Value.eq(value as i32))
                .one(db_conn)
                .await?
                .unwrap())
        }
    }
}

fn sieve_of_eratosthenes(num: u32) -> Vec<bool> {
    let num = num as usize;
    let mut primes = vec![false; num + 1];
    let mut p: usize = 2;
    while p * p <= num + 1 {
        if primes[p] {
            for i in (p * p..(num + 1)).step_by(p) {
                primes[i] = false
            }
        }
        p += 1;
    }
    primes
}
