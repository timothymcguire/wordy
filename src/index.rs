use std::borrow::Borrow;
use std::io::{BufReader, Error};
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::hash::Hash;
use std::io;

#[derive(Debug, Clone)]
pub struct IndexData {
    pub sysnet_cnt: usize,
    pub p_cnt: usize,
    pub ptr_symbol: Vec<PointerSymbol>,
    pub tagsense_cnt: u32,
    pub synset_offset: Vec<u32>
}

#[derive(Debug, Clone)]
pub struct LemmaIndices {
    pub noun_index: Option<IndexData>,
    pub verb_index: Option<IndexData>,
    pub adj_index: Option<IndexData>,
    pub adv_index: Option<IndexData>
}

#[derive(Debug, Clone)]
pub enum PointerSymbol {
    Antonym,
    Hypernym,
    InstanceHypernym,
    Hyponym,
    InstanceHyponym,
    MemberHolonym,
    SubstanceHolonym,
    PartHolonym,
    MemberMeronym,
    SubstanceMeronym,
    PartMeronym,
    Attribute,
    DerivationallyRelatedForm,
    DomainOfSynset,
    MemberOfThisDomain,

    // Verbs
    Entailment,
    Cause,
    AlsoSee,
    VerbGroup,

    // Adjectives / Adverbs
    ParticipleOfVerb,
    PartainymDerived,
    SimilarTo,
}

impl PointerSymbol {
    pub fn from_str(string: &str) -> PointerSymbol {
        match string {
            "!" => PointerSymbol::Antonym,
            "@" => PointerSymbol::Hypernym,
            "@i" => PointerSymbol::InstanceHypernym,
            "~" => PointerSymbol::Hyponym,
            "~i" => PointerSymbol::InstanceHyponym,
            "#m" => PointerSymbol::MemberHolonym,
            "#s" => PointerSymbol::SubstanceHolonym,
            "#p" => PointerSymbol::PartHolonym,
            "%m" => PointerSymbol::MemberMeronym,
            "%s" => PointerSymbol::SubstanceMeronym,
            "%p" => PointerSymbol::PartMeronym,
            "=" => PointerSymbol::Attribute,
            "+" => PointerSymbol::DerivationallyRelatedForm,
            ";" => PointerSymbol::DomainOfSynset,
            "-" => PointerSymbol::MemberOfThisDomain,

            // Verb
            "*" => PointerSymbol::Entailment,
            ">" => PointerSymbol::Cause,
            "^" => PointerSymbol::AlsoSee,
            "$" => PointerSymbol::VerbGroup,

            // Adjective / Adverb
            "<" => PointerSymbol::ParticipleOfVerb,
            "\\" => PointerSymbol::PartainymDerived,
            "&" => PointerSymbol::SimilarTo,
            _ => panic!("Failure to convert {} to Pointer", string)
        }
    }
}

pub struct Index {
    noun_map: HashMap<String, IndexData>,
    verb_map: HashMap<String, IndexData>,
    adv_map: HashMap<String, IndexData>,
    adj_map: HashMap<String, IndexData>,
}

impl Index {
    pub fn new() -> Index {
        Index {
            noun_map: HashMap::new(),
            verb_map: HashMap::new(),
            adv_map: HashMap::new(),
            adj_map: HashMap::new()
        }
    }

    pub fn with_capacity
    (
        noun_capacity: usize,
        verb_capacity: usize,
        adv_capacity: usize,
        adj_capacity: usize
    ) -> Index {
        Index {
            noun_map: HashMap::with_capacity(noun_capacity),
            verb_map: HashMap::with_capacity(verb_capacity),
            adv_map: HashMap::with_capacity(adv_capacity),
            adj_map: HashMap::with_capacity(adj_capacity)
        }
    }

    pub fn parse_file(&mut self, filename: &str) -> Result<(), Error> {
        // Open and check the file
        let file = match File::open(filename) {
            Ok(file) => file,
            Err(error) => return Result::Err(error),  // Some IO error
        };

        let reader = BufReader::new(file);

        // Read file, and parse
        for line in reader.lines() {
            let line =  line.unwrap();

            // Check for "comment lines" so they are ignored
            if line.chars().nth(0).unwrap() != ' ' {
                let mut iter= line.split_whitespace();
                let lem = iter.next().expect("NO lemma");
                let pos = iter.next().expect("NO pos");

                let sysnet_cnt: usize = iter.next().expect("NO sysnet_cnt").parse().unwrap();
                let p_cnt: usize  = iter.next().expect("NO p_cnt").parse().unwrap();
                let mut ptr_symbol: Vec<PointerSymbol> = Vec::with_capacity(p_cnt);
                for _ in 0..p_cnt {
                    let p_str = iter.next().expect("EXPECTED PTR");
                    let mut pointer = PointerSymbol::from_str(p_str);
                    ptr_symbol.push(pointer);
                }

                iter.next(); // ignore sense_cnt
                let tagsense_cnt: u32 = iter.next().expect("NO tagsense_cnt").parse().unwrap();

                let mut synset_offset: Vec<u32> = Vec::with_capacity(sysnet_cnt);
                for _ in 0..sysnet_cnt {
                    let ptr: u32 = iter.next().expect("EXPECTED PTR").parse().unwrap();
                    synset_offset.push(ptr);
                }

                let lemma_data = IndexData {
                    sysnet_cnt,
                    p_cnt,
                    ptr_symbol,
                    tagsense_cnt,
                    synset_offset
                };

                // Figure out the type of lemma, and add it to the appropriate map
                match pos {
                    "n" => {
                        self.noun_map.insert(
                            lem.to_string(),
                            lemma_data
                        );
                    },
                    "v" => {
                        self.verb_map.insert(
                            lem.to_string(),
                            lemma_data
                        );
                    },
                    "a" => {
                        self.adj_map.insert(
                            lem.to_string(),
                            lemma_data
                        );
                    },
                    "r" => {
                        self.adv_map.insert(
                            lem.to_string(),
                            lemma_data
                        );
                    },
                    _ => panic!("CANNOT PARSE")
                };
            }
        }
        Ok(()) // All good
    }

    pub fn len(&self) -> usize {
        self.noun_map.len() + self.verb_map.len() + self.adj_map.len() + self.adv_map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.noun_map.is_empty() && self.verb_map.is_empty() && self.adj_map.is_empty() && self.adv_map.is_empty()
    }

    pub fn contains(&self, word: &String) -> bool {
        self.noun_map.contains_key(word)
            || self.verb_map.contains_key(word)
            || self.adj_map.contains_key(word)
            || self.adv_map.contains_key(word)
    }

    pub fn contains_str(&self, word: &str) -> bool {
        self.noun_map.contains_key(word)
            || self.verb_map.contains_key(word)
            || self.adj_map.contains_key(word)
            || self.adv_map.contains_key(word)
    }

    // does a clone of the index data, shouldn't be too inefficient because the vectors
    // will not often be more than 6 values long, and only hold integer types or enums
    pub fn get_noun_index(&self, lemma: &str) -> Option<IndexData> {
        match self.noun_map.get(lemma) {
            Option::Some(index_data) => Option::Some(index_data.clone()),
            Option::None => Option::None
        }
    }

    pub fn get_verb_index(&self, lemma: &str) -> Option<IndexData> {
        match self.verb_map.get(lemma) {
            Option::Some(index_data) => Option::Some(index_data.clone()),
            Option::None => Option::None
        }
    }

    pub fn get_adj_index(&self, lemma: &str) -> Option<IndexData> {
        match self.adj_map.get(lemma) {
            Option::Some(index_data) => Option::Some(index_data.clone()),
            Option::None => Option::None
        }
    }

    pub fn get_adv_index(&self, lemma: &str) -> Option<IndexData> {
        match self.adv_map.get(lemma) {
            Option::Some(index_data) => Option::Some(index_data.clone()),
            Option::None => Option::None
        }
    }

    pub fn get_lemma_indices(&self, lemma: &str) -> LemmaIndices {
        LemmaIndices {
            noun_index: self.get_noun_index(lemma),
            verb_index: self.get_verb_index(lemma),
            adj_index: self.get_adj_index(lemma),
            adv_index: self.get_adv_index(lemma),
        }
    }
}