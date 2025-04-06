use serde::{Serialize, Deserialize};

type IssueId = i64;

#[derive(Serialize, Deserialize)]
pub struct Issue {
  id: IssueId,
  description: String
}

#[derive(Serialize, Deserialize)]
pub struct Issues {

}

impl Issue {

}

impl Issues {

}

