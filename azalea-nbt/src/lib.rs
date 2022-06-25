mod decode;
mod encode;
mod error;
mod tag;

pub use error::Error;
pub use tag::Tag;

#[cfg(test)]
mod tests {
    use super::*;
    use azalea_buf::{McBufReadable, McBufWritable};
    use std::{collections::HashMap, io::Cursor};

    #[test]
    fn mcbuf_nbt() {
        let mut buf = Vec::new();
        let tag = Tag::Compound(HashMap::from_iter(vec![(
            "hello world".to_string(),
            Tag::Compound(HashMap::from_iter(vec![(
                "name".to_string(),
                Tag::String("Bananrama".to_string()),
            )])),
        )]));
        tag.write_into(&mut buf).unwrap();

        let mut buf = Cursor::new(buf);

        let result = Tag::read_from(&mut buf).unwrap();
        assert_eq!(
            result,
            Tag::Compound(HashMap::from_iter(vec![(
                "hello world".to_string(),
                Tag::Compound(HashMap::from_iter(vec![(
                    "name".to_string(),
                    Tag::String("Bananrama".to_string()),
                )])),
            )]))
        );
    }
}
