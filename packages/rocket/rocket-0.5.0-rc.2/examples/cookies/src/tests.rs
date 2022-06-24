use super::{rocket, session, message};
use rocket::local::blocking::{Client, LocalResponse};
use rocket::http::{Status, Cookie, ContentType};

fn user_id_cookie(response: &LocalResponse<'_>) -> Option<Cookie<'static>> {
    let cookie = response.headers()
        .get("Set-Cookie")
        .filter(|v| v.starts_with("user_id"))
        .nth(0)
        .and_then(|val| Cookie::parse_encoded(val).ok());

    cookie.map(|c| c.into_owned())
}

fn login(client: &Client, user: &str, pass: &str) -> Option<Cookie<'static>> {
    let response = client.post(session::uri!(login))
        .header(ContentType::Form)
        .body(format!("username={}&password={}", user, pass))
        .dispatch();

    user_id_cookie(&response)
}

#[test]
fn redirect_logged_out_session() {
    let client = Client::tracked(rocket()).unwrap();
    let response = client.get(session::uri!(index)).dispatch();
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location").unwrap(), &session::uri!(login));

    let response = client.get(session::uri!(login_page)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().unwrap();
    assert!(body.contains("Please login to continue."));
}

#[test]
fn login_fails() {
    let client = Client::tracked(rocket()).unwrap();
    assert!(login(&client, "Seergio", "password").is_none());
    assert!(login(&client, "Sergio", "idontknow").is_none());
}

#[test]
fn login_logout_succeeds() {
    let client = Client::tracked(rocket()).unwrap();
    let login_cookie = login(&client, "Sergio", "password").expect("logged in");

    // Ensure we're logged in.
    let response = client.get(session::uri!(index)).cookie(login_cookie.clone()).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().unwrap();
    assert!(body.contains("Logged in with user ID 1"));

    // One more.
    let response = client.get(session::uri!(login)).cookie(login_cookie.clone()).dispatch();
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location").unwrap(), &session::uri!(index));

    // Logout.
    let response = client.post(session::uri!(logout)).cookie(login_cookie).dispatch();
    let cookie = user_id_cookie(&response).expect("logout cookie");
    assert!(cookie.value().is_empty());

    // The user should be redirected back to the login page.
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location").unwrap(), &session::uri!(login));

    // The page should show the success message, and no errors.
    let response = client.get(session::uri!(login)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().unwrap();
    assert!(body.contains("success: Successfully logged out."));
    assert!(!body.contains("Error"));
}

#[test]
fn test_message() {
    let client = Client::tracked(rocket()).unwrap();

    // Check that there's no message initially.
    let response = client.get(message::uri!(index)).dispatch();
    assert!(response.into_string().unwrap().contains("No message yet."));

    // Now set a message; we should get a cookie back.
    let response = client.post(message::uri!(submit))
        .header(ContentType::Form)
        .body("message=Hello from Rocket!")
        .dispatch();

    let cookie_headers: Vec<_> = response.headers().get("Set-Cookie").collect();
    assert_eq!(cookie_headers.len(), 1);
    assert!(cookie_headers[0].starts_with("message=Hello%20from%20Rocket!"));
    assert_eq!(response.headers().get_one("Location").unwrap(), &message::uri!(index));
    assert_eq!(response.status(), Status::SeeOther);

    // Check that the message is reflected.
    let response = client.get(message::uri!(index)).dispatch();
    assert!(response.into_string().unwrap().contains("Hello from Rocket!"));
}
