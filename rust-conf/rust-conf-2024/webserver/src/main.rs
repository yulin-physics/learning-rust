use anyhow::Result;
use axum::routing::get;
use axum::Extension;
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::Row;
use sqlx::SqlitePool;

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct BlogPost {
    id: i32,
    date: String,
    title: String,
    body: String,
    author: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    println!("Connecting to {database_url}");

    let pool = get_connection_pool(&database_url).await?;
    run_migrations(pool.clone()).await?;

    println!("{:?}", get_blog_posts(pool.clone()).await?);

    let new_id = add_blog_post(
        pool.clone(),
        "2021-01-05".to_string(),
        "something".to_string(),
        "anything".to_string(),
        "nobody".to_string(),
    )
    .await?;
    println!("{:?}", get_blog_post(pool.clone(), new_id).await?);

    let listen_address = std::env::var("LISTEN_ADDRESS")?;
    println!("listening {listen_address}");
    let listener = tokio::net::TcpListener::bind(&listen_address).await?;

    let app = axum::Router::new()
        .route("/hello", get(say_hello))
        .route("/", get(get_blog_posts_handler))
        .route("/:id", get(get_blog_post_handler))
        .layer(Extension(pool.clone()));
    axum::serve(listener, app).await?;

    Ok(())
}

async fn say_hello() -> &'static str {
    "Hello world!"
}

async fn get_blog_posts_handler(
    Extension(pool): Extension<SqlitePool>,
) -> axum::Json<Vec<BlogPost>> {
    let posts = get_blog_posts(pool).await.unwrap();
    axum::Json(posts)
}

async fn get_blog_post_handler(
    Extension(pool): Extension<SqlitePool>,
    axum::extract::Path(id): axum::extract::Path<i32>
) -> axum::Json<BlogPost> {
    let post = get_blog_post(pool, id).await.unwrap();
    axum::Json(post)
}

async fn get_connection_pool(url: &str) -> Result<SqlitePool> {
    let connection_pool = SqlitePool::connect(url).await?;
    Ok(connection_pool)
}

async fn run_migrations(pool: SqlitePool) -> Result<()> {
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(())
}

async fn get_blog_posts(pool: SqlitePool) -> Result<Vec<BlogPost>> {
    let posts = sqlx::query_as("SELECT * FROM blog_posts")
        .fetch_all(&pool)
        .await?;
    Ok(posts)
}

async fn get_blog_post(pool: SqlitePool, id: i32) -> Result<BlogPost> {
    let posts = sqlx::query_as("SELECT * FROM blog_posts WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;
    Ok(posts)
}

async fn add_blog_post(
    pool: SqlitePool,
    date: String,
    title: String,
    body: String,
    author: String,
) -> Result<i32> {
    let id = sqlx::query("INSERT INTO blog_posts (date, title, body, author) VALUES (?, ?, ?, ?); SELECT last_insert_rowid();").
    bind(date)
    .bind(title)
    .bind(body)
    .bind(author)
    .fetch_one(&pool)
    .await?
    .get(0);

    Ok(id)
}

async fn update_blog_post(
    pool: SqlitePool,
    date: String,
    title: String,
    body: String,
    author: String,
    id: i32,
) -> Result<()> {
    sqlx::query("UPDATE blog_posts SET date = ?, title =?, body=?, author=? WHERE id = ?")
        .bind(date)
        .bind(title)
        .bind(body)
        .bind(author)
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(())
}
