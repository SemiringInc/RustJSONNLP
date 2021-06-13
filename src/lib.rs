#![crate_name = "jsonnlp"]

//! This is an implementation of [JSON-NLP](https://github.com/SemiringInc/JSON-NLP) in Rust.
//! [JSON-NLP](https://github.com/SemiringInc/JSON-NLP) provides the data structures for
//! detailed Natural Language Processing (NLP) annotations of speech and text.
//!
//! (C) 2021 by [Semiring Inc.](https://semiring.com/),
//!     [Damir Cavar](http://damir.cavar.me/) <damir@semiring.com>
//!
//! Version 0.0.5
//!
//! See for more details:
//!
//! - [GitHub repo](https://github.com/SemiringInc/RustJSONNLP)
//!

// use serde_derive;

use serde;
use serde_json;
use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// contains the metadata for the [JSON-NLP](https://github.com/SemiringInc/JSON-NLP) and individual documents.
/// The metadata is using Dublin Core (DC) terms.
#[derive(Serialize, Deserialize)]
pub struct Meta {
	#[serde(default,
		rename = "DC.conformsTo",
		skip_serializing_if = "String::is_empty")]
	conforms_to: String, // String,
	#[serde(default,
		rename = "DC.author",
		skip_serializing_if = "String::is_empty")]
	author: String,
	#[serde(default,
		skip_serializing_if = "String::is_empty",
		rename = "DC.created")]
	created: String,
	#[serde(default,
		rename = "DC.date",
		skip_serializing_if = "String::is_empty")]
	date: String,
	#[serde(default,
		rename = "DC.source",
		skip_serializing_if = "String::is_empty")]
	source: String,
	#[serde(default,
		rename = "DC.language",
		skip_serializing_if = "String::is_empty")]
	language: String,
	#[serde(default,
		rename = "DC.creator",
		skip_serializing_if = "String::is_empty")]
	creator: String,
	#[serde(default,
		rename = "DC.publisher",
		skip_serializing_if = "String::is_empty")]
	publisher: String,
	#[serde(default,
		rename = "DC.title",
		skip_serializing_if = "String::is_empty")]
	title: String,
	#[serde(default,
		rename = "DC.description",
		skip_serializing_if = "String::is_empty")]
	description: String,
	#[serde(default,
		rename = "DC.identifier",
		skip_serializing_if = "String::is_empty")]
	identifier: String,
}

///  contains different morpho-syntactic, semantic, or orthographic token features.
#[derive(Serialize, Deserialize)]
pub struct TokenFeatures {
	#[serde(default)]
	overt: bool,
	#[serde(default)]
	stop: bool,
	#[serde(default)]
	alpha: bool,
	#[serde(default)]
	number: u8,
	#[serde(default,
		skip_serializing_if = "String::is_empty")]
	gender: String,
	#[serde(default)]
	person: u8,
	#[serde(default,
		skip_serializing_if = "String::is_empty")]
	tense: String,
	#[serde(default)]
	perfect: bool,
	#[serde(default)]
	continuous: bool,
	#[serde(default)]
	progressive: bool,
	#[serde(default,
		skip_serializing_if = "String::is_empty")]
	case: String,
	#[serde(default)]
	human: bool,
	#[serde(default)]
	animate: bool,
	#[serde(default)]
	negated: bool,
	#[serde(default)]
	countable: bool,
	#[serde(default)]
	factive: bool,
	#[serde(default)]
	counterfactive: bool,
	#[serde(default)]
	irregular: bool,
	#[serde(default,
		rename = "phrasalVerb")]
	phrasalverb: bool,
	#[serde(default,
		skip_serializing_if = "String::is_empty")]
	mood: String,
	#[serde(default)]
	foreign: bool,
	#[serde(default,
		rename = "spaceAfter")]
	spaceafter: bool,
}

/// contains the token information.
#[derive(Serialize, Deserialize)]
pub struct Token {
	id: u64,
	sentence_id: u64,
	text: String,
	lemma: String,
	#[serde(default,
		skip_serializing_if = "String::is_empty")]
	xpos: String,
	#[serde(default)]
	xpos_prob: f64,
	#[serde(default,
		skip_serializing_if = "String::is_empty")]
	upos: String,
	#[serde(default)]
	upos_prob: f64,
	#[serde(default,
		skip_serializing_if = "String::is_empty")]
	entity_iob: String,
	#[serde(default,
		rename = "characterOffsetBegin")]
	char_offset_begin: u64,
	#[serde(default,
		rename = "characterOffsetEnd")]
	char_offset_end: u64,
	#[serde(default,
		skip_serializing_if = "String::is_empty",
		rename = "propID")]
	prop_id: String,
	#[serde(rename = "propIDProbability",
		default)]
	prop_id_prob: f64,
	#[serde(rename = "frameID",
		default)]
	frame_id: u64,
	#[serde(rename = "frameIDProb",
		default)]
	frame_id_prob: f64,
	#[serde(rename = "wordNetID",
		default)]
	wordnet_id: u64,
	#[serde(rename = "wordNetIDProb",
		default)]
	wordnet_id_prob: f64,
	#[serde(rename = "verbNetID",
		default)]
	verbnet_id: u64,
	#[serde(rename = "verbNetIDProb",
		default)]
	verbnet_id_prob: f64,
	#[serde(skip_serializing_if = "String::is_empty",
		default)]
	lang: String,
	// #[serde(default)]
	features: TokenFeatures,
	#[serde(skip_serializing_if = "String::is_empty",
		default)]
	shape: String,
	#[serde(skip_serializing_if = "String::is_empty",
		default)]
	entity: String,
}

/// contains sentence information.
#[derive(Serialize, Deserialize)]
pub struct Sentence {
	id: u64,
	#[serde(rename = "tokenFrom",
		default)]
	token_from: u64,
	#[serde(rename = "tokenTo",
		default)]
	token_to: u64,
	#[serde(default)]
	tokens: Vec<u64>,
	#[serde(default)]
	clauses: Vec<u64>,
	#[serde(rename = "type",
		default,
		skip_serializing_if = "String::is_empty")]
	stype: String,
	#[serde(skip_serializing_if = "String::is_empty",
		default)]
	sentiment: String,
	#[serde(rename = "sentimentProb",
		default)]
	sentiment_prob: f64,
}

/// contains clause information, assuming that sentences contain one or more clauses.
#[derive(Serialize, Deserialize)]
pub struct Clause {
	id: u64,
	#[serde(rename = "sentenceId",
		default)]
	sentence_id: u64,
	#[serde(rename = "tokenFrom",
		default)]
	token_from: u64,
	#[serde(rename = "tokenTo",
		default)]
	token_to: u64,
	#[serde(default)]
	tokens: Vec<u64>,
	#[serde(default)]
	main: bool,
	#[serde(default)]
	gov: u64,
	#[serde(default)]
	head: u64,
	#[serde(default)]
	neg: bool,
	#[serde(skip_serializing_if = "String::is_empty",
		default)]
	tense: String,
	#[serde(skip_serializing_if = "String::is_empty",
		default)]
	mood: String,
	#[serde(default)]
	perfect: bool,
	#[serde(default)]
	continuous: bool,
	#[serde(skip_serializing_if = "String::is_empty",
		default)]
	aspect: String,
	#[serde(skip_serializing_if = "String::is_empty",
		default)]
	voice: String,
	#[serde(skip_serializing_if = "String::is_empty",
		default)]
	sentiment: String,
	#[serde(rename = "sentimentProb",
		default)]
	sentiment_prob: f64,
}

/// contains dependency information as part of dependency trees.
/// A dependency is a tuple that contains a governor token ID, a dependent token ID, and a dependency label.
/// In addition, each dependency can provide probability information about the confidence or another likelihood property.
#[derive(Serialize, Deserialize)]
pub struct Dependency {
	lab: String,
	gov: u64,
	dep: u64,
	#[serde(default)]
	prob: f64,
}

/// This struct contains information about a dependency tree.
/// A dependency tree is a set of dependency triples.
/// In addition a tree provides the possibility to encode a probability score for the dependency tree.
#[derive(Serialize, Deserialize)]
pub struct DependencyTree {
	#[serde(rename = "sentenceId",
		default)]
	sentence_id: u64,
	#[serde(skip_serializing_if = "String::is_empty",
		default)]
	style: String,
	#[serde(default)]
	dependencies: Vec<Dependency>,
	#[serde(default)]
	prob: f64,
}

/// This struct contains information about a representative phrase or token for coreference.
#[derive(Serialize, Deserialize)]
pub struct CoreferenceRepresentantive {
	tokens: Vec<u64>,
	head: u64,
}

/// This struct contains information about a referent or anaphoric expression that refers to some referent.
#[derive(Serialize, Deserialize)]
pub struct CoreferenceReferents {
	tokens: Vec<u64>,
	head: u64,
	#[serde(default)]
	prob: f64,
}

/// This struct contains information about a coreference relation between one referent and a list of refering expressions.
#[derive(Serialize, Deserialize)]
pub struct Coreference {
	id: u64,
	representative: CoreferenceRepresentantive,
	referents: Vec<CoreferenceReferents>,
}

/// This struct contains information about scope relations between tokens or phrases in a sentence.
#[derive(Serialize, Deserialize)]
pub struct Scope {
	id: u64,
	gov: Vec<u64>,
	dep: Vec<u64>,
	terminals: Vec<u64>,
}

/// This struct contains information about the constituent parse tree for a sentence.
#[derive(Serialize, Deserialize)]
pub struct ConstituentParse {
	#[serde(rename = "sentenceId")]
	sentence_id: u64,
	#[serde(rename = "type",
		default,
		skip_serializing_if = "String::is_empty")]
	ctype: String,
	#[serde(rename = "labeledBracketing",
		default,
		skip_serializing_if = "String::is_empty")]
	labeled_bracketing: String,
	#[serde(default)]
	prob: f64,
	#[serde(default)]
	scopes: Vec<Scope>,
}

/// This struct provides information about expressions or chunks in the text.
#[derive(Serialize, Deserialize)]
pub struct Expression {
	id: u64,
	#[serde(rename = "type",
		default,
		skip_serializing_if = "String::is_empty")]
	etype: String,
	#[serde(default)]
	head: u64,
	#[serde(skip_serializing_if = "String::is_empty",
		default)]
	dependency: String,
	#[serde(rename = "tokenFrom",
		default)]
	token_from: u64,
	#[serde(rename = "tokenTo",
		default)]
	token_to: u64,
	#[serde(default)]
	tokens: Vec<u64>,
	#[serde(default)]
	prob: f64,
}

/// This struct contains information about paragraph properties in the text.
#[derive(Serialize, Deserialize)]
pub struct Paragraph {
	id: u64,
	#[serde(rename = "tokenFrom",
		default)]
	token_from: u64,
	#[serde(rename = "tokenTo",
		default)]
	token_to: u64,
	#[serde(default)]
	tokens: Vec<u64>,
	#[serde(default)]
	sentences: Vec<u64>,
}

/// This struct encodes generic attribute value tuples for Attribute Value Matrix (AVM) based encoding of properties.
#[derive(Serialize, Deserialize)]
pub struct Attribute {
	lab: String,
	val: String,
}

/// This struct encodes entity properties.
#[derive(Serialize, Deserialize)]
pub struct Entity {
	id: u64,
	#[serde(skip_serializing_if = "String::is_empty",
		default)]
	label: String,
	#[serde(rename = "type",
		default,
		skip_serializing_if = "String::is_empty")]
	etype: String,
	#[serde(skip_serializing_if = "String::is_empty",
		default)]
	url: String,
	#[serde(default)]
	head: u64,
	#[serde(rename = "tokenFrom",
		default)]
	token_from: u64,
	#[serde(rename = "tokenTo",
		default)]
	token_to: u64,
	#[serde(default)]
	tokens: Vec<u64>,
	#[serde(rename = "tripleID",
		default)]
	triple_id: u64,
	#[serde(skip_serializing_if = "String::is_empty",
		default)]
	sentiment: String,
	#[serde(rename = "sentimentProb",
		default)]
	sentiment_prob: f64,
	#[serde(default)]
	count: u64,
	#[serde(default)]
	attributes: Vec<Attribute>,
}

/// This struct encodes relations and properties in a graph for entity, cocept, or knowledge graphs.
#[derive(Serialize, Deserialize)]
pub struct Relation {
	id: u64,
	#[serde(skip_serializing_if = "String::is_empty",
		default)]
	label: String,
	#[serde(rename = "type",
		default,
		skip_serializing_if = "String::is_empty")]
	rtype: String,
	#[serde(skip_serializing_if = "String::is_empty",
		default)]
	url: String,
	#[serde(default)]
	head: u64,
	#[serde(rename = "tokenFrom",
		default)]
	token_from: u64,
	#[serde(rename = "tokenTo",
		default)]
	token_to: u64,
	#[serde(default)]
	tokens: Vec<u64>,
	#[serde(skip_serializing_if = "String::is_empty",
		default)]
	sentiment: String,
	#[serde(rename = "sentimentProb",
		default)]
	sentiment_prob: f64,
	#[serde(default)]
	count: u64,
	#[serde(default)]
	attributes: Vec<Attribute>,
}

/// This struct encodes triples for RDF, JSON-LD, or general Knowledge Graph encoding.
#[derive(Serialize, Deserialize)]
pub struct Triple {
	id: u64,
	#[serde(rename = "fromEntity",
		default)]
	from_entity: u64,
	#[serde(rename = "toEntity",
		default)]
	to_entity: u64,
	#[serde(default)]
	rel: u64,
	#[serde(rename = "clauseID",
		default)]
	clause_id: Vec<u64>,
	#[serde(rename = "sentenceID",
		default)]
	sentence_id: Vec<u64>,
	#[serde(default)]
	directional: bool,
	#[serde(rename = "eventID",
		default)]
	event_id: u64,
	#[serde(rename = "tempSeq",
		default)]
	temp_seq: u64,
	#[serde(default)]
	prob: f64,
	#[serde(default)]
	syntactic: bool,
	#[serde(default)]
	implied: bool,
	#[serde(default)]
	presupposed: bool,
	#[serde(default)]
	count: u64,
}

/// This struct contains all the information for one particular document.
#[derive(Serialize, Deserialize)]
pub struct Document {
	meta: Meta,
	id: u64,
	#[serde(rename = "tokenList",
		default)]
	token_list: Vec<Token>,
	#[serde(default)]
	clauses: Vec<Clause>,
	#[serde(default)]
	sentences: Vec<Sentence>,
	#[serde(default)]
	paragraphs: Vec<Paragraph>,
	#[serde(rename = "dependencyTrees",
		default)]
	dependency_trees: Vec<DependencyTree>,
	#[serde(default)]
	coreferences: Vec<Coreference>,
	#[serde(default)]
	constituents: Vec<ConstituentParse>,
	#[serde(default)]
	expressions: Vec<Expression>,
	#[serde(default)]
	entities: Vec<Entity>,
	#[serde(default)]
	relations: Vec<Relation>,
	#[serde(default)]
	triples: Vec<Triple>,
}

/// This struct contains general elements of a [JSON-NLP](https://github.com/SemiringInc/JSON-NLP) document.
#[derive(Serialize, Deserialize)]
pub struct JSONNLP {
	meta: Meta,
	#[serde(default)]
	docs: Vec<Document>,
}

/*
fn deserialize_any<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
	T: Deserialize<'de>,
	D: Deserializer<'de>,
{
	Deserialize::deserialize(deserializer).map(Some)
}
*/

/// This function converts a string containing [JSON-NLP](https://github.com/SemiringInc/JSON-NLP), returning a JSONNLP struct.
pub fn from_string(json: &str) -> Result<JSONNLP, Box<dyn Error>> {
	let r = serde_json::from_str::<JSONNLP>(json).unwrap();
	Ok(r)
}

/// This function reads a [JSON-NLP](https://github.com/SemiringInc/JSON-NLP) document from a file and returns a JSONNLP struct.
pub fn from_file<P: AsRef<Path>>(path: P) -> Result<JSONNLP, Box<dyn Error>> {
	let file = File::open(path)?;
	let reader = BufReader::new(file);
	let u = serde_json::from_reader(reader)?;
	Ok(u)
}

/// This function returns a string representation of a JSONNLP struct/object.
pub fn get_json(j: &JSONNLP) -> Result<String, Box<dyn Error>> {
	let r = serde_json::to_string(j).unwrap();
	Ok(r)
}
