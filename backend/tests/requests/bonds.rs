use insta::assert_csv_snapshot;
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
        res.assert_json(&json!([
            "EDO0732", "EDO0835", "EDO1014", "ROD0832", "ROD0837", "ROD1028"
        ]));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_existing_bond_csv() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/bonds/ROD0837/csv").await;
        assert_eq!(res.status_code(), 200);
        assert_csv_snapshot!(res.text())
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
