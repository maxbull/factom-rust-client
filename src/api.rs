use super::*;

impl Factomd{
    pub fn ablock_by_height(self, height: u32)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("height".to_string(), json!(height));
        let json = ApiRequest::method("ablock-by-height")
                                .parameters(params)
                                .to_json();
        api_call(json, self.uri())
    }

}