#![crate_name = "jsonnlp"]

/// This is an example implementation of JSON-NLP tools.
/// (C) 2021 by Damir Cavar <damir@semiring.com>
/// Verion 0.0.1


//use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde_json::json;
use serde_json;
use serde;
use serde::{Serialize, Deserialize};
// use configparser::ini::Ini;
use std::env;


#[derive(Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "DC.conformsTo")]
    #[serde(skip_serializing_if = "String::is_empty")]
	conforms_to: String,
    #[serde(rename = "DC.author")]
    #[serde(skip_serializing_if = "String::is_empty")]
	author:      String,
    #[serde(rename = "DC.created")]
    #[serde(skip_serializing_if = "String::is_empty")]
	created:     String,
    #[serde(rename = "DC.date")]
    #[serde(skip_serializing_if = "String::is_empty")]
	date:        String,
    #[serde(rename = "DC.source")]
    #[serde(skip_serializing_if = "String::is_empty")]
	source:      String,
    #[serde(rename = "DC.language")]
    #[serde(skip_serializing_if = "String::is_empty")]
	language:    String,
    #[serde(rename = "DC.creator")]
    #[serde(skip_serializing_if = "String::is_empty")]
	creator:     String,
    #[serde(rename = "DC.publisher")]
    #[serde(skip_serializing_if = "String::is_empty")]
	publisher:   String,
    #[serde(rename = "DC.title")]
    #[serde(skip_serializing_if = "String::is_empty")]
	title:       String,
    #[serde(rename = "DC.description")]
    #[serde(skip_serializing_if = "String::is_empty")]
	description: String,
    #[serde(rename = "DC.identifier")]
    #[serde(skip_serializing_if = "String::is_empty")]
	identifier:  String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenFeatures {
	overt:          bool,
	stop:           bool,
	alpha:          bool,
	number:         u8,
    #[serde(skip_serializing_if = "String::is_empty")]
	gender:         String,
	person:         u8,
    #[serde(skip_serializing_if = "String::is_empty")]
	tense:          String,
	perfect:        bool,
	continuous:     bool,
	progressive:    bool,
    #[serde(skip_serializing_if = "String::is_empty")]
	case:           String,
	human:          bool,
	animate:        bool,
	negated:        bool,
	countable:      bool,
	factive:        bool,
	counterfactive: bool,
	irregular:      bool,
    #[serde(rename = "phrasalVerb")]
	phrasalverb:    bool,
    #[serde(skip_serializing_if = "String::is_empty")]
	mood:           String,
	foreign:        bool,
    #[serde(rename = "spaceAfter")]
	spaceafter:     bool,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
	id:                u64,
	sentence_id:       u64,
	text:              String,
	lemma:             String,
    #[serde(skip_serializing_if = "String::is_empty")]
	xpos:              String,
	xpos_prob:         f64,
    #[serde(skip_serializing_if = "String::is_empty")]
	upos:              String,
	upos_prob:         f64,
    #[serde(skip_serializing_if = "String::is_empty")]
	entity_iob:        String,
    #[serde(rename = "characterOffsetBegin")]
	char_offset_begin: u64,
    #[serde(rename = "characterOffsetEnd")]
	char_offset_end:   u64,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "propID")]
	prop_id:           String,
    #[serde(rename = "propIDProbability")]
	prop_id_prob:      f64,
    #[serde(rename = "frameID")]
	frame_id:          u64,
    #[serde(rename = "frameIDProb")]
	frame_id_prob:     f64,
    #[serde(rename = "wordNetID")]
	wordnet_id:        u64,
    #[serde(rename = "wordNetIDProb")]
	wordnet_id_prob:   f64,
    #[serde(rename = "verbNetID")]
	verbnet_id:        u64,
    #[serde(rename = "verbNetIDProb")]
	verbnet_id_prob:   f64,
    #[serde(skip_serializing_if = "String::is_empty")]
	lang:              String,
	features:          TokenFeatures,
    #[serde(skip_serializing_if = "String::is_empty")]
	shape:             String,
    #[serde(skip_serializing_if = "String::is_empty")]
	entity:            String,
}

#[derive(Serialize, Deserialize)]
pub struct Sentence {
	id:             u64,
    #[serde(rename = "tokenFrom")]
	token_from:     u64,
    #[serde(rename = "tokenTo")]
	token_to:       u64,
	tokens:         Vec<u64>,
	clauses:        Vec<u64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "String::is_empty")]
	stype:          String,
    #[serde(skip_serializing_if = "String::is_empty")]
	sentiment:      String,
    #[serde(rename = "sentimentProb")]
	sentiment_prob: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Clause {
	id:             u64,
    #[serde(rename = "sentenceId")]
	sentence_id:    u64,
    #[serde(rename = "tokenFrom")]
	token_from:     u64,
    #[serde(rename = "tokenTo")]
	token_to:       u64,
	tokens:         Vec<u64>,
	main:           bool,
	gov:            u64,
	head:           u64,
	neg:            bool,
    #[serde(skip_serializing_if = "String::is_empty")]
	tense:          String,
    #[serde(skip_serializing_if = "String::is_empty")]
	mood:           String,
	perfect:        bool,
	continuous:     bool,
    #[serde(skip_serializing_if = "String::is_empty")]
	aspect:         String,
    #[serde(skip_serializing_if = "String::is_empty")]
	voice:          String,
    #[serde(skip_serializing_if = "String::is_empty")]
	sentiment:      String,
    #[serde(rename = "sentimentProb")]
	sentiment_prob: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Dependency {
	lab:  String,
	gov:  u64,
	dep:  u64,
	prob: f64,
}

#[derive(Serialize, Deserialize)]
pub struct DependencyTree {
    #[serde(rename = "sentenceId")]
	sentence_id:  u64,
    #[serde(skip_serializing_if = "String::is_empty")]
	style:        String,
	dependencies: Vec<Dependency>,
	prob:         f64,
}

#[derive(Serialize, Deserialize)]
pub struct CoreferenceRepresentantive {
	tokens: Vec<u64>,
	head:   u64,
}

#[derive(Serialize, Deserialize)]
pub struct CoreferenceReferents {
	tokens: Vec<u64>,
	head:   u64,
	prob:   f64,
}

#[derive(Serialize, Deserialize)]
pub struct Coreference {
	id:             u64,
	representative: CoreferenceRepresentantive,
	referents:      Vec<CoreferenceReferents>,
}

#[derive(Serialize, Deserialize)]
pub struct Scope {
	id:        u64,
	gov:       Vec<u64>,
	dep:       Vec<u64>,
	terminals: Vec<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct ConstituentParse {
    #[serde(rename = "sentenceId")]
	sentence_id:        u64,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "String::is_empty")]
	ctype:              String,
    #[serde(rename = "labeledBracketing")]
    #[serde(skip_serializing_if = "String::is_empty")]
	labeled_bracketing: String,
	prob:               f64,
	scopes:             Vec<Scope>,
}

#[derive(Serialize, Deserialize)]
pub struct Expression {
	id:         u64,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "String::is_empty")]
	etype:      String,
	head:       u64,
    #[serde(skip_serializing_if = "String::is_empty")]
	dependency: String,
    #[serde(rename = "tokenFrom")]
	token_from: u64,
    #[serde(rename = "tokenTo")]
	token_to:   u64,
	tokens:     Vec<u64>,
	prob:       f64,
}

#[derive(Serialize, Deserialize)]
pub struct Paragraph {
	id:         u64,
    #[serde(rename = "tokenFrom")]
	token_from: u64,
    #[serde(rename = "tokenTo")]
	token_to:   u64,
	tokens:     Vec<u64>,
	sentences:  Vec<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct Attribute {
	lab: String,
	val: String,
}

#[derive(Serialize, Deserialize)]
pub struct Entity {
	id:             u64,
    #[serde(skip_serializing_if = "String::is_empty")]
	label:          String,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "String::is_empty")]
	etype:          String,
    #[serde(skip_serializing_if = "String::is_empty")]
	url:            String,
	head:           u64,
    #[serde(rename = "tokenFrom")]
	token_from:      u64,
    #[serde(rename = "tokenTo")]
	token_to:        u64,
	tokens:         Vec<u64>,
    #[serde(rename = "tripleID")]
	triple_id:       u64,
    #[serde(skip_serializing_if = "String::is_empty")]
	sentiment:      String,
    #[serde(rename = "sentimentProb")]
	sentiment_prob: f64,
	count:          u64,
	attributes:     Vec<Attribute>,
}

#[derive(Serialize, Deserialize)]
pub struct Relation {
	id:             u64,
    #[serde(skip_serializing_if = "String::is_empty")]
	label:          String,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "String::is_empty")]
	rtype:          String,
    #[serde(skip_serializing_if = "String::is_empty")]
	url:            String,
	head:           u64,
    #[serde(rename = "tokenFrom")]
	token_from:      u64,
    #[serde(rename = "tokenTo")]
	token_to:        u64,
	tokens:         Vec<u64>,
    #[serde(skip_serializing_if = "String::is_empty")]
	sentiment:      String,
    #[serde(rename = "sentimentProb")]
	sentiment_prob: f64,
	count:          u64,
	attributes:     Vec<Attribute>,
}

#[derive(Serialize, Deserialize)]
pub struct Triple {
	id:           u64,
    #[serde(rename = "fromEntity")]
	from_entity:  u64,
    #[serde(rename = "toEntity")]
	to_entity:    u64,
	rel:          u64,
    #[serde(rename = "clauseID")]
	clause_id:    Vec<u64>,
    #[serde(rename = "sentenceID")]
	sentence_id:  Vec<u64>,
	directional:  bool,
    #[serde(rename = "eventID")]
	event_id:     u64,
    #[serde(rename = "tempSeq")]
	temp_seq:     u64,
	prob:         f64,
	syntactic:    bool,
	implied:      bool,
	presupposed:  bool,
	count:        u64,
}

#[derive(Serialize, Deserialize)]
pub struct Document {
	meta:             Meta,
	id:               u64,
    #[serde(rename = "tokenList")]
	token_list:       Vec<Token>,
	clauses:          Vec<Clause>,
	sentences:        Vec<Sentence>,
	paragraphs:       Vec<Paragraph>,
    #[serde(rename = "dependencyTrees")]
	dependency_trees: Vec<DependencyTree>,
	coreferences:     Vec<Coreference>,
	constituents:     Vec<ConstituentParse>,
	expressions:      Vec<Expression>,
	entities:         Vec<Entity>,
	relations:        Vec<Relation>,
	triples:          Vec<Triple>,
}

#[derive(Serialize, Deserialize)]
pub struct JSONNLP {
	meta: Meta,
	docs: Vec<Document>,
}

pub fn from_string() {

}

pub fn from_file() {

}

pub fn get_json() {

}
