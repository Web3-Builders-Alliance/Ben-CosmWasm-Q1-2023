use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use pool_network::asset::{Asset, AssetInfo, AssetInfoRaw, AssetRaw, PairInfo, PairInfoRaw};
use std::env::current_dir;
use std::fs::create_dir_all;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(Asset), &out_dir);
    export_schema(&schema_for!(AssetInfo), &out_dir);
    export_schema(&schema_for!(AssetRaw), &out_dir);
    export_schema(&schema_for!(AssetInfoRaw), &out_dir);
    export_schema(&schema_for!(PairInfo), &out_dir);
    export_schema(&schema_for!(PairInfoRaw), &out_dir);
}
