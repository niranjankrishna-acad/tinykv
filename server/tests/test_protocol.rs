use server::protocol;

#[test]
fn validate_type() {

    fn validate_string(){
        let valid_data = b"+Hello\r\n";
        let result = protocol::read(&valid_data.to_vec());
        assert_eq!(result, 200);
    }    

    validate_string();

}
