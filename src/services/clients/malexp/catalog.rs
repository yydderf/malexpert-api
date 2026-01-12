use crate::domain::pipeline::Catalog;
use crate::api::error::APIErrorBody;
use crate::services::clients::malexp::MalexpClient;

impl MalexpClient {
    pub async fn get_catalog(&self) -> Result<Catalog, APIErrorBody> {
        self.get_json::<Catalog>(crate::consts::client::MALEXP_API_CATALOG).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn de_catalog() {
        let raw = r#"{
            "stages":{
                "analyzer":{"models":[{"name":"BinaryAnalyzer","help":"default analyzer"}],"params":{}},
                "encoder":{"models":[{"name":"SAFEEncoder","help":"SAFE neural network"},{"name":"Asm2VecEncoder","help":"asm2vec"}],"params":{}},
                "expander":{"models":[{"name":"NoExpand","help":"returns the original embeddings and dataframe"},{"name":"GraphExpander","help":"expand the graph by recursively resolve leaf nodes"}],"params":{}},
                "augmentor":{"models":[{"name":"NoAugment","help":"returns the original embeddings"},{"name":"LearnableAugmentor","help":"augment null nodes by mapping"}],"params":{}},
                "detector":{"models":[{"name":"GNNModel","help":"graph neural network"}],"params":{}},
                "explainer":{"models":[{"name":"GNNExplainer","help":"default explainer"}],"params":{}}
            },
            "version":""
        }"#;

        let cat: Catalog = serde_json::from_str(raw).expect("should parse Catalog");
    }
}
