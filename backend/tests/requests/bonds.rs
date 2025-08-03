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

#[tokio::test]
#[serial]
async fn can_get_existing_bond_csv() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/bonds/TEST123/csv").await;

        assert_eq!(res.status_code(), 200);
        
        // Expected CSV format based on the test fixture data
        let expected_csv = "date,value\n2023-01-15,1\n2023-01-16,1.5\n2023-01-17,2\n";
        assert_eq!(res.text(), expected_csv);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_non_existing_bond_csv() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/bonds/NONEXISTENT/csv").await;

        assert_eq!(res.status_code(), 404);
        res.assert_json(&json!({
            "error": "Bond with ID NONEXISTENT not found"
        }));
    })
    .await;
}