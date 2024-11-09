use candid::{CandidType, Principal};
use ic_cdk::export::candid::candid_method;
use ic_cdk::api;
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

type UserId = Principal;

#[derive(Clone, Debug, CandidType, Deserialize)]
struct HeaderField(String, String);

#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpRequest {
    method: String,
    url: String,
    headers: Vec<HeaderField>,
    body: Vec<u8>,
}

#[derive(Clone, Debug, CandidType,Deserialize)]
struct HttpResponse {
    status_code: u16,
    headers: Vec<HeaderField>,
    body: Vec<u8>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct UserProfile {
    pub principal_id: UserId,
    pub is_authenticated: bool,
    pub timestamp: u64,
}

thread_local! {
    static USERS: RefCell<HashMap<UserId, UserProfile>> = RefCell::new(HashMap::new());
}

// Add security headers function
fn get_security_headers() -> Vec<HeaderField> {
    vec![
        HeaderField(
            "Content-Security-Policy".to_string(),
            "default-src 'self' 'unsafe-eval' 'unsafe-inline' https://identity.ic0.app blob:; \
             connect-src 'self' https://identity.ic0.app https://*.ic0.app http://localhost:* \
             https://ic0.app https://*.ic0.app; \
             img-src 'self' data: blob:; \
             script-src 'self' 'unsafe-eval' 'unsafe-inline' blob: https://identity.ic0.app; \
             style-src 'self' 'unsafe-inline'; \
             frame-src 'self' https://identity.ic0.app blob:;".to_string(),
        ),
        HeaderField("X-Frame-Options".to_string(), "DENY".to_string()),
        HeaderField("X-Content-Type-Options".to_string(), "nosniff".to_string()),
        HeaderField(
            "Strict-Transport-Security".to_string(),
            "max-age=31536000; includeSubDomains".to_string(),
        ),
        HeaderField("Access-Control-Allow-Origin".to_string(), "*".to_string()),
        HeaderField(
            "Access-Control-Allow-Methods".to_string(), 
            "GET, POST, OPTIONS".to_string()
        ),
        HeaderField(
            "Access-Control-Allow-Headers".to_string(),
            "Content-Type, Authorization".to_string()
        ),
    ]
}

#[update]
#[candid_method(update)]
fn login() -> UserProfile {
    let principal_id = api::caller();
    let timestamp = api::time();

    let user = UserProfile {
        principal_id,
        is_authenticated: true,
        timestamp,
    };

    USERS.with(|users| {
        users.borrow_mut().insert(principal_id, user.clone());
    });

    user
}

#[query]
#[candid_method(query)]
fn get_user() -> Option<UserProfile> {
    let principal_id = api::caller();

    USERS.with(|users| {
        if let Some(user) = users.borrow().get(&principal_id) {
            if is_session_valid() {
                Some(user.clone())
            } else {
                None
            }
        } else {
            None
        }
    })
}

#[update]
#[candid_method(update)]
fn logout() -> bool {
    let principal_id = api::caller();

    USERS.with(|users| {
        if let Some(user) = users.borrow_mut().get_mut(&principal_id) {
            user.is_authenticated = false;
            true
        } else {
            false
        }
    })
}

#[query]
#[candid_method(query)]
fn is_session_valid() -> bool {
    let principal_id = api::caller();
    const SESSION_DURATION: u64 = 24 * 60 * 60 * 1_000_000_000; // 24 hours in nanoseconds
    
    USERS.with(|users| {
        if let Some(user) = users.borrow().get(&principal_id) {
            let current_time = api::time();
            user.is_authenticated && (current_time - user.timestamp) < SESSION_DURATION
        } else {
            false
        }
    })
}

#[query]
#[candid_method(query)]
fn http_request(request: HttpRequest) -> HttpResponse {
    // Get base security headers
    let mut headers = get_security_headers();
    
    match (request.method.as_str(), request.url.as_str()) {
        ("GET", "/") => {
            headers.push(HeaderField("Content-Type".to_string(), "text/plain".to_string()));
            HttpResponse {
                status_code: 200,
                headers,
                body: "Welcome to my canister!".as_bytes().to_vec(),
            }
        }
        ("POST", "/login") => {
            headers.push(HeaderField("Content-Type".to_string(), "application/json".to_string()));
            HttpResponse {
                status_code: 200,
                headers,
                body: b"{\"status\": \"Please use the login() method instead\"}".to_vec(),
            }
        }
        ("OPTIONS", _) => {
            // Handle CORS preflight requests
            HttpResponse {
                status_code: 204,
                headers,
                body: vec![],
            }
        }
        _ => {
            headers.push(HeaderField("Content-Type".to_string(), "text/plain".to_string()));
            HttpResponse {
                status_code: 404,
                headers,
                body: "Not Found".as_bytes().to_vec(),
            }
        }
    }
}

// Add pre-flight CORS handling
#[query]
#[candid_method(query)]
fn http_request_options(request: HttpRequest) -> HttpResponse {
    let headers = get_security_headers();
    HttpResponse {
        status_code: 204,
        headers,
        body: vec![],
    }
}