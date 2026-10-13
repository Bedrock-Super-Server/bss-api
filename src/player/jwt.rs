/*!
The tokens a Bedrock client sends when it logs in, as a [`LoginEvent`](crate::event::LoginEvent) carries them.

The server has already checked their signatures before the event fires. Every field is read leniently: a missing or malformed claim reads as an empty string or `0`, and claims this module does not name are kept in `extra`.
*/

use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};

/// Everything the client sent at login.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct LoginJwt {
    /// Protocol version the client speaks.
    pub protocol: u32,
    /// Protocol version the server talks to it.
    pub negotiated_protocol: u32,
    /// How the client authenticated.
    pub auth_info: AuthInfo,
    /// The identity token.
    pub token: Jwt,
    /// The token that carries the device and skin data.
    pub client_data_token: ClientDataJwt,
}

impl LoginJwt {
    /// Reads the JSON the server forwards to plugins; malformed JSON gives the default value.
    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(json).unwrap_or_default()
    }
}

/// How the client authenticated.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct AuthInfo {
    /// Authentication type the client declared.
    #[serde(rename = "AuthenticationType", deserialize_with = "lenient_i64")]
    pub authentication_type: i64,
    /// Authentication token, as the client sent it.
    #[serde(rename = "Token", deserialize_with = "lenient_string")]
    pub token: String,
    /// Other fields.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A decoded JSON Web Token.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Jwt {
    /// The token as received.
    pub raw: String,
    /// Its header.
    pub header: JwtHeader,
    /// Its claims.
    pub claims: JwtClaims,
    /// Its signature, base64url encoded.
    pub signature: String,
}

/// The client data token, whose claims describe the device and the skin.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct ClientDataJwt {
    /// The token as received.
    pub raw: String,
    /// Its header.
    pub header: JwtHeader,
    /// Its claims, as JSON.
    pub claims: Map<String, Value>,
    /// Its signature, base64url encoded.
    pub signature: String,
}

/// The header of a [`Jwt`].
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct JwtHeader {
    /// Signature algorithm, such as `ES384`.
    #[serde(deserialize_with = "lenient_string")]
    pub alg: String,
    /// Token type.
    #[serde(deserialize_with = "lenient_string")]
    pub typ: String,
    /// Id of the signing key.
    #[serde(deserialize_with = "lenient_string")]
    pub kid: String,
    /// Public key that signed the token, base64 encoded.
    #[serde(deserialize_with = "lenient_string")]
    pub x5u: String,
    /// Other fields.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/**
The claims of the identity token.

Fields keep the short names of the token; those this crate cannot give a meaning to are documented as the claim they hold.
*/
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct JwtClaims {
    /// Issuer of the token.
    #[serde(deserialize_with = "lenient_string")]
    pub iss: String,
    /// Audience of the token.
    #[serde(deserialize_with = "lenient_string")]
    pub aud: String,
    /// Subject of the token.
    #[serde(deserialize_with = "lenient_string")]
    pub sub: String,
    /// Expiry time, in seconds since the Unix epoch.
    #[serde(deserialize_with = "lenient_i64")]
    pub exp: i64,
    /// Time before which the token is not valid, in seconds since the Unix epoch.
    #[serde(deserialize_with = "lenient_i64")]
    pub nbf: i64,
    /// Time the token was issued, in seconds since the Unix epoch.
    #[serde(deserialize_with = "lenient_i64")]
    pub iat: i64,
    /// Public key of the client, base64 encoded.
    #[serde(deserialize_with = "lenient_string")]
    pub cpk: String,
    /// Xbox user id of the player.
    #[serde(deserialize_with = "lenient_string")]
    pub xid: String,
    /// Gamertag of the player.
    #[serde(deserialize_with = "lenient_string")]
    pub xname: String,
    /// UUID of the player.
    #[serde(deserialize_with = "lenient_string")]
    pub leguuid: String,
    /// The `tid` claim.
    #[serde(deserialize_with = "lenient_string")]
    pub tid: String,
    /// The `pid` claim.
    #[serde(deserialize_with = "lenient_string")]
    pub pid: String,
    /// The `pname` claim.
    #[serde(deserialize_with = "lenient_string")]
    pub pname: String,
    /// Time the account was created, in seconds since the Unix epoch.
    #[serde(deserialize_with = "lenient_i64")]
    pub pfcd: i64,
    /// The `ipt` claim.
    #[serde(deserialize_with = "lenient_string")]
    pub ipt: String,
    /// The `mid` claim.
    #[serde(deserialize_with = "lenient_string")]
    pub mid: String,
    /// The `ap` claim.
    #[serde(deserialize_with = "lenient_i64")]
    pub ap: i64,
    /// Other claims.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl JwtClaims {
    /// Returns how long the token is valid for, in seconds.
    pub fn lifetime_secs(&self) -> i64 {
        self.exp - self.iat
    }

    /// Returns the age of the account when the token was issued, in seconds, or `None` when the token does not say.
    pub fn account_age_secs(&self) -> Option<i64> {
        (self.pfcd != 0 && self.iat != 0).then(|| self.iat - self.pfcd)
    }
}

fn lenient_string<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(text) => text,
        Value::Null => String::new(),
        other => other.to_string(),
    })
}

fn lenient_i64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i64, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::Number(number) => number
            .as_i64()
            .unwrap_or(number.as_f64().unwrap_or(0.0) as i64),
        Value::String(text) => text.parse().unwrap_or(0),
        _ => 0,
    })
}
