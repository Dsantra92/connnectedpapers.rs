use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::Value as JsonValue;

type PaperID = String;

#[derive(Serialize, Deserialize, Debug)]
struct CommonAuthor {
    id: String,
    mention_indexes: Vec<i32>,
    mentions: Vec<PaperID>,
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PaperAuthor {
    ids: Vec<Option<String>>,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExternalIDs {
    #[serde(rename = "ACL")]
    acl: Option<String>,
    #[serde(rename = "ArXiv")]
    arxiv: Option<String>,
    #[serde(rename = "CorpusId")]
    corpus_id: Option<JsonValue>,
    #[serde(rename = "DBLP")]
    dblp: Option<String>,
    #[serde(rename = "DOI")]
    doi: Option<String>,
    #[serde(rename = "MAG")]
    mag: Option<String>,
    #[serde(rename = "PubMed")]
    pub_med: Option<String>,
    #[serde(rename = "PubMedCentral")]
    pub_med_central: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct BasePaper {
    #[serde(rename = "abstract")]
    _abstract: Option<String>,
    arxiv_id: Option<String>,
    authors: Vec<PaperAuthor>,
    corpusid: i32,
    doi: Option<String>,
    external_ids: ExternalIDs,
    fields_of_study: Option<Vec<String>>,
    id: PaperID,
    is_open_access: Option<bool>,
    journal_name: Option<String>,
    journal_pages: Option<String>,
    journal_volume: Option<String>,
    mag_id: Option<String>,
    number_of_authors: i32,
    paper_id: PaperID,
    pdf_urls: Option<Vec<String>>,
    pmid: Option<String>,
    publication_date: Option<String>,
    publication_types: Option<Vec<String>>,
    title: String,
    tldr: Option<String>,
    url: String,
    venue: Option<String>,
    year: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CommonCitation {
    #[serde(flatten)]
    base_paper: BasePaper,
    edges_count: i32,
    local_references: Vec<PaperID>,
    paper_id: PaperID,
    pi_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CommonReference {
    #[serde(flatten)]
    base_paper: BasePaper,
    edges_count: i32,
    local_citations: Vec<PaperID>,
    paper_id: PaperID,
    pi_name: Option<String>,
}

type Edge = (PaperID, PaperID, f32);

#[derive(Serialize, Deserialize, Debug)]
struct Paper {
    #[serde(flatten)]
    base_paper: BasePaper,
    path: Vec<PaperID>,
    path_length: f32,
    pos: (f32, f32)
}

#[derive(Serialize, Deserialize, Debug)]
struct Graph {
    common_authors: Vec<CommonAuthor>,
    common_citations: Vec<CommonCitation>,
    common_references: Vec<CommonReference>,
    edges: Vec<Edge>,
    nodes: HashMap<PaperID, Paper>,
    path_lengths: HashMap<PaperID, f32>,
    start_id: PaperID,
}
