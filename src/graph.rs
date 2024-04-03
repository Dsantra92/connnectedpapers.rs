use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub type PaperID = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommonAuthor {
    pub id: String,
    pub mention_indexes: Vec<i32>,
    pub mentions: Vec<PaperID>,
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaperAuthor {
    pub ids: Vec<Option<String>>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalIDs {
    #[serde(rename = "ACL")]
    pub acl: Option<String>,
    #[serde(rename = "ArXiv")]
    pub arxiv: Option<String>,
    #[serde(rename = "CorpusId")]
    pub corpus_id: Option<u64>,
    #[serde(rename = "DBLP")]
    pub dblp: Option<String>,
    #[serde(rename = "DOI")]
    pub doi: Option<String>,
    #[serde(rename = "MAG")]
    pub mag: Option<String>,
    #[serde(rename = "PubMed")]
    pub pub_med: Option<String>,
    #[serde(rename = "PubMedCentral")]
    pub pub_med_central: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasePaper {
    #[serde(rename = "abstract")]
    pub abstract_text: Option<String>,
    #[serde(rename = "arxivId")]
    pub arxiv_id: Option<String>,
    pub authors: Vec<PaperAuthor>,
    #[serde(rename = "corpusid")]
    pub corpus_id: i32,
    pub doi: Option<String>,
    #[serde(rename = "externalIds")]
    pub external_ids: ExternalIDs,
    #[serde(rename = "fieldsOfStudy")]
    pub fields_of_study: Option<Vec<String>>,
    pub id: PaperID,
    #[serde(rename = "isOpenAccess")]
    pub is_open_access: Option<bool>,
    #[serde(rename = "journalName")]
    pub journal_name: Option<String>,
    #[serde(rename = "journalPages")]
    pub journal_pages: Option<String>,
    #[serde(rename = "journalVolume")]
    pub journal_volume: Option<String>,
    #[serde(rename = "magId")]
    pub mag_id: Option<String>,
    pub number_of_authors: i32,
    #[serde(rename = "paperId")]
    pub paper_id: PaperID,
    #[serde(rename = "pdfUrls")]
    pub pdf_urls: Option<Vec<String>>,
    pub pmid: Option<String>,
    #[serde(rename = "publicationDate")]
    pub publication_date: Option<String>,
    #[serde(rename = "publicationTypes")]
    pub publication_types: Option<Vec<String>>,
    pub title: String,
    pub tldr: Option<String>,
    pub url: String,
    pub venue: Option<String>,
    pub year: Option<i32>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommonCitation {
    #[serde(flatten)]
    pub base_paper: BasePaper,
    pub edges_count: i32,
    pub local_references: Vec<PaperID>,
    pub paper_id: PaperID,
    pub pi_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommonReference {
    #[serde(flatten)]
    pub base_paper: BasePaper,
    pub edges_count: i32,
    pub local_citations: Vec<PaperID>,
    pub paper_id: PaperID,
    pub pi_name: Option<String>,
}

pub type Edge = (PaperID, PaperID, f32);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Paper {
    #[serde(flatten)]
    pub base_paper: BasePaper,
    pub path: Vec<PaperID>,
    pub path_length: f32,
    pub pos: (f32, f32)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Graph {
    pub common_authors: Vec<CommonAuthor>,
    pub common_citations: Vec<CommonCitation>,
    pub common_references: Vec<CommonReference>,
    pub edges: Vec<Edge>,
    pub nodes: HashMap<PaperID, Paper>,
    pub path_lengths: HashMap<PaperID, f32>,
    pub start_id: PaperID,
}
