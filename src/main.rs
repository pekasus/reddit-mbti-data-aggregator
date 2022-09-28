use dotenv::dotenv;
use roux::Reddit;

fn main() {
    let me: Me = get_me();

}

fn get_me() -> Me {
    dotenv().ok();
    const USER_AGENT = "macos:roux:v2.1.0 (by /u/VladimirApputini)";

    let client_id: String = match env::var("CLIENT_ID") {
        Ok(client_id) => client_id,
        Err(e) => return println!("Couldn't read CLIENT_ID ({}) in .env file.", e),
    };
    let client_secret: String = match env::var("CLIENT_SECRET") {
        Ok(client_secret) => client_secret,
        Err(e) => return println!("Couldn't read CLIENT_SECRET ({}) in .env file.", e),
    };

    // It fetches credentials from environment first, and if not found, asks the user.
    let mut username: String = match env::var("USERNAME") {
        Ok(username) => username,
        Err(_) => {
            println!("Please input your username:");
            let mut username = String::new();
            io::stdin()
                .read_line(&mut username)
                .expect("Failed to read line");
            username
        }
    };
    let mut password: String = match env::var("PASSWORD") {
        Ok(password) => password,
        Err(_) => {
            println!("Please input your password:");
            std::io::stdout().flush().unwrap();
            let password = read_password().unwrap();
            password
        }
    };

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }

    let client = Reddit::new(USER_AGENT, &client_id, &client_secret)
        .username(&username)
        .password(&password)
        .login();
    
    let me: Me = client.unwrap();
    println!("Your access token is: {}", me.access_token);

    me
}