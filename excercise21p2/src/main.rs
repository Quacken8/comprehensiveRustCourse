fn main() {
    println!("u");
}

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {

    // split input by slashes
    let prefix_arr: Vec<&str> = prefix.split("/").collect();
    let request_arr: Vec<&str> = request_path.split("/").collect();

    if prefix_arr.len() > request_arr.len() {
        return false;
    }

    // check slash for slash

    for i in 0..prefix_arr.len() {
        let pref_char = prefix_arr[i];
        if pref_char == "*" {
            continue;
        }
        let req_char = request_arr[i];
        if req_char != pref_char {
            return false;
        }
    }
    return true;
    
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}
