//! The client data a player logs in with, as [`Player::client_data`](crate::player::Player::client_data) and the login event give it.

use serde::{Deserialize, Serialize};

/**
The payload of the client data JWT a client sends at login: skin, device, input and locale details.

It is JSON with PascalCase keys; keys that are missing take their default value. Image, resource patch, geometry and animation fields hold base64 text, as the client sends them.
*/
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct ClientData {
    /// Animated skin textures.
    pub animated_image_data: Vec<AnimationFrame>,
    /// Arm model of the skin, `"slim"` or `"wide"`.
    pub arm_size: String,
    /// Cape image as base64 RGBA pixels, `cape_image_width` by `cape_image_height`.
    pub cape_data: String,
    /// Cape identifier.
    pub cape_id: String,
    /// Cape image height in pixels.
    pub cape_image_height: i32,
    /// Cape image width in pixels.
    pub cape_image_width: i32,
    /// Whether a persona cape is worn on a classic skin.
    pub cape_on_classic_skin: bool,
    /// Random id the client generates for itself.
    pub client_random_id: i64,
    /// Whether the client can generate chunks itself.
    pub compatible_with_client_side_chunk_gen: bool,
    /// Numeric input mode the client is currently using.
    pub current_input_mode: i32,
    /// Numeric default input mode of the device.
    pub default_input_mode: i32,
    /// Device identifier as sent by the client.
    pub device_id: String,
    /// Device model name.
    pub device_model: String,
    /// Numeric device platform (JSON key `DeviceOS`).
    #[serde(rename = "DeviceOS")]
    pub device_os: i32,
    /// Whether the client filters profanity.
    pub filter_profanity: bool,
    /// Game version string, such as `1.21.60`.
    pub game_version: String,
    /// Numeric graphics mode setting.
    pub graphics_mode: i32,
    /// GUI scale setting.
    pub gui_scale: i32,
    /// Whether the client supports Editor mode.
    pub client_is_editor_capable: bool,
    /// Locale code, such as `en_US`.
    pub language_code: String,
    /// Maximum view distance the client supports, in chunks.
    pub max_view_distance: i32,
    /// Numeric memory tier of the device.
    pub memory_tier: i32,
    /// Whether the skin overrides the player's appearance.
    pub override_skin: bool,
    /// Identifier of the party the player is in, if any.
    pub party_id: String,
    /// Whether the player leads their party.
    pub is_party_leader: bool,
    /// Persona pieces making up a persona skin.
    pub persona_pieces: Vec<PersonaPiece>,
    /// Whether the skin is a persona skin.
    pub persona_skin: bool,
    /// Tint colours of the persona pieces.
    pub piece_tint_colors: Vec<TintColor>,
    /// Platform offline id.
    pub platform_offline_id: String,
    /// Platform online id.
    pub platform_online_id: String,
    /// PlayFab id of the player (JSON key `PlayFabId`).
    #[serde(rename = "PlayFabId")]
    pub play_fab_id: String,
    /// Numeric platform type.
    pub platform_type: i32,
    /// Platform user id.
    pub platform_user_id: String,
    /// Whether the skin comes from a paid pack.
    pub premium_skin: bool,
    /// Profile hash sent by the client.
    pub profile_hash: String,
    /// Self-signed UUID of the player.
    pub self_signed_id: String,
    /// Server address the client connected to.
    pub server_address: String,
    /// Skin animation JSON, base64-encoded.
    pub skin_animation_data: String,
    /// Skin colour as a `#`-prefixed hex string.
    pub skin_color: String,
    /// Skin image as base64 RGBA pixels, `skin_image_width` by `skin_image_height`.
    pub skin_data: String,
    /// Skin geometry JSON, base64-encoded.
    pub skin_geometry_data: String,
    /// Minimum engine version of the geometry, plain or base64-encoded.
    pub skin_geometry_data_engine_version: String,
    /// Skin identifier.
    pub skin_id: String,
    /// Skin image height in pixels.
    pub skin_image_height: i32,
    /// Skin image width in pixels.
    pub skin_image_width: i32,
    /// Skin resource patch JSON, which names the geometry to use, base64-encoded.
    pub skin_resource_patch: String,
    /// Display name of the player on their platform.
    pub third_party_name: String,
    /// Whether the skin is marked as trusted.
    pub trusted_skin: bool,
    /// Numeric UI profile (JSON key `UIProfile`).
    #[serde(rename = "UIProfile")]
    pub ui_profile: i32,
    /// Numeric Editor connection intent.
    pub client_editor_connection_intent: i32,
}

/// One animated skin texture: an image strip with its frames stacked vertically.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct AnimationFrame {
    /// Strip height in pixels.
    pub image_height: i32,
    /// Strip width in pixels.
    pub image_width: i32,
    /// Number of frames in the strip.
    pub frames: f32,
    /// Numeric animation type (JSON key `Type`).
    #[serde(rename = "Type")]
    pub kind: i32,
    /// Strip image as base64 RGBA pixels.
    pub image: String,
    /// Numeric animation expression.
    pub animation_expression: i32,
}

/// One piece of a persona skin.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct PersonaPiece {
    /// Piece identifier.
    pub piece_id: String,
    /// Piece type name, such as `persona_hair`.
    pub piece_type: String,
    /// UUID of the pack the piece comes from, as text.
    pub pack_id: String,
    /// Whether the piece is a default piece.
    pub is_default: bool,
    /// Marketplace product id of the piece; empty for free pieces.
    pub product_id: String,
}

/// Tint colours for one persona piece type.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct TintColor {
    /// Piece type name the colours apply to.
    pub piece_type: String,
    /// Colours as `#`-prefixed hex strings.
    pub colors: Vec<String>,
}

impl ClientData {
    /// Parses client data JSON, returning the default value when it does not parse.
    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(json).unwrap_or_default()
    }
}
