use std::env;

use rocket::{get, put, post};
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket_dyn_templates::Template;

use stripe::PaymentIntentId;


#[get("/")]
pub async fn index() -> Template {
    Template::render(
        "tipjar/index", context! {
            parent: "layout",
        }
    )
}

#[derive(Deserialize)]
pub struct TipjarPutArgs {
    intent_id: PaymentIntentId,
    amount: i64,
}

#[put("/tipjar", data = "<args>")]
pub async fn update_intent(args: Json<TipjarPutArgs>) {
    use stripe::{Client, PaymentIntent, UpdatePaymentIntent};

    let client = Client::new(&env::var("STRIPE_SECRET_KEY").expect("Stripe secret key not provided"));

    let mut update_args = UpdatePaymentIntent::new();
    update_args.amount = Some(args.amount);

    PaymentIntent::update(&client, &args.intent_id, update_args).await.unwrap(); 
}

#[derive(Serialize)]
pub struct IntentResponse {
    public_key: String,
    client_secret: String,
    intent_id: PaymentIntentId
}
#[post("/tipjar")]
pub async fn create_intent() -> Json<IntentResponse> {
    use stripe::{Client, PaymentIntent, CreatePaymentIntent, Currency};

    // Create a 1 USD charge here to start
    let client = Client::new(&env::var("STRIPE_SECRET_KEY").expect("Stripe secret key not provided"));
    let mut intent_args = CreatePaymentIntent::new(100, Currency::USD);
    intent_args.description = Some("Dytrich Nguyen Tipjar");
    intent_args.payment_method_types = Some(vec!["card".to_owned()]);
    let intent = PaymentIntent::create(&client, intent_args).await.unwrap();
    let res = IntentResponse {
        public_key: env::var("STRIPE_PUBLIC_KEY").unwrap(),
        client_secret: intent.client_secret.unwrap(),
        intent_id: intent.id
    };
    Json(res)
}
