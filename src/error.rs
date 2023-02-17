pub fn check_http_error(data: &str) -> bool {
    let mut error: bool = false;
    if data.contains("<title>404 Not Found</title>") {
        error = true;
    }
    error
}
