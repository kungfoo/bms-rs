use bms_sm::StringData;

fn main() {
    let strings = StringData::read().unwrap();

    dbg!(strings);
}
