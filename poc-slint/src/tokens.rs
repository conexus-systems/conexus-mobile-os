//! Design tokens loader

use crate::models::DesignTokens;
use std::sync::OnceLock;

static TOKENS: OnceLock<DesignTokens> = OnceLock::new();

pub fn load_tokens() -> &'static DesignTokens {
    TOKENS.get_or_init(|| {
        DesignTokens::load_from_file("design-tokens.json")
            .expect("Failed to load design-tokens.json")
    )
}

pub fn tokens() -> &'static DesignTokens {
    load_tokens()
}