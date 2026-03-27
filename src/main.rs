use std::process::Command;

use tiny_http::{Method, Response};

static DEFAULT_PORT: u16 = 2424;

static COMMAND_ENV_KEY: &'static str = "COMMAND";
static PORT_ENV_KEY: &'static str = "PORT";
static API_KEY_ENV_KEY: &'static str = "API_KEY";

static API_KEY_HEADER: &'static str = "Authorization";

#[derive(Debug)]
struct Env {
    port: u16,
    api_key: String,
    command: String,
}

fn parse_env() -> Env {
    let _ = dotenvy::dotenv();

    let port = match dotenvy::var(PORT_ENV_KEY) {
        Ok(port) => port
            .parse()
            .expect("Unable parse port env, make sure its a valid integer between 0 and 65535"),
        Err(_) => {
            eprintln!("Port is not in env, using default: {}", DEFAULT_PORT);
            DEFAULT_PORT
        }
    };

    let api_key = dotenvy::var(API_KEY_ENV_KEY)
        .expect("Unable to find api key in env, make sure it exists in .env file. It's required for this program to run.");

    let command = dotenvy::var(COMMAND_ENV_KEY)
        .expect("Unable to find command in env, make sure it exists in .env file. It's required for this program to run.");

    Env {
        port,
        api_key,
        command,
    }
}

fn run_server(env: &Env) {
    let server = tiny_http::Server::http(format!("0.0.0.0:{}", env.port)).expect(&format!(
        "Failed to start at :{}. Make sure port is not already taken.",
        env.port
    ));

    // start accept loop
    for req in server.incoming_requests() {
        process_request(req, env);
    }
}

fn process_request(req: tiny_http::Request, env: &Env) {
    // validate api_key
    let valid = req
        .headers()
        .iter()
        .find(|&x| x.field.equiv(API_KEY_HEADER))
        .and_then(|h| Some(h.value.as_str().eq(&env.api_key)))
        .unwrap_or(false);
    if !valid {
        let response = Response::from_string("Not authorized").with_status_code(401);
        req.respond(response).expect("Failed to respond");
        return;
    }

    // check method is post
    if req.method() != &Method::Post {
        req.respond(Response::from_string("Method Not Allowed").with_status_code(405))
            .expect("Failed to respond");
        return;
    }

    // do the magic
    let is_success = run_sh(&env.command);

    let status = match is_success {
        true => 200,
        false => 500,
    };

    req.respond(
        Response::from_string(&format!("Is success: {is_success}")).with_status_code(status),
    )
    .expect("Failed to respond");
}

fn run_sh(command: &str) -> bool {
    // dangerous moment, DO NOT EVER ACCEPT COMMAND FROM USER
    // This leads to RCE, if any part of the command are provided for user.
    // Current example works safely because of the command's origin is .env file.
    let mut out = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command.");
    let is_success = out.stderr.is_empty();
    out.stdout.extend_from_slice(b"\n\nSTDERR:\n");
    out.stdout.append(&mut out.stderr);

    println!(
        "Executed `{}`. Results:\n{}",
        command,
        String::from_utf8_lossy(&out.stdout)
    );
    is_success
}

fn main() {
    let env = parse_env();
    run_server(&env);
}
