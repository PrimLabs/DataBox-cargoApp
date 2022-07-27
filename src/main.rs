use std::env::args;
use std::io;
use candid::parser::test::HostAssert::Decode;
use candid::{CandidType, Decode, Encode};
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::Agent;
use ic_types::Principal;
use serde::Deserialize;
use crate::FileExt::PlainFileExt;
use crate::FileExt::EncryptFileExt;
use clap::{App, Arg};

#[derive(CandidType, Deserialize)]
enum DataErr {
  FileKeyErr,
  FilePublic,
  BlobSizeError,
  PermissionDenied,
  SharedRepeat,
  FlagErr,
  SharedNotSet,
  MemoryInsufficient,
  FileAesPubKeyNotExist,
  UserAccessErr,
  DeviceNotExist,
  ShareRepeat,
}

#[derive(CandidType, Deserialize)]
struct AssetExt {
  file_extension: String,
  upload_status: bool,
  bucket_id: candid::Principal,
  aes_pub_key: Option<String>,
  file_name: String,
  file_key: String,
  total_size: u64,
  need_query_times: candid::Nat,
}

#[derive(CandidType, Deserialize)]
enum FileExt {
  EncryptFileExt(AssetExt),
  SharedFileExt{
    file_extension: String,
    other: candid::Principal,
    description: String,
    file_name: String,
    file_key: String,
    isPublic: bool,
  },
  PlainFileExt(AssetExt),
}

#[derive(CandidType, Deserialize)]
enum Result_9 { ok(Vec<FileExt>,Vec<FileExt>,Vec<FileExt>,), err(DataErr) }

// type DataBox = candid::Service;
// struct SERVICE(candid::Principal);
// impl SERVICE{
//   pub async fn getAssetexts(&self) -> CallResult<(Result_9,)> {
//     ic_cdk::call(self.0, "getAssetexts", ()).await
//   }
// }

#[tokio::main]
async fn main() {
    let matches = App::new("databox command app")
        .version("0.1.0")
        .author("xiaoyuanxun")
        .about("Help Dev to use databox")
        .arg(
            Arg::with_name("canister_id")
                .short('c')
                .long("canister")
                .takes_value(true)
                .help("databox you want to query 's principal"),
        )
        .get_matches();
    let canister_id = Principal::from_text(
        matches
                .value_of("canister_id")
                .expect("please specify the canister's principal"),
    ).expect("get principal failed");
    query_allFile(&canister_id).await;
}

async fn query_allFile(canister_id: &Principal) -> () {
    let url = "https://ic0.app".to_string();
    let transport = ReqwestHttpReplicaV2Transport::create(url).unwrap();
    let agent = Agent::builder()
        .with_transport(transport)
        .build()
        .expect("build agent error");
    let _ = agent.fetch_root_key();
    let response = agent
        .query(canister_id, "getAssetexts")
        .with_arg(Encode!().unwrap())
        .call()
        .await
        .expect("getAssetexts failed");
    let res = Decode!(&response, Result_9).unwrap();
    match res {
        Result_9::ok(x, y, z) => {
            for ans in x {
                match ans {
                    PlainFileExt(asExt) => {
                        println!("{}",asExt.file_key);
                    }
                    EncryptFileExt(bb) => {
                        println!("err");
                    }
                    cc => {
                        println!("err");
                    }
                }
            }
        }
        Result_9::err(e) => {
            println!("err");
        }
    };
}