use scale_info::TypeInfo;

#[derive(TypeInfo)]
struct Me<'a> {
    me: &'a Me<'a>,
}

fn assert_type_info<T: TypeInfo>() {}

fn main() {
    assert_type_info::<Me>();
}
