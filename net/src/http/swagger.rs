//! # swagger
//!
//! swagger 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Swagger UI integration

use axum::response::{Html, Json};
use axum::routing::get;
use axum::Router;
use utoipa::openapi::OpenApi;

/// Create Swagger UI router
pub fn create_swagger_ui_router(openapi: OpenApi, path: &str) -> Router {
    let openapi_json = serde_json::to_value(&openapi).unwrap_or_default();
    let swagger_path = format!("{}/", path);
    let json_path = format!("{}/openapi.json", path);
    
    Router::new()
        .route(&swagger_path, get({
            let openapi_url = json_path.clone();
            move || {
                let url = openapi_url.clone();
                async move { create_swagger_ui_html(&url) }
            }
        }))
        .route(&json_path, get({
            let json = openapi_json.clone();
            move || {
                let json_val = json.clone();
                async move { Json(json_val) }
            }
        }))
}

/// Create a simple Swagger UI HTML page
pub fn create_swagger_ui_html(openapi_url: &str) -> Html<String> {
    let html = format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>API Reference</title>
    <meta charset="utf-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui.css" />
    <style>
        html {{
            box-sizing: border-box;
            overflow: -moz-scrollbars-vertical;
            overflow-y: scroll;
        }}
        *, *:before, *:after {{
            box-sizing: inherit;
        }}
        body {{
            margin:0;
            background: #fafafa;
        }}
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-bundle.js"></script>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-standalone-preset.js"></script>
    <script>
        window.onload = function() {{
            const ui = SwaggerUIBundle({{
                url: "{}",
                dom_id: '#swagger-ui',
                deepLinking: true,
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIStandalonePreset
                ],
                plugins: [
                    SwaggerUIBundle.plugins.DownloadUrl
                ],
                layout: "StandaloneLayout"
            }});
        }};
    </script>
</body>
</html>
"#,
        openapi_url
    );
    Html(html)
}

/// Create ReDoc HTML page
pub fn create_redoc_html(openapi_url: &str) -> Html<String> {
    let html = format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>API Reference</title>
    <meta charset="utf-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <style>
        body {{
            margin: 0;
            padding: 0;
        }}
    </style>
</head>
<body>
    <redoc spec-url="{}" show-object-schema-examples="true"></redoc>
    <script src="https://cdn.redoc.ly/redoc/latest/bundles/redoc.standalone.js"></script>
</body>
</html>
"#,
        openapi_url
    );
    Html(html)
}

