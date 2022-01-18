use std::collections::HashMap;

pub fn search() -> Result<(), Box<dyn std::error::Error>> {
    //async {
    let resp =
        reqwest::blocking::get("https://httpbin.org/ip")?.json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
    //}
}

pub fn create_page() -> std::io::Result<()> {
    Ok(())
}

pub fn update_page() -> std::io::Result<()> {
    Ok(())
}
