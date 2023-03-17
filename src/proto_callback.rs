use std::sync::Arc;
use protobuf::{Message, CodedOutputStream};
use crate::fazi::Fazi;

use crate::mutate::Mutable;

use rand::Rng;

pub(crate) fn mutate_protobuf_message<R: Rng>(data: &[u8], fazi: &mut Fazi<R>) -> Vec<u8> {
    let mut message = if data.is_empty() {
        crate::e2e::Message::parse_from_bytes(data).expect("failed to parse input message")
    } else {
        Default::default()
    };

    message.mutate(fazi);

    let mut output = Vec::new();
    let mut output_stream = CodedOutputStream::vec(&mut output);

    message.write_to(&mut output_stream).expect("failed to write to output stream");
    drop(output_stream);

    output
}
