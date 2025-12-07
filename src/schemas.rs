#[allow(unused)]
#[allow(clippy::all)]
#[allow(mismatched_lifetime_syntaxes)]
pub mod common_generated;

// pub mod common_generated;
pub use self::common_generated::*;
pub mod v1 {

    // mod common_generated;
    // pub use self::common_generated::*;
    #[allow(unused)]
    #[allow(clippy::all)]
    #[allow(mismatched_lifetime_syntaxes)]
    mod requests_generated;
    pub use self::requests_generated::v_1::*;

    #[allow(unused)]
    #[allow(clippy::all)]
    #[allow(mismatched_lifetime_syntaxes)]
    mod responses_generated;
    pub use self::responses_generated::v_1::*;
} // patchify
