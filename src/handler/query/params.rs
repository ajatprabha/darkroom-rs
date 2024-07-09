use serde::Deserialize;
use crate::handler::query::auto::AutoFeature;
use crate::handler::query::crop::Crop;
use crate::handler::query::fit::Fit;
use crate::handler::query::monochrome::MonoChrome;
use crate::handler::query::rotate::Rotate;
use crate::handler::query::vec::CommaSeparatedVec;

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct ProcessParams {
    #[serde(rename = "w")]
    pub width: Option<u16>,
    #[serde(rename = "h")]
    pub height: Option<u16>,

    pub fit: Option<Fit>,
    pub crop: Option<Crop>,
    pub flip: Option<bool>,

    #[serde(rename = "rot")]
    pub rotate: Option<Rotate>,

    #[serde(rename = "auto")]
    pub auto_features: Option<CommaSeparatedVec<AutoFeature>>,

    pub monochrome: Option<MonoChrome>,
}

#[cfg(test)]
mod tests {
    use axum::extract::Query;
    use axum::http::Uri;
    use super::*;

    #[test]
    fn test_query_params() {
        let uri: Uri = "https://example.com/path/to/image?fit=crop&crop=top,left&w=100&h=200".parse().unwrap();
        let params: Query<ProcessParams> = Query::try_from_uri(&uri).unwrap();
        assert_eq!(params.fit, Some(Fit::Crop));
        assert_eq!(params.crop, Some(Crop::TopLeft));
        assert_eq!(params.width, Some(100));
        assert_eq!(params.height, Some(200));
    }

    #[test]
    fn test_query_params_auto() {
        let uri: Uri = "https://example.com/path/to/image?auto=compress,format&w=100&h=200".parse().unwrap();
        let params: Query<ProcessParams> = Query::try_from_uri(&uri).unwrap();
        assert_eq!(params.auto_features.as_deref(), Some(vec![AutoFeature::Compress, AutoFeature::Format].as_ref()));
        assert_eq!(params.width, Some(100));
        assert_eq!(params.height, Some(200));
    }

    #[test]
    fn test_query_params_monochrome() {
        let uri: Uri = "https://example.com/path/to/image?monochrome=000000".parse().unwrap();
        let params: Query<ProcessParams> = Query::try_from_uri(&uri).unwrap();
        assert_eq!(params.monochrome, Some(MonoChrome::RGB(0, 0, 0)));
    }
}