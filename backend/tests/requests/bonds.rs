use loco_rs::testing::prelude::*;
use myapp::app::App;
use pretty_assertions::assert_eq;
use serde_json::json;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_get_bonds() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/bonds").await;

        assert_eq!(res.status_code(), 200);
        res.assert_json(&json!(["TEST123"]));
    })
    .await;
}
