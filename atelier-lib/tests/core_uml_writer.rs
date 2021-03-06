pub mod common;

#[cfg(feature = "uml")]
#[test]
fn test_uml_writer() {
    use atelier_core::io::write_model_to_string;
    use atelier_lib::format::plant_uml::PlantUmlWriter;

    let model = common::make_message_of_the_day_model();
    let mut writer = PlantUmlWriter::new(true);
    let output = write_model_to_string(&mut writer, &model);
    assert!(output.is_ok());
    println!("{}", output.unwrap())
}
