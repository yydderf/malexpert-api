use crate::domain::pipeline::Catalog;
use crate::api::error::APIErrorBody;
use crate::services::clients::malexp::MalexpClient;

impl MalexpClient {
    pub async fn get_catalog(&self) -> Result<Catalog, APIErrorBody> {
        self.get_json::<Catalog>(crate::consts::client::malexp::CATALOG).await
    }
}

#[cfg(test)]
mod tests {
    use rocket::serde::json::serde_json;
    use super::*;

    #[test]
    fn de_catalog() {
        let raw = r#"{
            "stages":{
                "analyzer":{"models":[{"name":"BinaryAnalyzer","help":"default analyzer"}],"params":{},"description":"analyzer_description","next":[],"default":""},
                "encoder":{"models":[{"name":"SAFEEncoder","help":"SAFE neural network"},{"name":"Asm2VecEncoder","help":"asm2vec"}],"params":{},"description":"","next":[],"default":""},
                "expander":{"models":[{"name":"NoExpand","help":"returns the original embeddings and dataframe"},{"name":"GraphExpander","help":"expand the graph by recursively resolve leaf nodes"}],"params":{},"description":"","next":[],"default":""},
                "augmentor":{"models":[{"name":"NoAugment","help":"returns the original embeddings"},{"name":"LearnableAugmentor","help":"augment null nodes by mapping"}],"params":{},"description":"","next":[],"default":""},
                "detector":{"models":[{"name":"GNNModel","help":"graph neural network"}],"params":{},"description":"","next":[],"default":""},
                "explainer":{"models":[{"name":"GNNExplainer","help":"default explainer"}],"params":{},"description":"","next":[],"default":""}
            },
            "version":""
        }"#;

        let cat: Catalog = serde_json::from_str(raw).expect("should parse Catalog");
    }
}
