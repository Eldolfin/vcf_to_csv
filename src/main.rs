use ical::{parser::vcard::component::VcardContact, VcardParser};
use std::{env::args, error::Error, fs::File, io::Read};

const OUTPUT_FILENAME: &str = "output.csv";

fn main() -> Result<(), Box<dyn Error>> {
    // prends le deuxieme argument en tant que nom de fichier
    let filename = args().nth(1).expect("Usage: vcf_to_csv FILENAME");

    println!("Converting {filename} 🤓");

    let mut file = File::open(filename).expect("This file doesn't exist 😢");

    // Le fichier n'est peut etre pas en utf-8 donc il faut le convertir
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let contents = String::from_utf8_lossy(&buf);

    // Parse vcards
    let vcards = VcardParser::new(contents.as_bytes());

    let mut csv = csv::Writer::from_path(OUTPUT_FILENAME)?;

    csv.write_record(&["email", "name", "attributes"])?;

    for vc in vcards {
        let mut vc = vc
            .map_err(|err| eprintln!("error while parsing file: {err}"))
            .unwrap();

        let email = get_property_and_remove(&mut vc, "EMAIL");
        let name = get_property_and_remove(&mut vc, "FN");

        let other_properties = vc
            .properties
            .iter()
            .map(|p| {
                // maps a property to json format
                Some(format!(
                    "\"{name}\":\"{value}\"",
                    name = p.name,
                    value = p.value.clone()?
                ))
            })
            // to remove nones (where there is no value)
            .flatten()
            .collect::<Vec<_>>()
            .join(", ");

        // adds {} around json
        let other_properties = format!("{{{other_properties}}}");

        let record = [email, name, other_properties];
        csv.write_record(&record)?;
    }

    Ok(())
}

fn get_property_and_remove(vc: &mut VcardContact, property: &str) -> String {
    if let Some(index) = vc.properties.iter().position(|x| x.name == property) {
        if let Some(text) = vc.properties.remove(index).value {
            text
        } else {
            panic!("Could get the {property} data for {vc:?}");
        }
    } else {
        panic!("Could not find a `{property}` property for {vc:?}");
    }
}
