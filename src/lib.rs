
pub fn a_string() -> String {
	"a string".to_string()
}

#[test]
fn it_works() {
	assert_eq!("a string", &a_string());
}
