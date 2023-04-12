use ical::{parser::vcard::component::VcardContact, VcardParser};
use std::{env::args, error::Error, fs::File, io::Read};

const OUTPUT_FILENAME: &str = "output.csv";

fn main() -> Result<(), Box<dyn Error>> {
    let filename = args().nth(1).expect("Usage: vcf_to_csv FILENAME");

    println!("Converting {filename} 🤓");

    let mut file = File::open(filename).expect("This file doesn't exist 😢");

    // The file might not be in utf-8 so we convert it
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    // conversion happens here
    let contents: String = buf.iter().map(|&c| c as char).collect();

    // Parse vcards
    let vcards = VcardParser::new(contents.as_bytes());

    let mut csv = csv::Writer::from_path(OUTPUT_FILENAME)?;

    csv.write_record(["email", "name", "attributes"])?;

    for vc in vcards {
        let mut vc = vc
            .map_err(|err| eprintln!("error while parsing file: {err}"))
            .unwrap();

        let email = get_property_and_remove(&mut vc, "EMAIL");
        let name = get_property_and_remove(&mut vc, "FN");

        let other_properties = vc
            .properties
            .iter()
            .filter_map(|p| {
                // maps a property to json format
                Some(format!(
                    "\"{name}\":\"{value}\"",
                    name = p.name,
                    value = p.value.clone()?
                ))
            })
            .collect::<Vec<_>>()
            .join(", ");

        // adds {} around json
        let other_properties = format!("{{{other_properties}}}");

        csv.write_record([email, name, other_properties])?;
    }

    println!("Done converting to {OUTPUT_FILENAME} 😎");

    Ok(())
}

fn get_property_and_remove(vc: &mut VcardContact, property: &str) -> String {
    if let Some(index) = vc.properties.iter().position(|x| x.name == property) {
        let prop = vc.properties.remove(index);
        if let Some(text) = prop.value {
            text
        } else {
            panic!("Could get the {property} data for {prop} 😨");
        }
    } else {
        panic!("Could not find a `{property}` property for {vc:?} 😨");
    }
}
