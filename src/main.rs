use egg_mode::{KeyPair, Token, tweet::delete};
use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(name = "omoide")]
struct Opt {
    #[structopt(long)]
    delete: bool,

    #[structopt(long, default_value = "-1")]
    favorite_count: i32,

    #[structopt(long)]
    screen_name: String,

    #[structopt(long, default_value = "10")]
    per_page: i32,

    #[structopt(long, default_value = "5")]
    num_iters: u32,

    #[structopt(long)]
    verbose: bool,

    #[structopt(long)]
    include_rt: bool,

    #[structopt(long)]
    include_reply: bool,
}


trait Credential {
    fn new() -> Self;

    fn load(key: &str) -> String {
        match std::env::var(key) {
            Ok(val) => val,
            Err(e) => panic!("{}", e),
        }
    }
}


#[derive(Debug)]
struct EnvironmentVariableCredential {
    consumer_key: String,
    consumer_secret: String,
    access_key: String,
    access_secret: String,
}


impl Credential for EnvironmentVariableCredential {
    fn new() -> EnvironmentVariableCredential {
        let consumer_key = <EnvironmentVariableCredential as Credential>::load("CONSUMER_KEY");
        let consumer_secret = <EnvironmentVariableCredential as Credential>::load("CONSUMER_KEY_SECRET");
        let access_key = <EnvironmentVariableCredential as Credential>::load("API_KEY");
        let access_secret = <EnvironmentVariableCredential as Credential>::load("API_KEY_SECRET");

        EnvironmentVariableCredential {
            consumer_key,
            consumer_secret,
            access_key,
            access_secret,
        }
    }
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


#[tokio::main]
async fn main() {
    let opt: Opt = Opt::from_args();
    let credential = EnvironmentVariableCredential::new();

    if opt.verbose {
        std::dbg!(&credential);
    }

    let token = auth(
        credential.consumer_key,
        credential.consumer_secret,
        credential.access_key,
        credential.access_secret,
    );

    let user_id = egg_mode::user::show(opt.screen_name, &token).await.unwrap().response.id;

    let mut max_id: Option<u64> = None;
    let timeline = egg_mode::tweet::user_timeline(
        user_id,
        opt.include_reply,
        opt.include_rt,
        &token,
    )
        .with_page_size(opt.per_page);

    if opt.delete && opt.favorite_count == -1 {
        println!("If `--delete` is specified, you have to set `--favorite-count` as well.");
        return;
    }

    for _ in 0..opt.num_iters {
        let ret = timeline.call(None, max_id).await.unwrap();
        for status in ret.response.iter() {
            if opt.favorite_count < 0 {
                println!("{}", status.text);
            }
            else if status.favorite_count < opt.favorite_count {
                if !opt.delete {
                    println!("[dru-run] {:?}", status.text);
                }
                else {
                    delete(status.id, &token).await.unwrap();
                    println!("[deleted] {:?}", status.text);
                }
            }

            max_id = Some(status.id - 1);
        }
    }
}
