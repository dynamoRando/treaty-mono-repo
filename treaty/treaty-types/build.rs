use regex::Regex;
use std::{
    fs::{self, OpenOptions},
    io::Write,
};

fn main() {
    // this is insane, i know.
    // i need a "clean" mod of pure Rust structs than can be seralized to JSON
    // but are generated from the protobuf file, which is the source of truth for the
    // schema.

    // i need this because i need a lib that is web assembly compat.
    // prost is not web assembly (wasm) compatible.

    // after trying to use various Rust protobuf crates to try and generate
    // protobuf to Rust that could be referenced by a wasm create, i settled on this

    // other crates failed to generate consistent Rust structs (failing to account for optional types)
    // or couldn't be seralized fully to json without a ton of effort

    // this build script just copies the main generated proto implementation
    // and tries to tear out all references to prost or anything else not needed

    let remote_path = "../src/treaty_proto.rs";
    let local_path = "./src/types/treaty_proto.rs";
    fs::copy(remote_path, local_path).unwrap();
    let contents = fs::read_to_string(local_path).unwrap();

    let contents = contents.replace(", ::prost::Message", "");
    let contents = contents.replace("::prost::alloc::string::", "");
    let contents = contents.replace(r#"::prost::alloc::vec::"#, "");

    let re = Regex::new(r#"#\[prost\(.*\)\]"#).unwrap();
    let contents = re.replace_all(&contents, "");

    let end_index = contents
        .find("/// Generated client implementations.")
        .unwrap();
    let mut contents = contents.to_string();
    contents.replace_range(end_index..contents.len(), "");

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(local_path)
        .unwrap();
    file.write(contents.as_bytes()).unwrap();
}
