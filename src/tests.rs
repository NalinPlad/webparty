
mod test {
    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    // #[test]
    // fn hello_world() {
    //     let client = Client::tracked(rocket()).expect("valid rocket instance");
    //     let mut response = client.get(uri!(crate::hello)).dispatch();
    //     assert_eq!(response.status(), Status::Ok);
    //     assert_eq!(response.into_string().unwrap(), "Hello, world!");
    // }

    /// Test getting the default.html
    #[test]
    fn get_default() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(crate::index)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        // assert_eq!(response.into_string().unwrap(), "");
    }

    // / Test pushing an update to the page then fetching the page
    // fn push_update() {
    //     let client = Client::tracked(rocket()).expect("valid rocket instance");
    //     let response = client.post(uri!(crate::push_html), "Hello, world!").dispatch();
    //     assert_eq!(response.status(), Status::Ok);
    //     // assert_eq!(response.into_string().unwrap(), "");
    // }

    
    
}