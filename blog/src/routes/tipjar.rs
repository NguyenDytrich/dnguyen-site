use std::env;
use std::str::FromStr;

use rocket::{get, put, uri};
use rocket::http::{Cookie, CookieJar, Status};
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

async fn create_intent(client: stripe::Client) -> IntentResponse {
    use stripe::{PaymentIntent, CreatePaymentIntent, Currency};

    // Create a 1 USD charge here to start
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

async fn retreive_intent(client: stripe::Client, intent_id_str: &str) -> IntentResponse {
    let pid = stripe::PaymentIntentId::from_str(intent_id_str).unwrap();
    let res = stripe::PaymentIntent::retrieve(&client, &pid, &[]).await.unwrap();

    IntentResponse {
        public_key: env::var("STRIPE_PUBLIC_KEY").expect("Stripe public key not provided"),
        client_secret: res.client_secret.unwrap(),
        intent_id: res.id
    }
}

#[get("/")]
pub async fn index(cookies: &CookieJar<'_>) -> Template {

    let client = stripe::Client::new(&env::var("STRIPE_SECRET_KEY").expect("Stripe secret key not provided"));
    let cookie = cookies.get_private("payment_intent");

    let client_secret = match cookie {
        Some(c) => {
            let res = retreive_intent(client, c.value()).await;
            res.client_secret
        }
        None => {
            let res = create_intent(client).await;
            cookies.add_private(Cookie::new("payment_intent", res.intent_id.to_string()));
            res.client_secret
        }
    };

    Template::render(
        "tipjar/index", context! {
            parent: "layout",
            client_secret: client_secret
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

#[get("/thanks?<pid>")]
pub fn thanks(pid: String) -> Template {
    // TODO check if valid PID
    // let pid_exists = {{db lookup pid}}
    // match pid_exists {
    //  true =>
    Template::render(
        "tipjar/thanks", context! {
            parent: "layout",
        }
    )
    // false => Redirect::to(uri!("/"))
    // }
}

#[derive(FromForm)]
pub struct StripeResponse<'r> {
    payment_intent: &'r str
}

#[get("/redirect/payment?<stripe_response..>")]
pub fn post_payment(stripe_response: StripeResponse<'_>, cookies: &CookieJar<'_>) -> Redirect {
    let pid = stripe_response.payment_intent;

    // Remove cookie from the cookie jar on success.
    cookies.remove_private(Cookie::named("payment_intent"));
    // TODO check payment status, then pass enum to thanks()
    Redirect::to(uri!(thanks(pid)))
}
