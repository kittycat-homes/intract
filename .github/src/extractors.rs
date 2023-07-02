use aide::operation::OperationIo;
use axum_macros::FromRequest;
use serde::Serialize;

#[derive(OperationIo, FromRequest)]
#[from_request(via(axum_jsonschema::Json))]
#[aide(
    input_with = "axum_jsonschema::Json<T>",
    output_with = "axum_jsonschema::Json<T>",
    json_schema
)]
pub struct Json<T>(pub T);

impl<T> axum::response::IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}
