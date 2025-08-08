use beep;

fn main() -> Result<(), beep::RodioError> {
    beep::beep()?;
    Ok(())
}
