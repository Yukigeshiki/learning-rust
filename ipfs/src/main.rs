use std::io;
use std::io::Write;
use futures::TryStreamExt;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};

#[tokio::main]
async fn main() {
    let client = IpfsClient::default(); // connect to local node - tested using desktop app
    // let data = Cursor::new("Hello World!");
    //
    // match client.add(data).await {
    //     Ok(res) => println!("{}", res.hash),
    //     Err(e) => eprintln!("error adding file: {}", e)
    // }

    match client
        .get("/ipfs/QmcserdMRTjsbp7X4bUJ1TBn95Bi9KXehgFgMnAPCoXGoG")
        .map_ok(|chunk| { chunk.to_vec() })
        .try_concat()
        .await
    {
        Ok(res) => {
            let out = io::stdout();
            let mut out = out.lock();

            out.write_all(&res).unwrap();
        }
        Err(e) => eprintln!("error getting file: {}", e)
    }
}
