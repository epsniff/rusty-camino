#[cfg(test)]
#[macro_export]
macro_rules! setup_test_env {
    () => {
        crate::setup_test_environment();
    };
}

#[macro_export]
macro_rules! resp_success {
    ($data:expr) => {
        crate::ResultExt::wrap(routerify_json_response::json_success_resp(&$data));
    };
    ($code:expr, $data:expr) => {
        crate::ResultExt::wrap(routerify_json_response::json_success_resp_with_code($code, &$data));
    };
}

#[macro_export]
macro_rules! resp_failed {
    ($code:expr) => {
        crate::ResultExt::wrap(routerify_json_response::json_failed_resp($code));
    };
    ($code:expr, $($arg:tt)*) => {
        crate::ResultExt::wrap(routerify_json_response::json_failed_resp_with_message($code, format!($($arg)*)));
    };
}

#[macro_export]
macro_rules! resp_200 {
    ($data:expr) => {
        resp_success!($data);
    };
}

#[macro_export]
macro_rules! resp_201 {
    ($data:expr) => {
        resp_success!(hyper::StatusCode::CREATED, $data);
    };
}

#[macro_export]
macro_rules! resp_404 {
    () => {
        resp_failed!(hyper::StatusCode::NOT_FOUND);
    };
    ($($arg:tt)*) => {
        resp_failed!(hyper::StatusCode::NOT_FOUND, $($arg)*);
    };
}

#[macro_export]
macro_rules! resp_400 {
    () => {
        resp_failed!(hyper::StatusCode::BAD_REQUEST);
    };
    ($($arg:tt)*) => {
        resp_failed!(hyper::StatusCode::BAD_REQUEST, $($arg)*);
    };
}

#[macro_export]
macro_rules! resp_500 {
    () => {
        resp_failed!(hyper::StatusCode::INTERNAL_SERVER_ERROR);
    };
    ($($arg:tt)*) => {
        resp_failed!(hyper::StatusCode::INTERNAL_SERVER_ERROR, $($arg)*);
    };
}
