//! # General settings
//!

// Development status
pub static DEBUG: bool = true;
// Project
pub static PROJECT_NAME: &str = "Project Name";
// Http Server
pub static LOCAL_DOMAIN: &str = "127.0.0.1";
pub static PORT: u16 = 8080;
// Site
pub static SITE_DOMAIN: &str = "www.site-name.net";
// Max size of loaded content for Form and Json - 2.016 mb
pub static MAX_UPLOAD_SIZE: usize = (2.016 * 1024.0 * 1024.0) as usize;
// Security
// https://passwordsgenerator.net/
// Hint: Minimum 64 characters.
pub static SECRET_KEY: &str = "Nd2%c7WZk!W9Qqu4AAkE_zB_6+NpYv#L2jZjHCezFP#v8As5WvUpxnEY?K-H%+J$";
// Session
pub static SESSION_KEY: &[u8] = SECRET_KEY.as_bytes();
// Files
pub static MEDIA_URL: &str = "/media/";
pub static MEDIA_ROOT: &str = "./media/";
pub static STATIC_URL: &str = "/static/";
pub static STATIC_ROOT: &str = "./static/";
pub static TEMPLATES: &str = "./templates/";
