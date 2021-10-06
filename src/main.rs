use scraper::{node::Node, Html};
use structopt::StructOpt;
use url::Url;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The url of the recipe
    /// example: `https://www.disneyfoodblog.com/2021/04/03/10-epcot-recipes-you-need-to-make-now-like-right-now/`
    url: Url,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let r = reqwest::get(opt.url).await?;

    let document = Html::parse_document(r.text().await?.as_str());

    document
        .tree
        .nodes()
        .filter_map(|child| {
            if let Node::Element(el) = child.value() {
                return Some((child, el));
            }
            None
        })
        .filter(|(_child, el)| el.name() == "li")
        .for_each(|(child, _el)| println!("{:?}", child));

    Ok(())
}
