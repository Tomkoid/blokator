pub fn check_http_error(data: &str) -> bool {
    if data.contains("<title>404 Not Found</title>") {
        return true;
    }

    false
}
