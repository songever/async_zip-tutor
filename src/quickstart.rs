use async_zip::base::read::seek::ZipFileReader;
use async_zip::base::write::ZipFileWriter;
use async_zip::error::ZipError;
use async_zip::{Compression, ZipEntryBuilder, ZipString};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};
use tokio_util::compat::{FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt};
#[tokio::main]
async fn main() -> Result<(), ZipError> {
    let zip_entry = ZipEntryBuilder::new(ZipString::from("foo.txt"), Compression::Stored).build();
    // println!("ZipEntry: {zip_entry:#?}");

    let mut writer = ZipFileWriter::new(Vec::<u8>::new());
    let data = b"Let's see how to create a zip compress file with crate async_zip";
    writer.write_entry_whole(zip_entry, data).await?;
    let zip_data = writer.close().await?;
    tokio::fs::write("foo.zip", &zip_data).await?;

    let zip_data = BufReader::new(File::open("foo.zip").await?);
    let mut reader = ZipFileReader::new(zip_data.compat()).await?;

    let mut data = Vec::new();
    let mut entry = reader.reader_without_entry(0).await?.compat();
    entry.read_to_end(&mut data).await?;

    println!("Data: {}", String::from_utf8_lossy(&data));
    Ok(())
}
