extern crate jsonpath;
extern crate rdf;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod converter;
pub mod definition;
pub mod mapper;

pub use converter::Converter;
pub use mapper::loader::Mapper;

#[cfg(test)]
mod tests {
    #[test]
    fn json_path() {
        use Converter;
        use Mapper;
        use serde_json;
        use std::fs::File;
        use std::io::Read;

        let mut video_struct = String::new();
        let mut video_file = File::open("tests/video.json").unwrap();
        let _ = video_file.read_to_string(&mut video_struct).unwrap();

        let mut ntriple_struct = String::new();
        let mut ntriple_file = File::open("tests/ntriple.txt").unwrap();
        let _ = ntriple_file.read_to_string(&mut ntriple_struct).unwrap();

        let doc = serde_json::from_str(&video_struct).unwrap();

        let mut converter = Converter::new();
        let mapper = Mapper::load("tests/mapping.json");

        mapper.process(&doc, &mut converter);

        let content = converter.to_ntriple_string();
        println!("{}", content);
        assert!(content == ntriple_struct);
        // let mut turtle_struct = String::new();
        // let mut turtle_file = File::open("tests/turtle.txt").unwrap();
        // let _ = turtle_file.read_to_string(&mut turtle_struct).unwrap();

        // let content = converter.to_turtle_string();
        // println!("{}", content);
        // assert!(false);
    }
}
