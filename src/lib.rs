use serde::{Deserialize};
use worker::{*, kv::{KvStore, KvError}};

mod utils;

#[derive(Deserialize)]
struct SetOption {
  ttl: Option<u64>,
}

#[derive(Deserialize)]
struct SetBody {
  key: String,
  value: String,
  options: Option<SetOption>,
}


fn log_request(req: &Request) {
  console_log!(
    "{} - [{}], located at: {:?}, within: {}",
    Date::now().to_string(),
    req.path(),
    req.cf().coordinates().unwrap_or_default(),
    req.cf().region().unwrap_or("unknown region".into())
  );
}

async fn set_value(cache: &KvStore, key: &str, value: &str, ttl: Option<u64>) -> Option<bool> {
  let put_options_builder = cache.put(key, value).ok()?;
  let put_options = match ttl {
    Some(ttl) => put_options_builder.expiration_ttl(ttl),
    None => put_options_builder,
  };
  let res = put_options.execute().await;
  return match res {
    Ok(_) => Some(true),
    Err(_) => Some(false),
  };
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
  log_request(&req);
  utils::set_panic_hook();

  let router = Router::new();

  router
    .get_async("/:key", |_req, ctx| async move {
      if let Some(key) = ctx.param("key") {
        let cache = ctx.kv("KV_CACHE")?;
        let result = cache.get(key).text().await?.ok_or(KvError::InvalidKvStore(key.clone()));
        let value = match result {
          Ok(value) => Response::ok(value),
          Err(err) => Response::error(err.to_string(), 404)
        };
        return value;
      }
      return Response::error(String::from("key not found"), 404);
    })
    .put_async("/", |mut req, ctx| async move {
      let data: SetBody = match req.json().await {
          Ok(res) => res,
          Err(_) => return Response::error("Bad request", 400),
      };
      let cache = ctx.kv("KV_CACHE")?;
      let ttl = match data.options {
        Some(options) => options.ttl,
        None => None,
      };
      let result = set_value(&cache, &data.key, &data.value, ttl).await;
      return match result {
        Some(_) => Response::ok("Value was successfully set"),
        None => Response::error("Internal server error", 500),
      };
    }
    )
    .delete_async("/:key", | _req, ctx| async move {
      if let Some(key) = ctx.param("key") {
        let cache = ctx.kv("KV_CACHE")?;
        let result = cache.delete(key).await;
        match result {
          Ok(_) => return Response::ok("Value was successfully deleted"),
          Err(err) => return Response::error(err.to_string(), 404),
        };
      }
      return Response::error(String::from("key not found"), 404);
    })
    .run(req, env)
    .await
}
