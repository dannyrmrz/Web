use crate::models::{Match, NewMatch, UpdateMatch};
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn get_matches(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, Match>("SELECT * FROM partidos")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(matches) => HttpResponse::Ok().json(matches),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_match_by_id(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    match sqlx::query_as::<_, Match>("SELECT * FROM partidos WHERE id = $1")
        .bind(id.into_inner())
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(match_) => HttpResponse::Ok().json(match_),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn create_match(pool: web::Data<PgPool>, new_match: web::Json<NewMatch>) -> impl Responder {
    match sqlx::query("INSERT INTO partidos (equipo_local, equipo_visitante, date) VALUES ($1, $2, $3) RETURNING *")
        .bind(&new_match.home_team)
        .bind(&new_match.away_team)
        .bind(&new_match.match_date)
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(match_) => HttpResponse::Created().json(match_),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_match(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    update_match: web::Json<UpdateMatch>,
) -> impl Responder {
    match sqlx::query("UPDATE partidos SET equipo_local = COALESCE($1, equipo_local), equipo_visitante = COALESCE($2, equipo_visitante), date = COALESCE($3, date) WHERE id = $4 RETURNING *")
        .bind(&update_match.home_team)
        .bind(&update_match.away_team)
        .bind(&update_match.match_date)
        .bind(id.into_inner())
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(match_) => HttpResponse::Ok().json(match_),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn delete_match(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    match sqlx::query("DELETE FROM partidos WHERE id = $1")
        .bind(id.into_inner())
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Handlers para operaciones PATCH
pub async fn register_goal(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().json(format!("Gol registrado para el partido {}", id))
}

pub async fn register_yellow_card(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().json(format!("Tarjeta amarilla registrada para el partido {}", id))
}

pub async fn register_red_card(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().json(format!("Tarjeta roja registrada para el partido {}", id))
}

pub async fn set_extra_time(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().json(format!("Tiempo extra establecido para el partido {}", id))
}