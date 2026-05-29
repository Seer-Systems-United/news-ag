# news-ag
News aggregator library for rust projects.

## Usage
```rust
use news_ag::{
    ApNews,
    source::{Source, endpoint::EndpointScope},
};

fn main() {    
    let endpoint = ApNews::get_endpoint(EndpointScope::World).expect("apnews should define a world endpoint");

    let articles = endpoint.get_articles();
    println!("The URL for AP News is: {}", articles);
}
```

## Sources:
- [x] [AP News](https://apnews.com/)
- [x] [Reuters](https://www.reuters.com/)
- [x] [New York Times](https://www.nytimes.com/)
- [x] [The Guardian](https://www.theguardian.com/international)
- [x] [BBC News](https://www.bbc.com/news)
- [x] [CNN](https://edition.cnn.com/)
- [x] [Al Jazeera](https://www.aljazeera.com/)
- [x] [The Washington Post](https://www.washingtonpost.com/)
- [x] [The Wall Street Journal](https://www.wsj.com/)
- [x] [The Economist](https://www.economist.com/)
- [x] [NPR](https://www.npr.org/)
- [x] [Bloomberg](https://www.bloomberg.com/)
- [ ] [The Associated Press](https://www.ap.org/)
- [x] [The New Yorker](https://www.newyorker.com/)
- [x] [The Atlantic](https://www.theatlantic.com/)
- [x] [The Verge](https://www.theverge.com/)
- [x] [TechCrunch](https://techcrunch.com/)
- [x] [Engadget](https://www.engadget.com/)
- [x] [Vox](https://www.vox.com/)
- [x] [The Intercept](https://theintercept.com/)
- [x] [The Daily Beast](https://www.thedailybeast.com/)
- [x] [The Independent](https://www.independent.co.uk/)
- [ ] [The Times](https://www.thetimes.co.uk/)
- [x] [The Telegraph](https://www.telegraph.co.uk/)
- [x] [The Sun](https://www.thesun.co.uk/)
- [x] [The Mirror](https://www.mirror.co.uk/)
- [x] [The Daily Mail](https://www.dailymail.co.uk/)
- [x] [The Express](https://www.express.co.uk/)
- [x] [The Daily Telegraph](https://www.dailytelegraph.com.au/)
- [x] [The Sydney Morning Herald](https://www.smh.com.au/)
- [x] [The Age](https://www.theage.com.au/)
- [x] [The Australian](https://www.theaustralian.com.au/)
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
- [x] [WGN](https://wgntv.com/)
- [x] [The Chicago Tribune](https://www.chicagotribune.com/)
- [x] [The Los Angeles Times](https://www.latimes.com/)
- [ ] [The Miami Herald](https://www.miamiherald.com/)
- [x] [The Houston Chronicle](https://www.houstonchronicle.com/)
- [x] [The Philadelphia Inquirer](https://www.inquirer.com/)
- [ ] [The Atlanta Journal-Constitution](https://www.ajc.com/)
- [ ] [The Dallas Morning News](https://www.dallasnews.com/)
- [x] [The Denver Post](https://www.denverpost.com/)
- [x] [The Seattle Times](https://www.seattletimes.com/)
- [x] [The Boston Globe](https://www.bostonglobe.com/)
- [ ] [The Detroit Free Press](https://www.freep.com/)
- [x] [The Minneapolis Star Tribune](https://www.startribune.com/)
- [x] [The Cleveland Plain Dealer](https://www.cleveland.com/)
- [x] [The Tampa Bay Times](https://www.tampabay.com/)
- [x] [The Orlando Sentinel](https://www.orlandosentinel.com/)
- [x] [The Pittsburgh Post-Gazette](https://www.post-gazette.com/)
- [ ] [The Cincinnati Enquirer](https://www.cincinnati.com/)
- [x] [The St. Louis Post-Dispatch](https://www.stltoday.com/)
- [x] [The Baltimore Sun](https://www.baltimoresun.com/)
- [ ] [The Kansas City Star](https://www.kansascity.com/)
- [ ] [The Columbus Dispatch](https://www.dispatch.com/)
- [ ] [The Indianapolis Star](https://www.indystar.com/)
- [ ] [The Louisville Courier-Journal](https://www.courier-journal.com/)
- [ ] [The Nashville Tennessean](https://www.tennessean.com/)
- [ ] [The Oklahoma City Oklahoman](https://www.oklahoman.com/)
- [ ] [The Raleigh News & Observer](https://www.newsobserver.com/)
- [x] [The Richmond Times-Dispatch](https://www.richmond.com/)
- [x] [The Hartford Courant](https://www.courant.com/)
- [ ] [The Providence Journal](https://www.providencejournal.com/)
- [x] [The Charleston Gazette-Mail](https://www.wvgazettemail.com/)
- [x] [The Charleston Post and Courier](https://www.postandcourier.com/)
- [ ] [The Charleston Daily Mail](https://www.charlestondailymail.com/)
- [x] [The Charleston Gazette](https://www.wvgazettemail.com/)
- [x] [Democracy Now!](https://www.democracynow.org/)
- [x] [The Intercept](https://theintercept.com/)
- [ ] [The Young Turks](https://www.tyt.com/)
- [x] [The Hill](https://thehill.com/)
- [x] [The Daily Caller](https://dailycaller.com/)
- [x] [The Blaze](https://www.theblaze.com/)

## License
This project is licensed under the Apache 2.0 License - see the [LICENSE](LICENSE) file for details
