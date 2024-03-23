use server::protocol;

#[test]
fn validate_type() {

    fn validate_string(){
        let valid_string = b"+Hello\r\n";
        let result = protocol::read(&valid_string.to_vec());
        assert_eq!(result, 200);
    }

    fn validate_error(){
        let valid_error = b"-Error\r\n";
        let result = protocol::read(&valid_error.to_vec());
        assert_eq!(result, 200);
    }

    fn validate_number(){
        let valid_number = b":32\r\n";
        let result = protocol::read(&valid_number.to_vec());
        assert_eq!(result, 200);

        let invalid_number = b":abc\r\n";
        let result = protocol::read(&invalid_number.to_vec());
        assert_eq!(result, 400);
    }

    validate_string();
    validate_error();
    validate_number();

}
