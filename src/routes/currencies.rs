use rocket_contrib::json::Json;

use crate::routes::route_objects::currencies_response::{
    CurrenciesResponse, CurrencyResponseEntry,
};
use crate::routes::route_objects::error_response::ErrorResponse;

#[get("/currencies", format = "json")]
pub fn list_currencies<'r>() -> Result<Json<CurrenciesResponse<'r>>, ErrorResponse<'r>> {
    let entries = crate::handlers::currencies::AVAILABLE_CURRENCIES
        .iter()
        .map(|s| (*s).into())
        .collect();
    return Result::Ok(Json(CurrenciesResponse {
        currencies: entries,
    }));
}
