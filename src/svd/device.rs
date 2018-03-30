
use std::collections::HashMap;
use xmltree::Element;
use ElementExt;

use types::{Parse, Encode, new_element};
use parse;

use ::error::SVDError;
use ::svd::cpu::Cpu;
use ::svd::peripheral::Peripheral;
use ::svd::defaults::Defaults;

#[derive(Clone, Debug)]
pub struct Device {
    pub name: String,
    schema_version: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub address_unit_bits: Option<u32>,
    pub width: Option<u32>,
    pub cpu: Option<Cpu>,
    pub peripherals: Vec<Peripheral>,
    pub defaults: Defaults,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl Parse for Device {
    type Object = Device;
    type Error = SVDError;

    /// Parses a SVD file
    fn parse(tree: &Element) -> Result<Device, SVDError> {
        Ok(Device {
            name: tree.get_child_text("name")?,
            schema_version: tree.attributes.get("schemaVersion").unwrap().clone(),
            cpu: parse::optional("cpu", tree, Cpu::parse)?,
            version: parse::optional("version", tree, parse::get_text)?,
            description: parse::optional("description", tree, parse::get_text)?,
            address_unit_bits: parse::optional("addressUnitBits", tree, parse::u32)?,
            width: None,
            peripherals: {
                let ps: Result<Vec<_>, _> = parse::get_child_elem("peripherals", tree)?
                    .children
                    .iter()
                    .map(Peripheral::parse)
                    .collect();
                ps?
            },
            defaults: Defaults::parse(tree)?,
            _extensible: (),
        })
    }
}

impl Encode for Device {
    type Error = SVDError;

      fn encode(&self) -> Result<Element, SVDError> {
        let mut elem = Element {
            name: String::from("device"),
            attributes: HashMap::new(),
            children: vec![
                new_element("name", Some(self.name.clone())),
            ],
            text: None,
        };

        elem.attributes.insert(
            String::from("xmlns:xs"),
            String::from("http://www.w3.org/2001/XMLSchema-instance"),
        );
        elem.attributes.insert(
            String::from("schemaVersion"),
            format!("{}", self.schema_version),
        );
        elem.attributes.insert(
            String::from("xs:noNamespaceSchemaLocation"),
            format!("CMSIS-SVD_Schema_{}.xsd", self.schema_version),
        );
        
        match self.version {
            Some(ref v) => elem.children.push(new_element("version", Some(v.clone()))),
            None => (),
        }

        match self.description {
            Some(ref v) => elem.children.push(new_element("description", Some(v.clone()))),
            None => (),
        }

         match self.description {
            Some(ref v) => elem.children.push(new_element("addressUnitBits", Some(format!("{}", v)))),
            None => (),
        }

         match self.width {
            Some(ref v) => elem.children.push(new_element("width", Some(format!("{}", v)))),
            None => (),
        }

        match self.cpu {
            Some(ref v) => {
                elem.children.push(v.encode()?);
            }
            None => (),
        }

        let peripherals: Result<Vec<_>, _> = self.peripherals.iter().map(Peripheral::encode).collect();
        elem.children.push(Element {
            name: String::from("peripherals"),
            attributes: HashMap::new(),
            children: peripherals?,
            text: None,
        });

        Ok(elem)
    }
}
