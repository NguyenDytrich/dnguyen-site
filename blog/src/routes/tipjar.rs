use std::env;
use std::str::FromStr;

use rocket::{get, put, uri};
use rocket::http::Status;
use rocket::form::{FromForm};
use rocket::response::Redirect;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket_dyn_templates::Template;

use stripe::PaymentIntentId;

#[derive(Serialize)]
pub struct IntentResponse {
    public_key: String,
    client_secret: String,
    intent_id: PaymentIntentId
}

async fn create_intent() -> IntentResponse {
    use stripe::{Client, PaymentIntent, CreatePaymentIntent, Currency};

    // Create a 1 USD charge here to start
    let client = Client::new(&env::var("STRIPE_SECRET_KEY").expect("Stripe secret key not provided"));
    let mut intent_args = CreatePaymentIntent::new(100, Currency::USD);
    intent_args.description = Some("Dytrich Nguyen Tipjar");
    intent_args.payment_method_types = Some(vec!["card".to_owned()]);
    let intent = PaymentIntent::create(&client, intent_args).await.unwrap();

    IntentResponse {
        public_key: env::var("STRIPE_PUBLIC_KEY").expect("Stripe public key not provided"),
        client_secret: intent.client_secret.unwrap(),
        intent_id: intent.id
    }
}

#[get("/")]
pub async fn index() -> Template {
    let api_res = create_intent().await;
    Template::render(
        "tipjar/index", context! {
            parent: "layout",
            client_secret: api_res.client_secret
        }
    )
}

#[derive(Deserialize)]
pub struct TipjarPutArgs<'r> {
    client_secret: &'r str,
    amount: i64,
}

#[put("/tipjar", data = "<args>")]
pub async fn update_intent(args: Json<TipjarPutArgs<'_>>) -> Status {
    use stripe::{Client, PaymentIntent, UpdatePaymentIntent};

    let client = Client::new(&env::var("STRIPE_SECRET_KEY").expect("Stripe secret key not provided"));

    // Split and extract the payment id
    let splits: Vec<&str> = args.client_secret.split('_').collect();
    let id_str = splits[0..=1].join("_");
    let intent_id = PaymentIntentId::from_str(&id_str).unwrap();

    let mut update_args = UpdatePaymentIntent::new();
    update_args.amount = Some(args.amount);

    match PaymentIntent::update(&client, &intent_id, update_args).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::BadRequest
    }

}



#[get("/thanks")]
pub fn thanks() -> Template {
    Template::render(
        "tipjar/thanks", context! {
            parent: "layout",
        }
    )
}

#[derive(FromForm)]
pub struct StripeResponse<'r> {
    payment_intent: &'r str
}

#[get("/redirect/payment?<stripe_response..>")]
pub fn complete_payment(stripe_response: StripeResponse<'_>) -> Redirect {
    println!("{:?} completed", stripe_response.payment_intent);
    Redirect::to(uri!(thanks()))
}
