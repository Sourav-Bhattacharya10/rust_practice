fn convert_fahrenheit_to_celsius(temp_fahrenheit: f32) -> f32 {
    (temp_fahrenheit - 32.0) * (5.0f32/9.0)
}

fn convert_celsius_to_fahrenheit(temp_celsius: f32) -> f32 {
    (temp_celsius * (9.0f32/5.0)) + 32.0
}

pub enum Temperature {
    Fahrenheit,
    Celsius
}

pub fn convert_temp(temp: f32, scale: Temperature) -> f32 {
    match scale {
        Temperature::Fahrenheit => convert_fahrenheit_to_celsius(temp),
        Temperature::Celsius => convert_celsius_to_fahrenheit(temp)
    }
}