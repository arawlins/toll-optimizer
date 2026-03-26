# Authentication & Social Login Strategy

This document outlines the current authentication architecture and provides two paths for adding social logins (Google and Microsoft).

## 1. Current Architecture Overview

The Toll Optimizer currently uses a custom **JWT-based** authentication system.

- **Backend (Rust/Axum)**:
  - **Password Hashing**: Uses the `argon2` crate for secure hashing.
  - **JWT Management**: Uses the `jsonwebtoken` crate to create and verify tokens signed with a local `JWT_SECRET`.
  - **Token Extraction**: A custom Axum extractor (`Claims`) validates the `Bearer` token in the `Authorization` header for protected routes.
- **Frontend (React)**:
  - **State Management**: `Zustand` store saves the JWT and User object to `localStorage`.
  - **API Requests**: An `axios` interceptor automatically attaches the stored token to every request.

---

## 2. Option 1: The "Simple" Path (Recommended)
**Approach**: Use an external Authentication-as-a-Service (AuthaaS) provider like **Supabase Auth** or **Clerk**.

### How it Works:
1. **Frontend**: Replace the manual login form with a pre-built UI component (e.g., `@supabase/auth-ui-react`).
2. **Login Flow**:
   - The user clicks "Sign in with Google."
   - The provider handles the entire OAuth 2.0 flow and returns a JWT to the frontend.
3. **Backend Integration**: 
   - Your Rust API receives this "foreign" JWT.
   - Instead of verifying against a local `JWT_SECRET`, the `Auth::decode_jwt` function is updated to verify the signature using the provider's public key (via a JWKS endpoint).
4. **User Sync**: On the first login, the Rust backend extracts the user's email and unique ID from the token and creates a record in the local `users` table.

### Code Changes (Backend):
You would update `crates/api/src/auth/mod.rs` to fetch the provider's public keys and use them for validation:

```rust
// In crates/api/src/auth/mod.rs
pub fn decode_external_jwt(token: &str) -> Result<Claims, Error> {
    // 1. Fetch public keys from Supabase/Clerk (cached)
    // 2. Decode and verify the token signature
    // 3. Extract the 'sub' and 'email' claims
}
```

---

## 3. Option 2: The "Manual" Path (Custom OAuth 2.0 Logic)
**Approach**: Implement the full OAuth 2.0 flow directly in your Rust backend and React frontend.

### What the Flow Looks Like:

1. **Authorization Request**:
   - Frontend: User clicks "Sign in with Google."
   - Frontend redirects the user to Google's Authorization Server with your `client_id`, `redirect_uri`, and a `state` parameter (to prevent CSRF).
   - URL looks like: `https://accounts.google.com/o/oauth2/v2/auth?client_id=...&redirect_uri=...&scope=openid email profile&state=random_string`

2. **User Consent**:
   - User logs in to Google and approves your application.

3. **Authorization Callback**:
   - Google redirects the user back to your frontend/backend callback URL with a temporary `code`:
   - URL: `https://your-app.com/auth/callback?code=4/0AfgeX...&state=random_string`

4. **Token Exchange**:
   - Your backend receives the `code`.
   - Backend makes a **POST** request to Google's Token Server (`https://oauth2.googleapis.com/token`) sending:
     - `code`
     - `client_id`
     - `client_secret` (Secret! Never share with frontend)
     - `grant_type=authorization_code`
   - Google returns an `access_token` and an `id_token` (JWT).

5. **Token Validation & User Profile**:
   - Your backend validates the `id_token` signature.
   - It extracts the user's Google ID, email, and name.

6. **Session Creation**:
   - Your backend creates a local session/JWT and sends it to the frontend.

### Rust Implementation Details:
You would likely use the `oauth2` and `openidconnect` crates.

```rust
// Example logic in Rust
let client = CoreClient::discover(
    &IssuerUrl::new("https://accounts.google.com".to_string())?,
    &ClientId::new(client_id),
    &ClientSecret::new(client_secret),
    &RedirectUrl::new(redirect_uri)?,
)?;

// Generate the authorization URL
let (auth_url, csrf_state, nonce) = client
    .authorize_url(
        AuthenticationContextClass::default,
        CsrfToken::new_random,
        Nonce::new_random,
    )
    .add_scope(Scope::new("email".to_string()))
    .url();
```

### Why this is complex:
- **State Management**: You must securely store the `state` and `nonce` parameters between the redirect and the callback.
- **Provider Differences**: Google and Microsoft have slight variations in their OIDC implementations.
- **Maintenance**: You are responsible for rotating secrets, handling token refresh flows, and keeping up with provider API changes.

---

## Summary Comparison

| Feature | Option 1: External Provider | Option 2: Manual OAuth |
| :--- | :--- | :--- |
| **Effort** | Low (Hours) | High (Days/Weeks) |
| **Security** | Handled by experts (Safe) | You must implement and audit it |
| **Code Complexity** | minimal frontend, small backend shift | New routes, state management, encryption |
| **Cost** | Free for small usage, paid for scale | Free (just your developer time) |
| **Recommendation** | **Best for prototypes and startups.** | **Best for enterprise control/sovereignty.** |
