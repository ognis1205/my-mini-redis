use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];
    let n = f.read(&mut buffer[..]).await?;
    println!("The bytes: {:?}", &buffer[..n]);

    let mut f = File::open("foo.txt").await?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).await?;
    println!("The bytes: {:?}", &buffer[..n]);

    let mut file = File::create("bar.txt").await?;
    let n = file.write(b"some bytes").await?;
    println!("Wrote the first {} bytes of 'some bytes'.", n);

    Ok(())
}
