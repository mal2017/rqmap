use rust_htslib::bam::HeaderView;
use std::collections::HashMap;
use std::str;
use std::string::String;

/// Wrapper for a HashMap for converting TIDs to chromosome names.
/// Keys are TIDs from a bam, while associated values are strings.
/// Other functions may take a reference to this map as an argument
/// and us thus intended to be created once for every bam rather
/// than reinitialized frequently.
pub struct ScaffoldDict {
    i2s: HashMap<i32, String>,
    s2i: HashMap<String, i32>
}

impl ScaffoldDict {
    // TODO handle panics
    /// Create a ScaffoldDict from a header view as returned
    /// by `bam.header()`.
    pub fn from_header_view(h: &HeaderView) -> ScaffoldDict {
        let tgts = h.target_names();
        let mut dict: HashMap<i32, String> = HashMap::with_capacity(tgts.len());
        let mut rev: HashMap<String, i32> = HashMap::with_capacity(tgts.len());
        for (i, t) in tgts.iter().map(|a| str::from_utf8(a).unwrap()).enumerate() {
            dict.insert(i as i32, t.to_owned());
            rev.insert(t.to_owned(), i as i32);
        }
        ScaffoldDict {
            i2s: dict,
            s2i: rev,
        }
    }

    /// Take a TID (from a bam record, for example) and return
    /// the human-readable representation of the host scaffold.
    pub fn id_to_str(&self, id: i32) -> Option<&str> {
        match self.i2s.get(&id) {
            Some(c) => Some(c),
            None => None,
            }
        }

    /// Take the human-readable representation of the host scaffold
    /// and return a TID (from a bam record, for example).
    pub fn str_to_id(&self, s: &str) -> Option<i32> {
        match self.s2i.get(s) {
            Some(c) => Some(c.clone()),
            None => None,
            }
        }

}
