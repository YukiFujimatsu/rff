use web_view::{WebView, WVResult};

use crate::structs::*;
use serde_json::json;

// When API Called
// Todo: d_ 系　共通処理　まとめられないかな？
pub fn d_insert(wv: &mut WebView<()>, cmd: String, db: String) -> WVResult {
    let cmd = cmd.into_cmd();
    let st = cmd.query.into_dins();
    let db: sled::Db = sled::open(db).unwrap();
    // Todo: insert と delete dbのハンドリング系
    db.insert(st.key.as_bytes(), st.value.as_bytes());
    let val = CmdSend{
        t: "dataInsert".to_string(),
        callback: cmd.cb,
        param: "".to_string()
    };
    let result = wv.eval(&format!("receiver_from_rust({})", serde_json::to_string(&val).unwrap()));
    result

}

pub fn d_fetch(wv: &mut WebView<()>, cmd: String, db: String) -> WVResult {
    let cmd = cmd.into_cmd();
    let st = cmd.query.into_dfet();
    let db: sled::Db = sled::open(db).unwrap();
    let re;
    match db.get(st.key) {
        Ok(Some(res)) => {re = String::from_utf8(res.to_vec()).unwrap();}
        _ => {re = String::from("");}
    }
    let v = json!({"v": re});
    // todo: CmdSend builder
    let val = CmdSend{
        t: "dataFetch".to_string(),
        callback: cmd.cb,
        param: serde_json::to_string(&v).unwrap()
    };
    let result = wv.eval(&format!("receiver_from_rust({})", serde_json::to_string(&val).unwrap()));
    result
}

pub fn d_delete(wv: &mut WebView<()>, cmd: String, db: String) -> WVResult {
    let cmd = cmd.into_cmd();
    let st = cmd.query.into_ddel();
    let db: sled::Db = sled::open(db).unwrap();
    db.remove(st.key);
    let val = CmdSend{
        t: "dataDelete".to_string(),
        callback: cmd.cb,
        param: String::from("")
    };
    let result = wv.eval(&format!("receiver_from_rust({})", serde_json::to_string(&val).unwrap()));
    result
}
// wv.set_title("a");
// wv.set_visibility(true); で Windowをつくる（見かけ上）
// webview.eval(&format!("notify_success({})", serde_json::to_string(&("Switched to ".to_owned() + arg)).unwrap()));
