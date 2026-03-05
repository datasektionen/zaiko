use actix_web::{delete, get, patch, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Row};
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use utoipa_actix_web::service_config::ServiceConfig;

use crate::error::Error;
use crate::auth::types::{Group, HivePermission};

/// Type of alcohol
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum AlcoholType {
    #[serde(rename = "cider")]
    Cider,
    #[serde(rename = "beer")]
    Beer,
    #[serde(rename = "spirits")]
    Spirits,
    #[serde(rename = "wine")]
    Wine,
}

impl ToString for AlcoholType {
    fn to_string(&self) -> String {
        match self {
            AlcoholType::Cider => "cider".to_string(),
            AlcoholType::Beer => "beer".to_string(),
            AlcoholType::Spirits => "spirits".to_string(),
            AlcoholType::Wine => "wine".to_string(),
        }
    }
}

impl std::str::FromStr for AlcoholType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cider" => Ok(AlcoholType::Cider),
            "beer" => Ok(AlcoholType::Beer),
            "spirits" => Ok(AlcoholType::Spirits),
            "wine" => Ok(AlcoholType::Wine),
            _ => Err(format!("Unknown alcohol type: {}", s)),
        }
    }
}

/// Request to create an alcohol product
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AlcoholProductCreateRequest {
    /// Item name (must be unique)
    pub item_name: String,
    /// Type of alcohol
    pub alcohol_type: AlcoholType,
    /// Volume in centiliters
    pub volume_cl: f32,
    /// Supplier name
    pub supplier: String,
}

/// Request to update alcohol inventory
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AlcoholInventoryUpdateRequest {
    /// Item name
    pub item_name: String,
    /// Current number of bottles
    pub current_bottles: f32,
    /// Previous number of bottles
    pub previous_bottles: f32,
    /// Current purchase price in SEK
    pub current_purchase_price: f32,
    /// Previous purchase price in SEK (optional)
    pub previous_purchase_price: Option<f32>,
    /// Sale price in SEK (calculated from minimum if not provided)
    pub sale_price: Option<f32>,
    /// Price per cl (only for spirits)
    pub price_per_cl: Option<f32>,
}

/// Alcohol product with current inventory
#[derive(Debug, Serialize, ToSchema)]
pub struct AlcoholProduct {
    pub item_name: String,
    pub alcohol_type: AlcoholType,
    pub volume_cl: f32,
    pub supplier: String,
    pub current_bottles: f32,
    pub previous_bottles: f32,
    pub current_purchase_price: f32,
    pub previous_purchase_price: Option<f32>,
    pub minimum_sale_price: f32,
    pub sale_price: f32,
    pub price_per_cl: Option<f32>,
    pub last_updated: Option<DateTime<Utc>>,
}

/// Report entry for a single alcohol product
#[derive(Debug, Serialize, ToSchema)]
pub struct AlcoholReportEntry {
    pub item_name: String,
    pub alcohol_type: AlcoholType,
    pub supplier: String,
    pub volume_cl: f32,
    pub current_bottles: f32,
    pub previous_bottles: f32,
    pub bottle_change: f32,
    pub current_total_value: f32,
    pub previous_total_value: f32,
    pub value_change: f32,
    pub current_purchase_price: f32,
    pub previous_purchase_price: Option<f32>,
    pub sale_price: f32,
}

/// Alcohol report grouped by type
#[derive(Debug, Serialize, ToSchema)]
pub struct AlcoholInventoryReport {
    pub report_date: DateTime<Utc>,
    pub entries: Vec<AlcoholReportEntry>,
    pub summary_by_type: Vec<AlcoholTypeSummary>,
    pub total_value: f32,
}

/// Summary for a single alcohol type
#[derive(Debug, Serialize, ToSchema)]
pub struct AlcoholTypeSummary {
    pub alcohol_type: AlcoholType,
    pub count: usize,
    pub total_bottles: f32,
    pub total_value: f32,
    pub value_change: f32,
}

/// Database operations for alcohol products
pub fn config() -> impl FnOnce(&mut ServiceConfig) {
    |cfg: &mut ServiceConfig| {
        cfg.service(create_alcohol_product_handler)
            .service(get_alcohol_products)
            .service(get_alcohol_product_handler)
            .service(update_alcohol_inventory_handler)
            .service(delete_alcohol_product_handler)
            .service(get_alcohol_report_handler);
    }
}

// API Handlers

#[utoipa::path(
    post,
    path = "/alcohol",
    tag = "alcohol",
    request_body = AlcoholProductCreateRequest,
    responses(
        (status = 200, description = "Alcohol product created", body = AlcoholProduct),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[post("/alcohol")]
async fn create_alcohol_product_handler(
    body: web::Json<AlcoholProductCreateRequest>,
    db: web::Data<Pool<Postgres>>,
    _permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    db::create_alcohol_product(
        &db,
        &body.item_name,
        &body.alcohol_type.to_string(),
        body.volume_cl,
        &body.supplier,
    )
    .await?;

    let product = db::get_alcohol_product(&db, &body.item_name)
        .await?
        .ok_or_else(|| Error::InternalServerError("Alcohol product not found".to_string()))?;

    Ok(HttpResponse::Ok().json(product))
}

#[utoipa::path(
    get,
    path = "/alcohol",
    tag = "alcohol",
    responses(
        (status = 200, description = "List of alcohol products", body = Vec<AlcoholProduct>),
        (status = 500, description = "Internal Server Error")
    )
)]
#[get("/alcohol")]
async fn get_alcohol_products(
    db: web::Data<Pool<Postgres>>,
    _permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    let products = db::list_alcohol_products(&db).await?;
    Ok(HttpResponse::Ok().json(products))
}

#[utoipa::path(
    get,
    path = "/alcohol/{item_name}",
    tag = "alcohol",
    responses(
        (status = 200, description = "Alcohol product", body = AlcoholProduct),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[get("/alcohol/{item_name}")]
async fn get_alcohol_product_handler(
    item_name: web::Path<String>,
    db: web::Data<Pool<Postgres>>,
    _permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    let product = db::get_alcohol_product(&db, &item_name)
        .await?
        .ok_or_else(|| Error::InternalServerError("Alcohol product not found".to_string()))?;

    Ok(HttpResponse::Ok().json(product))
}

#[utoipa::path(
    patch,
    path = "/alcohol/{item_name}/inventory",
    tag = "alcohol",
    request_body = AlcoholInventoryUpdateRequest,
    responses(
        (status = 200, description = "Inventory updated", body = AlcoholProduct),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[patch("/alcohol/{item_name}/inventory")]
async fn update_alcohol_inventory_handler(
    item_name: web::Path<String>,
    body: web::Json<AlcoholInventoryUpdateRequest>,
    db: web::Data<Pool<Postgres>>,
    groups: web::ReqData<Vec<Group>>,
) -> Result<HttpResponse, Error> {
    let user = groups.into_inner().first()
        .map(|g| g.0.clone())
        .unwrap_or_else(|| "unknown".to_string());

    // Calculate sale price if not provided
    let minimum_sale_price = body.current_purchase_price * 1.25;
    let sale_price = body.sale_price.unwrap_or_else(|| {
        // Round up to nearest 5
        (minimum_sale_price / 5.0).ceil() * 5.0
    });

    db::update_alcohol_inventory(
        &db,
        &item_name,
        body.current_bottles,
        body.previous_bottles,
        body.current_purchase_price,
        body.previous_purchase_price,
        sale_price,
        body.price_per_cl,
        &user,
    )
    .await?;

    let product = db::get_alcohol_product(&db, &item_name)
        .await?
        .ok_or_else(|| Error::InternalServerError("Alcohol product not found".to_string()))?;

    Ok(HttpResponse::Ok().json(product))
}

#[utoipa::path(
    delete,
    path = "/alcohol/{item_name}",
    tag = "alcohol",
    responses(
        (status = 204, description = "Alcohol product deleted"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[delete("/alcohol/{item_name}")]
async fn delete_alcohol_product_handler(
    item_name: web::Path<String>,
    db: web::Data<Pool<Postgres>>,
    _permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    db::delete_alcohol_product(&db, &item_name).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(
    get,
    path = "/alcohol/report",
    tag = "alcohol",
    responses(
        (status = 200, description = "Alcohol inventory report", body = AlcoholInventoryReport),
        (status = 500, description = "Internal Server Error")
    )
)]
#[get("/alcohol/report")]
async fn get_alcohol_report_handler(
    db: web::Data<Pool<Postgres>>,
    _permissions: web::ReqData<Vec<HivePermission>>,
) -> Result<HttpResponse, Error> {
    let report = db::get_alcohol_report(&db).await?;
    Ok(HttpResponse::Ok().json(report))
}

// Database module

pub mod db {
    use super::*;

    pub async fn create_alcohol_product(
        pool: &Pool<Postgres>,
        item_name: &str,
        alcohol_type: &str,
        volume_cl: f32,
        supplier: &str,
    ) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO alcohol_product (item_name, alcohol_type, volume_cl, supplier)
             VALUES ($1, $2::alcohol_type, $3, $4)
             ON CONFLICT (item_name) DO UPDATE SET
             alcohol_type = EXCLUDED.alcohol_type,
             volume_cl = EXCLUDED.volume_cl,
             supplier = EXCLUDED.supplier"
        )
        .bind(item_name)
        .bind(alcohol_type)
        .bind(volume_cl)
        .bind(supplier)
        .execute(pool)
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

        Ok(())
    }

    pub async fn get_alcohol_product(
        pool: &Pool<Postgres>,
        item_name: &str,
    ) -> Result<Option<AlcoholProduct>, Error> {
        let row = sqlx::query(
            "SELECT 
                ap.item_name, ap.alcohol_type, ap.volume_cl, ap.supplier,
                ai.current_bottles, ai.previous_bottles,
                ai.current_purchase_price, ai.previous_purchase_price,
                ai.minimum_sale_price, ai.sale_price, ai.price_per_cl,
                ai.last_updated
            FROM alcohol_product ap
            LEFT JOIN alcohol_inventory ai ON ap.item_name = ai.item_name
            WHERE ap.item_name = $1"
        )
        .bind(item_name)
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

        Ok(row.map(|r| {
            let alcohol_type_str: String = r.get("alcohol_type");
            AlcoholProduct {
                item_name: r.get("item_name"),
                alcohol_type: alcohol_type_str.parse().unwrap_or(AlcoholType::Beer),
                volume_cl: r.get("volume_cl"),
                supplier: r.get("supplier"),
                current_bottles: r.get("current_bottles"),
                previous_bottles: r.get("previous_bottles"),
                current_purchase_price: r.get("current_purchase_price"),
                previous_purchase_price: r.get("previous_purchase_price"),
                minimum_sale_price: r.get("minimum_sale_price"),
                sale_price: r.get("sale_price"),
                price_per_cl: r.get("price_per_cl"),
                last_updated: r.get("last_updated"),
            }
        }))
    }

    pub async fn list_alcohol_products(
        pool: &Pool<Postgres>,
    ) -> Result<Vec<AlcoholProduct>, Error> {
        let rows = sqlx::query(
            "SELECT 
                ap.item_name, ap.alcohol_type, ap.volume_cl, ap.supplier,
                COALESCE(ai.current_bottles, 0) as current_bottles,
                COALESCE(ai.previous_bottles, 0) as previous_bottles,
                COALESCE(ai.current_purchase_price, 0) as current_purchase_price,
                ai.previous_purchase_price,
                COALESCE(ai.minimum_sale_price, 0) as minimum_sale_price,
                COALESCE(ai.sale_price, 0) as sale_price,
                ai.price_per_cl,
                ai.last_updated
            FROM alcohol_product ap
            LEFT JOIN alcohol_inventory ai ON ap.item_name = ai.item_name
            ORDER BY ap.item_name"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| {
            let alcohol_type_str: String = r.get("alcohol_type");
            AlcoholProduct {
                item_name: r.get("item_name"),
                alcohol_type: alcohol_type_str.parse().unwrap_or(AlcoholType::Beer),
                volume_cl: r.get("volume_cl"),
                supplier: r.get("supplier"),
                current_bottles: r.get("current_bottles"),
                previous_bottles: r.get("previous_bottles"),
                current_purchase_price: r.get("current_purchase_price"),
                previous_purchase_price: r.get("previous_purchase_price"),
                minimum_sale_price: r.get("minimum_sale_price"),
                sale_price: r.get("sale_price"),
                price_per_cl: r.get("price_per_cl"),
                last_updated: r.get("last_updated"),
            }
        }).collect())
    }

    pub async fn update_alcohol_inventory(
        pool: &Pool<Postgres>,
        item_name: &str,
        current_bottles: f32,
        previous_bottles: f32,
        current_purchase_price: f32,
        previous_purchase_price: Option<f32>,
        sale_price: f32,
        price_per_cl: Option<f32>,
        user: &str,
    ) -> Result<(), Error> {
        // Calculate minimum sale price (purchase price × 1.25)
        let minimum_sale_price = current_purchase_price * 1.25;

        // Start transaction
        let mut tx = pool.begin().await
            .map_err(|e| Error::InternalServerError(e.to_string()))?;

        // Update current inventory
        sqlx::query(
            "INSERT INTO alcohol_inventory 
             (item_name, current_bottles, previous_bottles, current_purchase_price, 
              previous_purchase_price, minimum_sale_price, sale_price, price_per_cl, last_updated)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, CURRENT_TIMESTAMP)
             ON CONFLICT (item_name) DO UPDATE SET
             current_bottles = EXCLUDED.current_bottles,
             previous_bottles = EXCLUDED.previous_bottles,
             current_purchase_price = EXCLUDED.current_purchase_price,
             previous_purchase_price = EXCLUDED.previous_purchase_price,
             minimum_sale_price = EXCLUDED.minimum_sale_price,
             sale_price = EXCLUDED.sale_price,
             price_per_cl = EXCLUDED.price_per_cl,
             last_updated = CURRENT_TIMESTAMP"
        )
        .bind(item_name)
        .bind(current_bottles)
        .bind(previous_bottles)
        .bind(current_purchase_price)
        .bind(previous_purchase_price)
        .bind(minimum_sale_price)
        .bind(sale_price)
        .bind(price_per_cl)
        .execute(&mut *tx)
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

        // Add to audit trail
        sqlx::query(
            "INSERT INTO alcohol_inventory_history 
             (item_name, current_bottles, previous_bottles, current_purchase_price,
              previous_purchase_price, minimum_sale_price, sale_price, price_per_cl, user_)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(item_name)
        .bind(current_bottles)
        .bind(previous_bottles)
        .bind(current_purchase_price)
        .bind(previous_purchase_price)
        .bind(minimum_sale_price)
        .bind(sale_price)
        .bind(price_per_cl)
        .bind(user)
        .execute(&mut *tx)
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

        tx.commit().await
            .map_err(|e| Error::InternalServerError(e.to_string()))?;

        Ok(())
    }

    pub async fn delete_alcohol_product(
        pool: &Pool<Postgres>,
        item_name: &str,
    ) -> Result<(), Error> {
        sqlx::query("DELETE FROM alcohol_product WHERE item_name = $1")
            .bind(item_name)
            .execute(pool)
            .await
            .map_err(|e| Error::InternalServerError(e.to_string()))?;

        Ok(())
    }

    pub async fn get_alcohol_report(
        pool: &Pool<Postgres>,
    ) -> Result<AlcoholInventoryReport, Error> {
        let rows = sqlx::query(
            "SELECT 
                ap.item_name, ap.alcohol_type, ap.supplier, ap.volume_cl,
                ai.current_bottles, ai.previous_bottles,
                ai.current_purchase_price, ai.previous_purchase_price,
                ai.sale_price
            FROM alcohol_product ap
            LEFT JOIN alcohol_inventory ai ON ap.item_name = ai.item_name
            ORDER BY ap.alcohol_type, ap.item_name"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

        let mut entries = Vec::new();
        let mut type_summaries: std::collections::HashMap<String, AlcoholTypeSummary> = std::collections::HashMap::new();
        let mut total_value = 0.0;

        for row in rows {
            let alcohol_type_str: String = row.get("alcohol_type");
            let alcohol_type = alcohol_type_str.parse().unwrap_or(AlcoholType::Beer);
            
            let current_bottles: f32 = row.get("current_bottles");
            let previous_bottles: f32 = row.get("previous_bottles");
            let current_purchase_price: f32 = row.get("current_purchase_price");
            let previous_purchase_price: Option<f32> = row.get("previous_purchase_price");
            let sale_price: f32 = row.get("sale_price");

            let current_total_value = current_bottles * current_purchase_price;
            let previous_total_value = if let Some(prev_price) = previous_purchase_price {
                previous_bottles * prev_price
            } else {
                0.0
            };
            let value_change = current_total_value - previous_total_value;

            total_value += current_total_value;

            let entry = AlcoholReportEntry {
                item_name: row.get("item_name"),
                alcohol_type: alcohol_type.clone(),
                supplier: row.get("supplier"),
                volume_cl: row.get("volume_cl"),
                current_bottles,
                previous_bottles,
                bottle_change: current_bottles - previous_bottles,
                current_total_value,
                previous_total_value,
                value_change,
                current_purchase_price,
                previous_purchase_price,
                sale_price,
            };

            // Update type summary
            let type_key = alcohol_type.to_string();
            let summary = type_summaries.entry(type_key).or_insert_with(|| AlcoholTypeSummary {
                alcohol_type: alcohol_type.clone(),
                count: 0,
                total_bottles: 0.0,
                total_value: 0.0,
                value_change: 0.0,
            });
            summary.count += 1;
            summary.total_bottles += current_bottles;
            summary.total_value += current_total_value;
            summary.value_change += value_change;

            entries.push(entry);
        }

        let mut summary_by_type: Vec<AlcoholTypeSummary> = type_summaries.into_values().collect();
        summary_by_type.sort_by(|a, b| a.alcohol_type.to_string().cmp(&b.alcohol_type.to_string()));

        Ok(AlcoholInventoryReport {
            report_date: Utc::now(),
            entries,
            summary_by_type,
            total_value,
        })
    }
}
