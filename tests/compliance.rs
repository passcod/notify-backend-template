#[macro_use]
extern crate notify_backend;
extern crate notify_backend_{{crate_name}};
extern crate tempdir;

use notify_backend_{{crate_name}}::Backend;

test_compliance!(Backend);
