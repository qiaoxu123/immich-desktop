use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize,Serialize};
use serde_json::{json, Value};

#[derive(Serialize,Deserialize)]
struct User { name:String,email:String }

fn validate(base_url:&str)->Result<String,String>{
 let url=reqwest::Url::parse(base_url.trim()).map_err(|_|"Invalid server URL".to_string())?;
 if !matches!(url.scheme(), "http"|"https"){return Err("Only HTTP(S) servers are supported".into())}
 if url.username()!="" || url.password().is_some() || url.query().is_some() || url.fragment().is_some(){return Err("Unexpected credentials, query, or fragment in server URL".into())}
 let host=url.host_str().ok_or("Missing host")?;
 if url.scheme()=="http" && !matches!(host,"localhost"|"127.0.0.1"|"::1"){return Err("HTTP is only allowed for localhost; configure HTTPS for remote servers".into())}
 let trimmed=url.as_str().trim_end_matches('/').trim_end_matches("/api");
 Ok(format!("{trimmed}/api"))
}
fn client(api_key:&str)->Result<reqwest::Client,String>{if api_key.trim().is_empty(){return Err("API Key cannot be empty".into())}reqwest::Client::builder().timeout(std::time::Duration::from_secs(25)).default_headers({let mut h=reqwest::header::HeaderMap::new();h.insert("x-api-key",reqwest::header::HeaderValue::from_str(api_key).map_err(|_|"Invalid API Key".to_string())?);h}).build().map_err(|e|e.to_string())}
async fn payload(resp:reqwest::Response)->Result<Value,String>{let status=resp.status();if !status.is_success(){return Err(format!("Immich returned HTTP {}",status.as_u16()))}resp.json().await.map_err(|e|format!("Invalid Immich response: {e}"))}
#[tauri::command]
async fn connect(base_url:String,api_key:String)->Result<User,String>{let root=validate(&base_url)?;let c=client(&api_key)?;let v=payload(c.get(format!("{root}/users/me")).send().await.map_err(|e|e.to_string())?).await?;serde_json::from_value(v).map_err(|e|e.to_string())}
#[tauri::command]
async fn list_albums(base_url:String,api_key:String)->Result<Value,String>{let root=validate(&base_url)?;let c=client(&api_key)?;payload(c.get(format!("{root}/albums")).send().await.map_err(|e|e.to_string())?).await}
#[tauri::command]
async fn search_assets(base_url:String,api_key:String,cursor:Option<String>,favorite:bool)->Result<Value,String>{let root=validate(&base_url)?;let c=client(&api_key)?;let mut body=json!({"size":60,"withExif":false,"filter":{"visibility":{"eq":"timeline"}},"orderBy":{"field":"fileCreatedAt","direction":"desc"}});if favorite{body["filter"]["isFavorite"]=json!({"eq":true})}if let Some(cursor)=cursor{body["cursor"]=json!(cursor)}payload(c.post(format!("{root}/search/metadata")).json(&body).send().await.map_err(|e|e.to_string())?).await}
#[tauri::command]
async fn thumbnail(base_url:String,api_key:String,asset_id:String)->Result<String,String>{let root=validate(&base_url)?;let c=client(&api_key)?;let resp=c.get(format!("{root}/assets/{asset_id}/thumbnail?size=thumbnail")).send().await.map_err(|e|e.to_string())?;if !resp.status().is_success(){return Err(format!("Thumbnail HTTP {}",resp.status()))}let mime=resp.headers().get(reqwest::header::CONTENT_TYPE).and_then(|v|v.to_str().ok()).unwrap_or("image/jpeg").to_string();if !mime.starts_with("image/"){return Err("Expected image content".into())}let bytes=resp.bytes().await.map_err(|e|e.to_string())?;if bytes.len()>15*1024*1024{return Err("Thumbnail too large".into())}Ok(format!("data:{mime};base64,{}",STANDARD.encode(bytes)))}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(){tauri::Builder::default().invoke_handler(tauri::generate_handler![connect,list_albums,search_assets,thumbnail]).run(tauri::generate_context!()).expect("error while running Immich Desktop")}
