# news-ag
News aggregator library for rust projects. 170+ news sources, with more being added regularly. Provides a unified API for fetching articles and content from a wide variety of news outlets.

## Usage
```rust
use news_ag::{
    ApNews,
    source::{Source, endpoint::EndpointScope},
};

#[tokio::main]
async fn main() {
    let endpoint = ApNews::get_endpoint(EndpointScope::World).expect("apnews should define a world endpoint");

    let articles = endpoint.get_articles().await;
    let content = articles[0].get_content().await.expect("article content should be available");
    println!("Articles from AP News: {articles:#?}");
    println!("First article: {content}");
}
```

Enable the `async` feature to use the awaitable API. Without it, `get_articles()` and `get_content()` remain blocking.

## Sources

`(*)` Article listings are available, but article content cannot currently be retrieved anonymously from this source.

### Global News Agencies & Major Networks
- [x] [AP News](https://apnews.com/)
- [ ] [The Associated Press](https://www.ap.org/)
- [x] [Reuters](https://www.reuters.com/) (*)
- [x] [BBC News](https://www.bbc.com/news)
- [x] [CNN](https://edition.cnn.com/)
- [x] [ABC News](https://abcnews.go.com/)
- [x] [CBS News](https://www.cbsnews.com/)
- [x] [NBC News](https://www.nbcnews.com/)
- [x] [USA Today](https://www.usatoday.com/)
- [x] [Sky News](https://news.sky.com/) (*)
- [x] [France 24](https://www.france24.com/en/) (*)
- [x] [Deutsche Welle](https://www.dw.com/en/)
- [x] [Al Jazeera](https://www.aljazeera.com/)
- [x] [NPR](https://www.npr.org/)
- [ ] [Agence France-Presse (AFP)](https://www.afp.com/)
- [x] [United Press International](https://www.upi.com/)
- [x] [Euronews](https://www.euronews.com/)
- [x] [Al-Monitor](https://www.al-monitor.com/)

### Business & Finance
- [x] [The Wall Street Journal](https://www.wsj.com/) (*)
- [x] [The Economist](https://www.economist.com/) (*)
- [x] [Bloomberg](https://www.bloomberg.com/) (*)
- [x] [Financial Times](https://www.ft.com/) (*)
- [x] [CNBC](https://www.cnbc.com/)
- [x] [Business Insider](https://www.businessinsider.com/)
- [x] [Fortune](https://fortune.com/)
- [x] [Forbes](https://www.forbes.com/)
- [x] [Quartz](https://qz.com/) (*)
- [x] [MarketWatch](https://www.marketwatch.com/) (*)
- [ ] [Investor's Business Daily](https://www.investors.com/)
- [x] [Fast Company](https://www.fastcompany.com/) (*)
- [x] [Kiplinger](https://www.kiplinger.com/)

### Technology & Science
- [x] [Wired](https://www.wired.com/)
- [x] [Ars Technica](https://arstechnica.com/)
- [x] [TechCrunch](https://techcrunch.com/)
- [x] [Engadget](https://www.engadget.com/)
- [x] [The Verge](https://www.theverge.com/)
- [x] [Gizmodo](https://gizmodo.com/)
- [x] [Science Magazine](https://www.science.org/) (*)
- [x] [Nature](https://www.nature.com/)
- [x] [Scientific American](https://www.scientificamerican.com/)
- [x] [New Scientist](https://www.newscientist.com/)
- [ ] [National Geographic](https://www.nationalgeographic.com/)
- [x] [CNET](https://www.cnet.com/)
- [x] [PCMag](https://www.pcmag.com/)
- [x] [VentureBeat](https://venturebeat.com/) (*)
- [x] [Mashable](https://mashable.com/)

### Regional News - United States
- [x] [New York Times](https://www.nytimes.com/) (*)
- [x] [The Washington Post](https://www.washingtonpost.com/) (*)
- [x] [The Chicago Tribune](https://www.chicagotribune.com/)
- [x] [The Los Angeles Times](https://www.latimes.com/)
- [ ] [The Miami Herald](https://www.miamiherald.com/)
- [x] [The Houston Chronicle](https://www.houstonchronicle.com/)
- [x] [The Philadelphia Inquirer](https://www.inquirer.com/)
- [x] [The Atlanta Journal-Constitution](https://www.ajc.com/)
- [x] [The Dallas Morning News](https://www.dallasnews.com/)
- [x] [The Denver Post](https://www.denverpost.com/)
- [x] [The Seattle Times](https://www.seattletimes.com/)
- [x] [The Boston Globe](https://www.bostonglobe.com/)
- [x] [The Detroit Free Press](https://www.freep.com/) (*)
- [x] [The Minneapolis Star Tribune](https://www.startribune.com/) (*)
- [x] [The Cleveland Plain Dealer](https://www.cleveland.com/)
- [x] [The Tampa Bay Times](https://www.tampabay.com/)
- [x] [The Orlando Sentinel](https://www.orlandosentinel.com/)
- [x] [The Pittsburgh Post-Gazette](https://www.post-gazette.com/)
- [x] [The Cincinnati Enquirer](https://www.cincinnati.com/) (*)
- [x] [The St. Louis Post-Dispatch](https://www.stltoday.com/)
- [x] [The Baltimore Sun](https://www.baltimoresun.com/)
- [ ] [The Kansas City Star](https://www.kansascity.com/)
- [x] [The Columbus Dispatch](https://www.dispatch.com/) (*)
- [x] [The Indianapolis Star](https://www.indystar.com/) (*)
- [x] [The Louisville Courier-Journal](https://www.courier-journal.com/) (*)
- [x] [The Nashville Tennessean](https://www.tennessean.com/) (*)
- [x] [The Oklahoma City Oklahoman](https://www.oklahoman.com/) (*)
- [ ] [The Raleigh News & Observer](https://www.newsobserver.com/)
- [x] [The Richmond Times-Dispatch](https://www.richmond.com/)
- [x] [The Hartford Courant](https://www.courant.com/)
- [x] [The Providence Journal](https://www.providencejournal.com/) (*)
- [x] [The Charleston Gazette-Mail](https://www.wvgazettemail.com/)
- [x] [The Charleston Post and Courier](https://www.postandcourier.com/)
- [x] [The Charleston Gazette](https://www.wvgazettemail.com/)
- [x] [WGN](https://wgntv.com/)
- [ ] [San Francisco Chronicle](https://www.sfchronicle.com/)
- [ ] [The Arizona Republic](https://www.azcentral.com/)
- [x] [The Oregonian](https://www.oregonlive.com/)
- [x] [The Star-Ledger](https://www.nj.com/starledger/)

### Regional News - Canada
- [x] [The Globe and Mail](https://www.theglobeandmail.com/)
- [x] [The Toronto Star](https://www.thestar.com/)
- [x] [The National Post](https://nationalpost.com/)
- [x] [The Vancouver Sun](https://vancouversun.com/)
- [x] [The Calgary Herald](https://calgaryherald.com/)
- [x] [The Edmonton Journal](https://edmontonjournal.com/)
- [x] [The Montreal Gazette](https://montrealgazette.com/)
- [x] [The Ottawa Citizen](https://ottawacitizen.com/)
- [x] [The Winnipeg Free Press](https://www.winnipegfreepress.com/)
- [x] [The Halifax Chronicle Herald](https://www.thechronicleherald.ca/)
- [x] [The Regina Leader-Post](https://leaderpost.com/)
- [x] [The Saskatoon StarPhoenix](https://thestarphoenix.com/)
- [x] [The St. John's Telegram](https://www.thetelegram.com/)
- [x] [Global News](https://globalnews.ca/)
- [ ] [CTV News](https://www.ctvnews.ca/)
- [x] [La Presse](https://www.lapresse.ca/)

### Regional News - United Kingdom & Ireland
- [x] [The Guardian](https://www.theguardian.com/international)
- [x] [The Independent](https://www.independent.co.uk/)
- [x] [The Times](https://www.thetimes.co.uk/)
- [x] [The Telegraph](https://www.telegraph.co.uk/) (*)
- [x] [The Sun](https://www.thesun.co.uk/) (*)
- [x] [The Mirror](https://www.mirror.co.uk/)
- [x] [The Daily Mail](https://www.dailymail.co.uk/)
- [x] [The Express](https://www.express.co.uk/)
- [x] [The Irish Times](https://www.irishtimes.com/)
- [x] [The Herald](https://www.heraldscotland.com/)
- [x] [The Belfast Telegraph](https://www.belfasttelegraph.co.uk/)
- [x] [The Scotsman](https://www.scotsman.com/)

### Regional News - Asia, Oceania & Middle East
- [x] [South China Morning Post](https://www.scmp.com/)
- [x] [The Japan Times](https://www.japantimes.co.jp/)
- [x] [The Times of India](https://timesofindia.indiatimes.com/)
- [x] [The Straits Times](https://www.straitstimes.com/)
- [x] [Nikkei Asia](https://asia.nikkei.com/)
- [x] [The Daily Telegraph](https://www.dailytelegraph.com.au/) (*)
- [x] [The Sydney Morning Herald](https://www.smh.com.au/)
- [x] [The Age](https://www.theage.com.au/)
- [x] [The Australian](https://www.theaustralian.com.au/) (*)
- [ ] [The New Zealand Herald](https://www.nzherald.co.nz/)
- [x] [Haaretz](https://www.haaretz.com/)
- [x] [The Jerusalem Post](https://www.jpost.com/)
- [x] [Middle East Eye](https://www.middleeasteye.net/)
- [ ] [The Korea Herald](https://www.koreaherald.com/)
- [x] [The Bangkok Post](https://www.bangkokpost.com/)
- [x] [The Hindu](https://www.thehindu.com/)

### Regional News - Europe & Latin America
- [x] [El País](https://elpais.com/elpais/inenglish.html)
- [x] [Le Monde](https://www.lemonde.fr/en/)
- [x] [Der Spiegel](https://www.spiegel.de/international/)
- [x] [The Kyiv Independent](https://kyivindependent.com/)
- [x] [The Moscow Times](https://www.themoscowtimes.com/)
- [x] [Folha de S.Paulo](https://www.folha.uol.com.br/)
- [x] [Clarín](https://www.clarin.com/)
- [x] [Corriere della Sera](https://www.corriere.it/english/)
- [x] [Frankfurter Allgemeine Zeitung](https://www.faz.net/english/)
- [x] [La Repubblica](https://www.repubblica.it/)

### Politics, Opinion & Commentary
- [x] [The New Yorker](https://www.newyorker.com/)
- [x] [The Atlantic](https://www.theatlantic.com/)
- [x] [Vox](https://www.vox.com/)
- [x] [The Intercept](https://theintercept.com/)
- [x] [The Daily Beast](https://www.thedailybeast.com/)
- [x] [Democracy Now!](https://www.democracynow.org/)
- [x] [The Young Turks](https://www.tyt.com/)
- [x] [The Hill](https://thehill.com/)
- [x] [The Daily Caller](https://dailycaller.com/)
- [x] [The Blaze](https://www.theblaze.com/)
- [ ] [The Drudge Report](https://www.drudgereport.com/)
- [x] [The Huffington Post](https://www.huffpost.com/)
- [x] [The Daily Wire](https://www.dailywire.com/)
- [x] [The Daily Kos](https://www.dailykos.com/)
- [x] [Vice News](https://www.vice.com/)
- [x] [Politico](https://www.politico.com/)
- [x] [Axios](https://www.axios.com/)
- [x] [The New Republic](https://newrepublic.com/)
- [x] [The Nation](https://www.thenation.com/)
- [x] [The American Prospect](https://prospect.org/)
- [x] [The New Statesman](https://www.newstatesman.com/)
- [x] [The Spectator](https://spectator.com/)
- [x] [The New York Post](https://nypost.com/)
- [x] [MSNBC](https://www.msnbc.com/)
- [x] [Fox News](https://www.foxnews.com/)
- [x] [Mother Jones](https://www.motherjones.com/)
- [x] [Reason](https://reason.com/)
- [x] [ProPublica](https://www.propublica.org/)
- [x] [Jacobin](https://jacobin.com/)
- [x] [National Review](https://www.nationalreview.com/)
- [x] [Newsweek](https://www.newsweek.com/)
- [x] [Time](https://time.com/)
- [x] [Foreign Policy](https://foreignpolicy.com/)
- [x] [The Christian Science Monitor](https://www.csmonitor.com/)
- [x] [The Bulwark](https://www.thebulwark.com/)
- [x] [Common Dreams](https://www.commondreams.org/)
- [x] [Drop Site News](https://www.dropsitenews.com/)
- [x] [Zeteo](https://zeteo.com/)
- [x] [The American Conservative](https://www.theamericanconservative.com/)

### Arts, Entertainment & Gaming
- [x] [Variety](https://variety.com/)
- [x] [The Hollywood Reporter](https://www.hollywoodreporter.com/)
- [x] [Rolling Stone](https://www.rollingstone.com/)
- [x] [Kotaku](https://kotaku.com/)
- [x] [Polygon](https://www.polygon.com/)
- [x] [IGN](https://www.ign.com/)
- [x] [Billboard](https://www.billboard.com/)
- [x] [Pitchfork](https://pitchfork.com/)
- [x] [Empire](https://www.empireonline.com/)
- [x] [The Art Newspaper](https://www.theartnewspaper.com/)
- [x] [GameSpot](https://www.gamespot.com/) (*)
- [ ] [Vulture](https://www.vulture.com/)
- [x] [Screen Rant](https://screenrant.com/)
- [x] [NME](https://www.nme.com/)

## License
This project is licensed under either of the following licenses, at your option:

- [Apache License, Version 2.0](LICENSE)
- [MIT License](LICENSE-MIT)
