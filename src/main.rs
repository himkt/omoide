use egg_mode::{KeyPair, Token, tweet::delete};
use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(name = "omoide")]
struct Opt {
    #[structopt(short, long)]
    delete: bool,

    #[structopt(short = "f", long)]
    favorite_count: i32,

    #[structopt(short, long)]
    screen_name: String,
}


fn auth(
    consumer_key: String,
    consumer_secret: String,
    access_key: String,
    access_secret: String,
) -> Token {
    let consumer_token = KeyPair::new(consumer_key, consumer_secret);
    let access_token = KeyPair::new(access_key, access_secret);
    Token::Access { consumer: consumer_token, access: access_token }
}


fn load_env(key: &str) -> String {
    match std::env::var(key) {
        Ok(val) => val,
        Err(e) => panic!(e),
    }
}


#[tokio::main]
async fn main() {
    let opt: Opt = Opt::from_args();
    let consumer_key = load_env("CONSUMER_KEY");
    let consumer_secret = load_env("CONSUMER_KEY_SECRET");
    let access_key = load_env("API_KEY");
    let access_secret = load_env("API_KEY_SECRET");

    println!(
        "consumer_key: {}\nconsumer_secret: {}\napi_key: {}\napi_secret: {}",
        &consumer_key,
        &consumer_secret,
        &access_key,
        &access_secret,
    );

    let token = auth(consumer_key, consumer_secret, access_key, access_secret);
    let user_id = egg_mode::user::show(opt.screen_name, &token).await.unwrap().response.id;

    let mut max_id: Option<u64> = None;
    let timeline = egg_mode::tweet::user_timeline(user_id, false, false, &token);

    for _ in 0..10 {
        let ret = timeline.call(None, max_id).await.unwrap();
        for status in ret.response.iter() {
            if status.favorite_count < opt.favorite_count {
                if !opt.delete {
                    println!("[dru-run] {:?}", status.text);
                }
                else {
                    delete(status.id, &token).await.unwrap();
                    println!("[deleted] {:?}", status.text);
                }
            }

            max_id = Some(status.id);
        }
    }
}
