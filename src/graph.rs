use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub type PaperID = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CommonAuthor {
    id: String,
    mention_indexes: Vec<i32>,
    mentions: Vec<PaperID>,
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PaperAuthor {
    ids: Vec<Option<String>>,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ExternalIDs {
    #[serde(rename = "ACL")]
    acl: Option<String>,
    #[serde(rename = "ArXiv")]
    arxiv: Option<String>,
    #[serde(rename = "CorpusId")]
    corpus_id: Option<String>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BasePaper {
    #[serde(rename = "abstract")]
    abstract_text: Option<String>,
    #[serde(rename = "arxivId")]
    arxiv_id: Option<String>,
    authors: Vec<PaperAuthor>,
    #[serde(rename = "corpusid")]
    corpus_id: i32,
    doi: Option<String>,
    #[serde(rename = "externalIds")]
    external_ids: ExternalIDs,
    #[serde(rename = "fieldsOfStudy")]
    fields_of_study: Option<Vec<String>>,
    id: PaperID,
    #[serde(rename = "isOpenAccess")]
    is_open_access: Option<bool>,
    #[serde(rename = "journalName")]
    journal_name: Option<String>,
    #[serde(rename = "journalPages")]
    journal_pages: Option<String>,
    #[serde(rename = "journalVolume")]
    journal_volume: Option<String>,
    #[serde(rename = "magId")]
    mag_id: Option<String>,
    number_of_authors: i32,
    #[serde(rename = "paperId")]
    paper_id: PaperID,
    #[serde(rename = "pdfUrls")]
    pdf_urls: Option<Vec<String>>,
    pmid: Option<String>,
    #[serde(rename = "publicationDate")]
    publication_date: Option<String>,
    #[serde(rename = "publicationTypes")]
    publication_types: Option<Vec<String>>,
    title: String,
    tldr: Option<String>,
    url: String,
    venue: Option<String>,
    year: Option<i32>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct CommonCitation {
    #[serde(flatten)]
    base_paper: BasePaper,
    edges_count: i32,
    local_references: Vec<PaperID>,
    paper_id: PaperID,
    pi_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CommonReference {
    #[serde(flatten)]
    base_paper: BasePaper,
    edges_count: i32,
    local_citations: Vec<PaperID>,
    paper_id: PaperID,
    pi_name: Option<String>,
}

type Edge = (PaperID, PaperID, f32);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Paper {
    #[serde(flatten)]
    base_paper: BasePaper,
    path: Vec<PaperID>,
    path_length: f32,
    pos: (f32, f32)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Graph {
    common_authors: Vec<CommonAuthor>,
    common_citations: Vec<CommonCitation>,
    common_references: Vec<CommonReference>,
    edges: Vec<Edge>,
    nodes: HashMap<PaperID, Paper>,
    path_lengths: HashMap<PaperID, f32>,
    start_id: PaperID,
}
