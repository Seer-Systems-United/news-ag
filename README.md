# news-ag

A comprehensive news aggregator library for Rust. It provides a unified API for fetching articles and content from over 170 news outlets, with more sources being added regularly.

## Usage

```rust
use news_ag::{
    ApNews,
    source::{Source, endpoint::EndpointScope},
};

#[tokio::main]
async fn main() {
    // Select an endpoint
    let endpoint = ApNews::get_endpoint(EndpointScope::World)
        .expect("AP News should define a world endpoint");

    // Fetch current articles
    let articles = endpoint.get_articles().await;

    // Retrieve content from an article
    let content = articles[0]
        .get_content()
        .await
        .expect("Article content should be available");

    println!("Articles from AP News: {articles:#?}");
    println!("First article content: {content}");
}
```

## Features

- **`async`** (optional): Enables the asynchronous API. When disabled, `get_articles()` and `get_content()` are blocking calls.
- **`rkyv`**: Provides zero-copy serialization and deserialization support for articles and endpoint metadata.
- **`serde`**: Provides serialization and deserialization support for articles and endpoint metadata.

## Source IDs

Every source implements `Source::id()`, which returns a deterministic UUID derived from its assigned number. The numbering scheme organizes sources into blocks of 100:

| Range | Category |
|-------|----------|
| 001–099 | Global News & Major Networks |
| 100–199 | Business & Finance |
| 200–299 | Technology & Science |
| 300–399 | Regional: United States |
| 400–499 | Regional: Canada |
| 500–599 | Regional: United Kingdom & Ireland |
| 600–699 | Regional: Asia, Oceania & Middle East |
| 700–799 | Regional: Europe & Latin America |
| 800–899 | Politics, Opinion & Commentary |
| 900–999 | Arts, Entertainment & Gaming |

The IDs are UUIDv5 values generated from a project namespace and the source number, making them stable and reproducible:

```rust
use news_ag::source::Source;

let id = news_ag::ApNews::id(); // ce8ea65b-85a6-5fee-8789-98fd53887c5a (AP News)
```

To add a new source, assign the next available number in its category and hardcode the corresponding UUID in `src/source/feed.rs`'s `source_id()` match table. Use a tool such as `python3 -c "import uuid; print(uuid.uuid5(uuid.UUID('6ba7b810-9dad-11d1-80b4-00c04fd430c8'), 'news-sources:YOUR_NUMBER'))"` to generate the new UUID.

## Supported Sources

`(*)` Indicates that article listings are available, but full article content cannot currently be retrieved anonymously from this source.

### Global News & Major Networks
- [x] [AP News](https://apnews.com/) — UUID: `ce8ea65b-85a6-5fee-8789-98fd53887c5a`
- [ ] [The Associated Press](https://www.ap.org/) — UUID: `c15e7183-d48e-588e-8804-c40d65c7cbd1`
- [x] [Reuters](https://www.reuters.com/) — UUID: `b7487efe-b9d5-51da-951d-92f2af5ca190`
- [x] [BBC News](https://www.bbc.com/news) — UUID: `e88c96aa-45b3-5523-8d35-0fb4083c68b1`
- [x] [CNN](https://edition.cnn.com/) — UUID: `ae3f8985-03ff-538e-8442-c8f8e822fdb1`
- [x] [ABC News](https://abcnews.go.com/) — UUID: `fb5030ee-e573-5b33-bb99-3b0f10dfcd46`
- [x] [CBS News](https://www.cbsnews.com/) — UUID: `d776156e-a136-5e83-af40-dd08102a0ce7`
- [x] [NBC News](https://www.nbcnews.com/) — UUID: `026a4ff4-eb40-5212-8824-cbe3ef578277`
- [x] [USA Today](https://www.usatoday.com/) — UUID: `f9dd7397-bdbd-58a6-8956-d469fcbb281e`
- [x] [Sky News](https://news.sky.com/) — UUID: `8409b28c-693b-5061-9943-31072d2a37c4`
- [x] [France 24](https://www.france24.com/en/) — UUID: `fdb1f043-ee0a-513f-bdc4-874c5cd996ba`
- [x] [Deutsche Welle](https://www.dw.com/en/) — UUID: `b3a8ffa6-df24-556b-8976-cb658a04426d`
- [x] [Al Jazeera](https://www.aljazeera.com/) — UUID: `96cf30fe-a1fc-5645-89d3-8d15c33f7c7d`
- [x] [NPR](https://www.npr.org/) — UUID: `5ece0ca7-52d1-5ba8-813b-3152b16d17ac`
- [ ] [Agence France-Presse (AFP)](https://www.afp.com/) — UUID: `28fa1544-8079-5e94-a47d-449039028a26`
- [x] [United Press International](https://www.upi.com/) — UUID: `6a7d3b7d-6568-5a07-948d-cc556bd683a3`
- [x] [Euronews](https://www.euronews.com/) — UUID: `6cf3a2aa-8c16-5a2e-8a03-773025fbcdb6`
- [x] [Al-Monitor](https://www.al-monitor.com/) — UUID: `e6f7c900-2fee-51aa-9746-a7e07155397e`

### Business & Finance
- [x] [The Wall Street Journal](https://www.wsj.com/) — UUID: `b5c969f2-c7d9-5ed6-bbc7-cfbd4920d7fb`
- [x] [The Economist](https://www.economist.com/) — UUID: `31443912-1b21-58f9-b23c-aa0d31c00206`
- [x] [Bloomberg](https://www.bloomberg.com/) — UUID: `a633de5f-5239-55f9-b0db-bec202b631c4`
- [x] [Financial Times](https://www.ft.com/) — UUID: `314261c1-d2d5-50bc-bcff-1fcdece1d289`
- [x] [CNBC](https://www.cnbc.com/) — UUID: `8f1afa95-f295-58dd-81a6-2f0c32eddac0`
- [x] [Business Insider](https://www.businessinsider.com/) — UUID: `2c4b6fd2-9dff-5398-a603-eeecbe0c2a02`
- [x] [Fortune](https://fortune.com/) — UUID: `471de1dd-0645-5034-b09d-c75ecd6dcc3f`
- [x] [Forbes](https://www.forbes.com/) — UUID: `1544049c-6e9b-5d1a-941e-2f244b2c17fe`
- [x] [Quartz](https://qz.com/) — UUID: `091aae1b-7573-5909-ae17-65877f90986e`
- [x] [MarketWatch](https://www.marketwatch.com/) — UUID: `3b9334d0-7cec-5957-93d3-03558570afad`
- [ ] [Investor's Business Daily](https://www.investors.com/) — UUID: `bc74137d-aa13-504d-9fca-b2276d071aa2`
- [x] [Fast Company](https://www.fastcompany.com/) — UUID: `7b211dbd-bb97-5afa-81e4-2ac19776dd56`
- [x] [Kiplinger](https://www.kiplinger.com/) — UUID: `06e5dc9c-f2b7-5af0-9052-ecd2ab02c790`

### Technology & Science
- [x] [Wired](https://www.wired.com/) — UUID: `a4888510-6377-57d4-9266-b187c9d0cf2c`
- [x] [Ars Technica](https://arstechnica.com/) — UUID: `dd7f5adc-8d60-5f82-b8c7-4187d38e8988`
- [x] [TechCrunch](https://techcrunch.com/) — UUID: `f3aead82-0d45-516e-98f5-c8e37822cba6`
- [x] [Engadget](https://www.engadget.com/) — UUID: `0b3f5d41-da07-5480-afff-ae4d83bbac8f`
- [x] [The Verge](https://www.theverge.com/) — UUID: `d7ddb8c2-c94b-5e5c-a4f2-f0f3b7195159`
- [x] [Gizmodo](https://gizmodo.com/) — UUID: `5ca3113a-9977-540c-8602-dc20ae8bfdb9`
- [x] [Science Magazine](https://www.science.org/) — UUID: `27622844-8147-55e8-8c69-eb80f902324b`
- [x] [Nature](https://www.nature.com/) — UUID: `68b74f02-10da-5545-9424-15673e7e9197`
- [x] [Scientific American](https://www.scientificamerican.com/) — UUID: `e3b278ef-0be8-5f5c-8c81-3c62c973c331`
- [x] [New Scientist](https://www.newscientist.com/) — UUID: `b8b42123-120d-59e2-9973-81af3477a413`
- [ ] [National Geographic](https://www.nationalgeographic.com/) — UUID: `4fef77ae-a3a5-5f88-8782-35ef260dd8f6`
- [x] [CNET](https://www.cnet.com/) — UUID: `9980e7c7-5810-5cc2-82c5-a0cd9e4bffeb`
- [x] [PCMag](https://www.pcmag.com/) — UUID: `69ea7ec5-2328-57f7-8358-b6dbc91a2113`
- [x] [VentureBeat](https://venturebeat.com/) — UUID: `da8e3f48-eb7f-5bf4-bb12-7f265338040a`
- [x] [Mashable](https://mashable.com/) — UUID: `079e001b-bcc6-590c-a1a1-b3d5e278edc0`

### Regional: United States
- [x] [New York Times](https://www.nytimes.com/) — UUID: `1a56ce8d-67a2-58fa-89be-eae50be6982a`
- [x] [The Washington Post](https://www.washingtonpost.com/) — UUID: `667cdada-06b0-51a4-b33b-053412baa71a`
- [x] [The Chicago Tribune](https://www.chicagotribune.com/) — UUID: `d0c23604-79ad-5209-a6fd-6482232363fd`
- [x] [The Los Angeles Times](https://www.latimes.com/) — UUID: `adaf2dec-d18a-5fab-889f-8eef35722ae1`
- [ ] [The Miami Herald](https://www.miamiherald.com/) — UUID: `5484d07b-452a-563d-b85c-841e0f784438`
- [x] [The Houston Chronicle](https://www.houstonchronicle.com/) — UUID: `44b12633-1600-53e9-8958-ce4d0b3a0dc7`
- [x] [The Philadelphia Inquirer](https://www.inquirer.com/) — UUID: `438e8740-72c7-5e65-9008-1739274f8e9d`
- [x] [The Atlanta Journal-Constitution](https://www.ajc.com/) — UUID: `860bb1af-1a3b-5060-9b7e-aaf459539a0d`
- [x] [The Dallas Morning News](https://www.dallasnews.com/) — UUID: `8745886f-a780-523c-810d-5e449cb6b59a`
- [x] [The Denver Post](https://www.denverpost.com/) — UUID: `c7c87f74-442e-55d3-8a83-8780c4423416`
- [x] [The Seattle Times](https://www.seattletimes.com/) — UUID: `4f97d87a-a4e5-50aa-a3ec-4a8788136f7b`
- [x] [The Boston Globe](https://www.bostonglobe.com/) — UUID: `98759f03-9cc9-53a9-8c34-b1ebdb5b9136`
- [x] [The Detroit Free Press](https://www.freep.com/) — UUID: `02920e57-d4a3-56bc-8d21-d2cd9e2441f2`
- [x] [The Minneapolis Star Tribune](https://www.startribune.com/) — UUID: `a5b15c9c-3eb8-507e-8704-f9c993e2d25b`
- [x] [The Cleveland Plain Dealer](https://www.cleveland.com/) — UUID: `0e2b9d69-991c-5606-aedd-227a5d68d820`
- [x] [The Tampa Bay Times](https://www.tampabay.com/) — UUID: `cd3d3661-941e-5093-a47f-8d49f277efd7`
- [x] [The Orlando Sentinel](https://www.orlandosentinel.com/) — UUID: `4f05d2e4-71dc-5e9b-8ffa-96f2efa335ee`
- [x] [The Pittsburgh Post-Gazette](https://www.post-gazette.com/) — UUID: `64348b3e-bb92-5e21-90fc-e08978c07f16`
- [x] [The Cincinnati Enquirer](https://www.cincinnati.com/) — UUID: `382b9717-720d-5935-96f3-ffce8bcb2702`
- [x] [The St. Louis Post-Dispatch](https://www.stltoday.com/) — UUID: `78d4a04d-36ef-5f97-948b-3f52829c4b1c`
- [x] [The Baltimore Sun](https://www.baltimoresun.com/) — UUID: `f23aca1b-a922-5429-972d-c5abf3f7ce49`
- [ ] [The Kansas City Star](https://www.kansascity.com/) — UUID: `3a91b4ed-e75e-5eaa-81b2-7bb891bacafd`
- [x] [The Columbus Dispatch](https://www.dispatch.com/) — UUID: `9a058af1-bf98-56dd-8ae8-db19378c397a`
- [x] [The Indianapolis Star](https://www.indystar.com/) — UUID: `0e77a4c7-5915-59c3-89bd-51c06a250d32`
- [x] [The Louisville Courier-Journal](https://www.courier-journal.com/) — UUID: `6b671f91-dd84-51f8-b908-ea22f3d34603`
- [x] [The Nashville Tennessean](https://www.tennessean.com/) — UUID: `bcbc00f2-50cd-524b-93ed-0a1fe02cc879`
- [x] [The Oklahoma City Oklahoman](https://www.oklahoman.com/) — UUID: `090ca54b-04c4-55e5-bf8f-cad453148368`
- [ ] [The Raleigh News & Observer](https://www.newsobserver.com/) — UUID: `8ea8830f-611d-540a-a3b5-235db9610b85`
- [x] [The Richmond Times-Dispatch](https://www.richmond.com/) — UUID: `9da8d2d1-1c79-576f-b4bc-46658e7144d7`
- [x] [The Hartford Courant](https://www.courant.com/) — UUID: `10285a70-8e5c-5166-8feb-8ae457deafff`
- [x] [The Providence Journal](https://www.providencejournal.com/) — UUID: `628a3ebf-7bad-56a3-98d2-58d8cad5b2fe`
- [x] [The Charleston Gazette-Mail](https://www.wvgazettemail.com/) — UUID: `d0134862-7f3d-5f9f-a0de-c432be2917fd`
- [x] [The Charleston Post and Courier](https://www.postandcourier.com/) — UUID: `901062d1-dcce-5798-94bf-750a59805045`
- [x] [The Charleston Gazette](https://www.wvgazettemail.com/) — UUID: `9393b164-92cd-58be-8759-c22144a27410`
- [x] [WGN](https://wgntv.com/) — UUID: `64215c66-6a5a-5659-b16b-cc546a39c531`
- [ ] [San Francisco Chronicle](https://www.sfchronicle.com/) — UUID: `63370d04-23b9-5846-a655-63aa666c1331`
- [ ] [The Arizona Republic](https://www.azcentral.com/) — UUID: `c57f6a76-5110-5cb7-ba1d-18b5a3f2ac27`
- [x] [The Oregonian](https://www.oregonlive.com/) — UUID: `e8354b61-7f45-5f50-9812-cfb52ca2e1e1`
- [x] [The Star-Ledger](https://www.nj.com/starledger/) — UUID: `29b24375-d3ef-5175-9966-c3aeb706b6d0`

### Regional: Canada
- [x] [The Globe and Mail](https://www.theglobeandmail.com/) — UUID: `c81b1b79-80af-55cf-b306-61ffc6b3c8f4`
- [x] [The Toronto Star](https://www.thestar.com/) — UUID: `9b51c413-a2b1-5e60-8b6a-0036acc5808c`
- [x] [The National Post](https://nationalpost.com/) — UUID: `568b354a-4733-5231-818f-3f6746ee9cbc`
- [x] [The Vancouver Sun](https://vancouversun.com/) — UUID: `0d00d7b2-6ff5-5ed5-a5c8-9cd054203f05`
- [x] [The Calgary Herald](https://calgaryherald.com/) — UUID: `3e34f9d3-e56e-5861-b6e7-a38bd124cf7a`
- [x] [The Edmonton Journal](https://edmontonjournal.com/) — UUID: `abf340ee-21fa-57e6-8ac3-c9146031de0d`
- [x] [The Montreal Gazette](https://montrealgazette.com/) — UUID: `026cd877-3b27-525a-81b6-718a4fb27058`
- [x] [The Ottawa Citizen](https://ottawacitizen.com/) — UUID: `ab73c916-f6d6-52b2-8e04-ec3e65fe1604`
- [x] [The Winnipeg Free Press](https://www.winnipegfreepress.com/) — UUID: `e8dca6af-e87b-5578-865e-c196c6fe96d5`
- [x] [The Halifax Chronicle Herald](https://www.thechronicleherald.ca/) — UUID: `c3a52872-b6b0-5a89-9b09-79f45be66f0e`
- [x] [The Regina Leader-Post](https://leaderpost.com/) — UUID: `d639e6d6-a20a-5c23-b695-3a3b3814ba9a`
- [x] [The Saskatoon StarPhoenix](https://thestarphoenix.com/) — UUID: `3193a34e-cd6c-5171-9ca3-1510474659cf`
- [x] [The St. John's Telegram](https://www.thetelegram.com/) — UUID: `dbf2be1c-15a1-5f3d-9db7-1368df16637b`
- [x] [Global News](https://globalnews.ca/) — UUID: `b126d47e-a5a8-5608-9a8f-ffbf3a4cc8f1`
- [ ] [CTV News](https://www.ctvnews.ca/) — UUID: `692d12cf-6f51-54c9-9f81-bdadaa2e96c5`
- [x] [La Presse](https://www.lapresse.ca/) — UUID: `3c16cac4-68f5-5ec0-a1f6-b28d6c4ebd1b`

### Regional: United Kingdom & Ireland
- [x] [The Guardian](https://www.theguardian.com/international) — UUID: `d8694288-00fa-5a25-835a-37ccd14dfcd0`
- [x] [The Independent](https://www.independent.co.uk/) — UUID: `270d3930-a471-582c-9e49-469b7ba637e9`
- [x] [The Times](https://www.thetimes.co.uk/) — UUID: `e1d16561-ebd2-5a2c-a967-d90511870cea`
- [x] [The Telegraph](https://www.telegraph.co.uk/) — UUID: `2409fba9-a371-5684-8c28-b4b8174cfac4`
- [x] [The Sun](https://www.thesun.co.uk/) — UUID: `1fbeb8c9-c18d-5a9c-9d74-5b73680a956b`
- [x] [The Mirror](https://www.mirror.co.uk/) — UUID: `673bf53a-1aed-58d5-b33f-f81d77d8f2d8`
- [x] [The Daily Mail](https://www.dailymail.co.uk/) — UUID: `05467408-e747-59fa-bc8e-ea6301078ba4`
- [x] [The Express](https://www.express.co.uk/) — UUID: `7bfe0775-17d1-5c0c-b058-b74651051ecc`
- [x] [The Irish Times](https://www.irishtimes.com/) — UUID: `977b8325-b25e-50f3-93e3-c9d151fe5134`
- [x] [The Herald](https://www.heraldscotland.com/) — UUID: `43f72e82-9a8b-5784-9879-2bfe14ba9d7b`
- [x] [The Belfast Telegraph](https://www.belfasttelegraph.co.uk/) — UUID: `df6e3463-325d-5c52-9455-6be0c9a04d82`
- [x] [The Scotsman](https://www.scotsman.com/) — UUID: `be9b1712-0063-523b-9155-dff7e55d3a4b`

### Regional: Asia, Oceania & Middle East
- [x] [South China Morning Post](https://www.scmp.com/) — UUID: `bd4f3154-91b7-5934-abdd-f1729c0a6f35`
- [x] [The Japan Times](https://www.japantimes.co.jp/) — UUID: `22d4af94-d056-502f-b84a-60222bc937d8`
- [x] [The Times of India](https://timesofindia.indiatimes.com/) — UUID: `17830b27-da99-56c0-82bb-fa0b3a8c6e82`
- [x] [The Straits Times](https://www.straitstimes.com/) — UUID: `14bbc696-f642-5594-83a6-a59e566b6ddf`
- [x] [Nikkei Asia](https://asia.nikkei.com/) — UUID: `f7731152-33e1-558f-85e0-46f5caf0f2c1`
- [x] [The Daily Telegraph](https://www.dailytelegraph.com.au/) — UUID: `acb652ba-3430-5479-8cc7-ba618e2c5a0e`
- [x] [The Sydney Morning Herald](https://www.smh.com.au/) — UUID: `b9405589-4d4c-52d1-a75a-618650f04e5d`
- [x] [The Age](https://www.theage.com.au/) — UUID: `0b19ded4-0ced-55ef-b48b-6fc31cfb538f`
- [x] [The Australian](https://www.theaustralian.com.au/) — UUID: `027d28e0-d9ea-5e5e-aa05-4016b9cf2353`
- [ ] [The New Zealand Herald](https://www.nzherald.co.nz/) — UUID: `24c43007-21ec-5f78-8df3-0b0aebf01a0b`
- [x] [Haaretz](https://www.haaretz.com/) — UUID: `5ec39399-4977-5710-8df8-98511b50b1ae`
- [x] [The Jerusalem Post](https://www.jpost.com/) — UUID: `a7cf0b4a-5bc0-5eff-a605-8961b77472fd`
- [x] [Middle East Eye](https://www.middleeasteye.net/) — UUID: `1b0b5468-a53a-58f5-a8dc-7ff310597285`
- [ ] [The Korea Herald](https://www.koreaherald.com/) — UUID: `4a6827f8-8453-5c8a-b54d-5959788fc604`
- [x] [The Bangkok Post](https://www.bangkokpost.com/) — UUID: `26f262c2-db86-5eba-b785-e95dc23d150c`
- [x] [The Hindu](https://www.thehindu.com/) — UUID: `1063a172-d91a-546a-8e45-ffcf61be9add`

### Regional: Europe & Latin America
- [x] [El País](https://elpais.com/elpais/inenglish.html) — UUID: `a8211257-cb6a-50c7-8422-1077c64006ea`
- [x] [Le Monde](https://www.lemonde.fr/en/) — UUID: `403add74-fd09-59bd-9459-47d4725fdb9b`
- [x] [Der Spiegel](https://www.spiegel.de/international/) — UUID: `f0ffb230-b07e-5568-b381-7ae03b8bedbe`
- [x] [The Kyiv Independent](https://kyivindependent.com/) — UUID: `e64a0cde-e848-5c56-902b-fc772d0ced18`
- [x] [The Moscow Times](https://www.themoscowtimes.com/) — UUID: `3eebd8d8-f842-5110-b9ac-9e4df69c2f51`
- [x] [Folha de S.Paulo](https://www.folha.uol.com.br/) — UUID: `493c7e12-dcff-5dce-bf87-e5537c90095c`
- [x] [Clarín](https://www.clarin.com/) — UUID: `512abf50-d22d-59b3-bd0f-c271d00dd0b5`
- [x] [Corriere della Sera](https://www.corriere.it/english/) — UUID: `1eb6d2d4-08b1-5460-ba1e-4231624b354f`
- [x] [Frankfurter Allgemeine Zeitung](https://www.faz.net/english/) — UUID: `cbf5e8e7-c63e-53ee-b94d-b8b88ee4138d`
- [x] [La Repubblica](https://www.repubblica.it/) — UUID: `dd73f6ac-ca9d-5840-8445-6d8025860fa6`

### Politics, Opinion & Commentary
- [x] [The New Yorker](https://www.newyorker.com/) — UUID: `0e5a2977-d96d-5d77-b645-bab609887ef8`
- [x] [The Atlantic](https://www.theatlantic.com/) — UUID: `faf6f4c2-469e-574b-9438-6984f594ebac`
- [x] [Vox](https://www.vox.com/) — UUID: `1a708ddd-dc3c-527d-8ba2-ec763a621a30`
- [x] [The Intercept](https://theintercept.com/) — UUID: `f2c50155-110d-50cc-97f9-7b8d2c7d24ec`
- [x] [The Daily Beast](https://www.thedailybeast.com/) — UUID: `49602c02-54f5-59e1-a84c-3de08de22329`
- [x] [Democracy Now!](https://www.democracynow.org/) — UUID: `548323d3-d1b8-5c96-9784-6c0668430794`
- [x] [The Young Turks](https://www.tyt.com/) — UUID: `8c221600-9464-5aa3-9789-60e571e665fa`
- [x] [The Hill](https://thehill.com/) — UUID: `b6b915ff-0855-55c7-9f03-4cee6106a7d4`
- [x] [The Daily Caller](https://dailycaller.com/) — UUID: `454f9d17-21d9-55de-b5dd-dccacac606dd`
- [x] [The Blaze](https://www.theblaze.com/) — UUID: `06cdfdcb-97ca-5734-ba7c-10a362a0af92`
- [ ] [The Drudge Report](https://www.drudgereport.com/) — UUID: `c8a32954-70c2-58d9-81c5-9e7cfc4a23b7`
- [x] [The Huffington Post](https://www.huffpost.com/) — UUID: `02bf5bc8-8dba-51d8-88f4-b448ebc92b18`
- [x] [The Daily Wire](https://www.dailywire.com/) — UUID: `4c7bebbb-2b21-5148-8262-f8b5cf6b6564`
- [x] [The Daily Kos](https://www.dailykos.com/) — UUID: `3cbe3e91-2993-5e7f-9c86-bd713d87d854`
- [x] [Vice News](https://www.vice.com/) — UUID: `fe415725-ade9-5ed7-836c-549074ea9ca5`
- [x] [Politico](https://www.politico.com/) — UUID: `6a7551ce-1232-518f-b039-a2359dddc9a1`
- [x] [Axios](https://www.axios.com/) — UUID: `4d3755a3-a7a7-5ce5-b554-bcf7e4688d93`
- [x] [The New Republic](https://newrepublic.com/) — UUID: `459e9350-113f-5c08-90aa-421bd279bf00`
- [x] [The Nation](https://www.thenation.com/) — UUID: `651054b8-d328-5dac-b372-e23d0b6392f0`
- [x] [The American Prospect](https://prospect.org/) — UUID: `5f4815a4-85de-589d-93ee-cab2c1dbc6f9`
- [x] [The New Statesman](https://www.newstatesman.com/) — UUID: `c1ed83db-d323-5818-89e6-ec688657955c`
- [x] [The Spectator](https://spectator.com/) — UUID: `5c4da702-a184-5c21-97f7-8b7f2162459d`
- [x] [The New York Post](https://nypost.com/) — UUID: `502e6b76-c829-510f-892d-056df5b2f526`
- [x] [MSNBC](https://www.msnbc.com/) — UUID: `41658524-6896-50c3-be55-3a5e9eb6917e`
- [x] [Fox News](https://www.foxnews.com/) — UUID: `a1248c39-6950-5b7e-ad1c-65facd96ca99`
- [x] [Mother Jones](https://www.motherjones.com/) — UUID: `6a3cd60d-b3db-5bd0-b56e-c414abcd4135`
- [x] [Reason](https://reason.com/) — UUID: `13edee18-f805-55ce-b76b-e272044171a3`
- [x] [ProPublica](https://www.propublica.org/) — UUID: `d5f0e2b4-dcc2-5852-9108-53c712993905`
- [x] [Jacobin](https://jacobin.com/) — UUID: `1d4ff238-e1ac-5b59-bbd0-db382255d02d`
- [x] [National Review](https://www.nationalreview.com/) — UUID: `5e5ae665-eb12-5295-ab26-d606b4aa818f`
- [x] [Newsweek](https://www.newsweek.com/) — UUID: `bc66a488-3750-53c7-af39-23f19edb36c1`
- [x] [Time](https://time.com/) — UUID: `9eea0241-dd92-5281-91a4-05448ac6cd71`
- [x] [Foreign Policy](https://foreignpolicy.com/) — UUID: `00f4ad70-5bef-595a-a9d7-313619785346`
- [x] [The Christian Science Monitor](https://www.csmonitor.com/) — UUID: `c0d08550-1d3f-5d2b-91f2-61a469ef83a8`
- [x] [The Bulwark](https://www.thebulwark.com/) — UUID: `d0d27219-6d60-5b8c-9f33-0406acb71c73`
- [x] [Common Dreams](https://www.commondreams.org/) — UUID: `ff0f387d-915c-56d3-8549-4422cf685073`
- [x] [Drop Site News](https://www.dropsitenews.com/) — UUID: `c447ecef-7944-5c42-9081-66bfd850232d`
- [x] [Zeteo](https://zeteo.com/) — UUID: `a426669e-ea0c-5299-ad91-6a8dee3d86ae`
- [x] [The American Conservative](https://www.theamericanconservative.com/) — UUID: `8b2bf5a5-7fb9-56d4-a258-a7c33a033533`

### Arts, Entertainment & Gaming
- [x] [Variety](https://variety.com/) — UUID: `af4c6944-b24a-5e6b-9e98-2d79c97b410e`
- [x] [The Hollywood Reporter](https://www.hollywoodreporter.com/) — UUID: `0d3d756e-c539-5024-939b-d606f00f01ab`
- [x] [Rolling Stone](https://www.rollingstone.com/) — UUID: `b591f2a6-0d30-5f01-883f-b74d3c4988b3`
- [x] [Kotaku](https://kotaku.com/) — UUID: `a8f57c2b-ab92-5322-8b63-2860570dac63`
- [x] [Polygon](https://www.polygon.com/) — UUID: `2a93155e-02cd-5d62-90b4-2ea7b4d9635d`
- [x] [IGN](https://www.ign.com/) — UUID: `38501e0f-5452-58df-8253-53e00348cea9`
- [x] [Billboard](https://www.billboard.com/) — UUID: `6721c59d-4b72-59e7-baf3-313ee1a0f0f5`
- [x] [Pitchfork](https://pitchfork.com/) — UUID: `9085f1e3-cb20-515c-b2c6-c8a9d3b7ace0`
- [x] [Empire](https://www.empireonline.com/) — UUID: `235070ec-463e-54c4-8e16-c75292ff67d6`
- [x] [The Art Newspaper](https://www.theartnewspaper.com/) — UUID: `8fbfab72-f92d-5e12-85e4-67b532bd9efd`
- [x] [GameSpot](https://www.gamespot.com/) — UUID: `62ee1190-e742-502a-b961-3d3e6e053eae`
- [ ] [Vulture](https://www.vulture.com/) — UUID: `82c09a9d-5a7b-5776-914e-accb0771b8f9`
- [x] [Screen Rant](https://screenrant.com/) — UUID: `6f0bdd70-d0f5-50cf-bfbf-181783a69aa6`
- [x] [NME](https://www.nme.com/) — UUID: `e41ea9d1-a801-52a2-850b-8d00b28acbb9`

## License

This project is licensed under either of the following licenses, at your option:

- [Apache License, Version 2.0](LICENSE)
- [MIT License](LICENSE-MIT)
