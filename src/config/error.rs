pub mod UI { /// User-facing errors
  pub const ERRINTERNAL: &'static str = "An internal error occurred, please try again later.";
  pub const ERRVALIDFAIL: &'static str = "Validation failed: {}";
  pub const ERRPERMDENIED: &'static str = "Permission denied.";
  pub const ERRAUTHFAIL: &'static str = "Authorization failed.";
  pub const ERRTIMEOUT: &'static str = "Application timedout.";
}

pub mod SRV { /// Dev-facing errors
  pub const ERRUNKNOWN: &'static str = "Unknown Error: {}";
  pub const ERRTIMEOUT: &'static str = "Timedout Error: {}";
}