use crate::{
    models::Article,
    parse::{Format, approach::ParseApproach, rule::Rule, section::ParseSection},
    source::endpoint::{Endpoint, EndpointScope},
};

const USER_AGENT: &str = "Mozilla/5.0 (compatible; news-sources/0.1)";

pub fn source_id(number: u32) -> uuid::Uuid {
    match number {
        1 => uuid::uuid!("ce8ea65b-85a6-5fee-8789-98fd53887c5a"),
        3 => uuid::uuid!("b7487efe-b9d5-51da-951d-92f2af5ca190"),
        4 => uuid::uuid!("e88c96aa-45b3-5523-8d35-0fb4083c68b1"),
        5 => uuid::uuid!("ae3f8985-03ff-538e-8442-c8f8e822fdb1"),
        6 => uuid::uuid!("fb5030ee-e573-5b33-bb99-3b0f10dfcd46"),
        7 => uuid::uuid!("d776156e-a136-5e83-af40-dd08102a0ce7"),
        8 => uuid::uuid!("026a4ff4-eb40-5212-8824-cbe3ef578277"),
        9 => uuid::uuid!("f9dd7397-bdbd-58a6-8956-d469fcbb281e"),
        10 => uuid::uuid!("8409b28c-693b-5061-9943-31072d2a37c4"),
        11 => uuid::uuid!("fdb1f043-ee0a-513f-bdc4-874c5cd996ba"),
        12 => uuid::uuid!("b3a8ffa6-df24-556b-8976-cb658a04426d"),
        13 => uuid::uuid!("96cf30fe-a1fc-5645-89d3-8d15c33f7c7d"),
        14 => uuid::uuid!("5ece0ca7-52d1-5ba8-813b-3152b16d17ac"),
        16 => uuid::uuid!("6a7d3b7d-6568-5a07-948d-cc556bd683a3"),
        17 => uuid::uuid!("6cf3a2aa-8c16-5a2e-8a03-773025fbcdb6"),
        18 => uuid::uuid!("e6f7c900-2fee-51aa-9746-a7e07155397e"),
        100 => uuid::uuid!("b5c969f2-c7d9-5ed6-bbc7-cfbd4920d7fb"),
        101 => uuid::uuid!("31443912-1b21-58f9-b23c-aa0d31c00206"),
        102 => uuid::uuid!("a633de5f-5239-55f9-b0db-bec202b631c4"),
        103 => uuid::uuid!("314261c1-d2d5-50bc-bcff-1fcdece1d289"),
        104 => uuid::uuid!("8f1afa95-f295-58dd-81a6-2f0c32eddac0"),
        105 => uuid::uuid!("2c4b6fd2-9dff-5398-a603-eeecbe0c2a02"),
        106 => uuid::uuid!("471de1dd-0645-5034-b09d-c75ecd6dcc3f"),
        107 => uuid::uuid!("1544049c-6e9b-5d1a-941e-2f244b2c17fe"),
        108 => uuid::uuid!("091aae1b-7573-5909-ae17-65877f90986e"),
        109 => uuid::uuid!("3b9334d0-7cec-5957-93d3-03558570afad"),
        111 => uuid::uuid!("7b211dbd-bb97-5afa-81e4-2ac19776dd56"),
        112 => uuid::uuid!("06e5dc9c-f2b7-5af0-9052-ecd2ab02c790"),
        200 => uuid::uuid!("a4888510-6377-57d4-9266-b187c9d0cf2c"),
        201 => uuid::uuid!("dd7f5adc-8d60-5f82-b8c7-4187d38e8988"),
        202 => uuid::uuid!("f3aead82-0d45-516e-98f5-c8e37822cba6"),
        203 => uuid::uuid!("0b3f5d41-da07-5480-afff-ae4d83bbac8f"),
        204 => uuid::uuid!("d7ddb8c2-c94b-5e5c-a4f2-f0f3b7195159"),
        205 => uuid::uuid!("5ca3113a-9977-540c-8602-dc20ae8bfdb9"),
        206 => uuid::uuid!("27622844-8147-55e8-8c69-eb80f902324b"),
        207 => uuid::uuid!("68b74f02-10da-5545-9424-15673e7e9197"),
        208 => uuid::uuid!("e3b278ef-0be8-5f5c-8c81-3c62c973c331"),
        209 => uuid::uuid!("b8b42123-120d-59e2-9973-81af3477a413"),
        211 => uuid::uuid!("9980e7c7-5810-5cc2-82c5-a0cd9e4bffeb"),
        212 => uuid::uuid!("69ea7ec5-2328-57f7-8358-b6dbc91a2113"),
        213 => uuid::uuid!("da8e3f48-eb7f-5bf4-bb12-7f265338040a"),
        214 => uuid::uuid!("079e001b-bcc6-590c-a1a1-b3d5e278edc0"),
        300 => uuid::uuid!("1a56ce8d-67a2-58fa-89be-eae50be6982a"),
        301 => uuid::uuid!("667cdada-06b0-51a4-b33b-053412baa71a"),
        302 => uuid::uuid!("d0c23604-79ad-5209-a6fd-6482232363fd"),
        303 => uuid::uuid!("adaf2dec-d18a-5fab-889f-8eef35722ae1"),
        305 => uuid::uuid!("44b12633-1600-53e9-8958-ce4d0b3a0dc7"),
        306 => uuid::uuid!("438e8740-72c7-5e65-9008-1739274f8e9d"),
        307 => uuid::uuid!("860bb1af-1a3b-5060-9b7e-aaf459539a0d"),
        308 => uuid::uuid!("8745886f-a780-523c-810d-5e449cb6b59a"),
        309 => uuid::uuid!("c7c87f74-442e-55d3-8a83-8780c4423416"),
        310 => uuid::uuid!("4f97d87a-a4e5-50aa-a3ec-4a8788136f7b"),
        311 => uuid::uuid!("98759f03-9cc9-53a9-8c34-b1ebdb5b9136"),
        312 => uuid::uuid!("02920e57-d4a3-56bc-8d21-d2cd9e2441f2"),
        313 => uuid::uuid!("a5b15c9c-3eb8-507e-8704-f9c993e2d25b"),
        314 => uuid::uuid!("0e2b9d69-991c-5606-aedd-227a5d68d820"),
        315 => uuid::uuid!("cd3d3661-941e-5093-a47f-8d49f277efd7"),
        316 => uuid::uuid!("4f05d2e4-71dc-5e9b-8ffa-96f2efa335ee"),
        317 => uuid::uuid!("64348b3e-bb92-5e21-90fc-e08978c07f16"),
        318 => uuid::uuid!("382b9717-720d-5935-96f3-ffce8bcb2702"),
        319 => uuid::uuid!("78d4a04d-36ef-5f97-948b-3f52829c4b1c"),
        320 => uuid::uuid!("f23aca1b-a922-5429-972d-c5abf3f7ce49"),
        322 => uuid::uuid!("9a058af1-bf98-56dd-8ae8-db19378c397a"),
        323 => uuid::uuid!("0e77a4c7-5915-59c3-89bd-51c06a250d32"),
        324 => uuid::uuid!("6b671f91-dd84-51f8-b908-ea22f3d34603"),
        325 => uuid::uuid!("bcbc00f2-50cd-524b-93ed-0a1fe02cc879"),
        326 => uuid::uuid!("090ca54b-04c4-55e5-bf8f-cad453148368"),
        328 => uuid::uuid!("9da8d2d1-1c79-576f-b4bc-46658e7144d7"),
        329 => uuid::uuid!("10285a70-8e5c-5166-8feb-8ae457deafff"),
        330 => uuid::uuid!("628a3ebf-7bad-56a3-98d2-58d8cad5b2fe"),
        331 => uuid::uuid!("d0134862-7f3d-5f9f-a0de-c432be2917fd"),
        332 => uuid::uuid!("901062d1-dcce-5798-94bf-750a59805045"),
        333 => uuid::uuid!("9393b164-92cd-58be-8759-c22144a27410"),
        334 => uuid::uuid!("64215c66-6a5a-5659-b16b-cc546a39c531"),
        337 => uuid::uuid!("e8354b61-7f45-5f50-9812-cfb52ca2e1e1"),
        338 => uuid::uuid!("29b24375-d3ef-5175-9966-c3aeb706b6d0"),
        400 => uuid::uuid!("c81b1b79-80af-55cf-b306-61ffc6b3c8f4"),
        401 => uuid::uuid!("9b51c413-a2b1-5e60-8b6a-0036acc5808c"),
        402 => uuid::uuid!("568b354a-4733-5231-818f-3f6746ee9cbc"),
        403 => uuid::uuid!("0d00d7b2-6ff5-5ed5-a5c8-9cd054203f05"),
        404 => uuid::uuid!("3e34f9d3-e56e-5861-b6e7-a38bd124cf7a"),
        405 => uuid::uuid!("abf340ee-21fa-57e6-8ac3-c9146031de0d"),
        406 => uuid::uuid!("026cd877-3b27-525a-81b6-718a4fb27058"),
        407 => uuid::uuid!("ab73c916-f6d6-52b2-8e04-ec3e65fe1604"),
        408 => uuid::uuid!("e8dca6af-e87b-5578-865e-c196c6fe96d5"),
        409 => uuid::uuid!("c3a52872-b6b0-5a89-9b09-79f45be66f0e"),
        410 => uuid::uuid!("d639e6d6-a20a-5c23-b695-3a3b3814ba9a"),
        411 => uuid::uuid!("3193a34e-cd6c-5171-9ca3-1510474659cf"),
        412 => uuid::uuid!("dbf2be1c-15a1-5f3d-9db7-1368df16637b"),
        413 => uuid::uuid!("b126d47e-a5a8-5608-9a8f-ffbf3a4cc8f1"),
        415 => uuid::uuid!("3c16cac4-68f5-5ec0-a1f6-b28d6c4ebd1b"),
        500 => uuid::uuid!("d8694288-00fa-5a25-835a-37ccd14dfcd0"),
        501 => uuid::uuid!("270d3930-a471-582c-9e49-469b7ba637e9"),
        502 => uuid::uuid!("e1d16561-ebd2-5a2c-a967-d90511870cea"),
        503 => uuid::uuid!("2409fba9-a371-5684-8c28-b4b8174cfac4"),
        504 => uuid::uuid!("1fbeb8c9-c18d-5a9c-9d74-5b73680a956b"),
        505 => uuid::uuid!("673bf53a-1aed-58d5-b33f-f81d77d8f2d8"),
        506 => uuid::uuid!("05467408-e747-59fa-bc8e-ea6301078ba4"),
        507 => uuid::uuid!("7bfe0775-17d1-5c0c-b058-b74651051ecc"),
        508 => uuid::uuid!("977b8325-b25e-50f3-93e3-c9d151fe5134"),
        509 => uuid::uuid!("43f72e82-9a8b-5784-9879-2bfe14ba9d7b"),
        510 => uuid::uuid!("df6e3463-325d-5c52-9455-6be0c9a04d82"),
        511 => uuid::uuid!("be9b1712-0063-523b-9155-dff7e55d3a4b"),
        600 => uuid::uuid!("bd4f3154-91b7-5934-abdd-f1729c0a6f35"),
        601 => uuid::uuid!("22d4af94-d056-502f-b84a-60222bc937d8"),
        602 => uuid::uuid!("17830b27-da99-56c0-82bb-fa0b3a8c6e82"),
        603 => uuid::uuid!("14bbc696-f642-5594-83a6-a59e566b6ddf"),
        604 => uuid::uuid!("f7731152-33e1-558f-85e0-46f5caf0f2c1"),
        605 => uuid::uuid!("acb652ba-3430-5479-8cc7-ba618e2c5a0e"),
        606 => uuid::uuid!("b9405589-4d4c-52d1-a75a-618650f04e5d"),
        607 => uuid::uuid!("0b19ded4-0ced-55ef-b48b-6fc31cfb538f"),
        608 => uuid::uuid!("027d28e0-d9ea-5e5e-aa05-4016b9cf2353"),
        610 => uuid::uuid!("5ec39399-4977-5710-8df8-98511b50b1ae"),
        611 => uuid::uuid!("a7cf0b4a-5bc0-5eff-a605-8961b77472fd"),
        612 => uuid::uuid!("1b0b5468-a53a-58f5-a8dc-7ff310597285"),
        614 => uuid::uuid!("26f262c2-db86-5eba-b785-e95dc23d150c"),
        615 => uuid::uuid!("1063a172-d91a-546a-8e45-ffcf61be9add"),
        700 => uuid::uuid!("a8211257-cb6a-50c7-8422-1077c64006ea"),
        701 => uuid::uuid!("403add74-fd09-59bd-9459-47d4725fdb9b"),
        702 => uuid::uuid!("f0ffb230-b07e-5568-b381-7ae03b8bedbe"),
        703 => uuid::uuid!("e64a0cde-e848-5c56-902b-fc772d0ced18"),
        704 => uuid::uuid!("3eebd8d8-f842-5110-b9ac-9e4df69c2f51"),
        705 => uuid::uuid!("493c7e12-dcff-5dce-bf87-e5537c90095c"),
        706 => uuid::uuid!("512abf50-d22d-59b3-bd0f-c271d00dd0b5"),
        707 => uuid::uuid!("1eb6d2d4-08b1-5460-ba1e-4231624b354f"),
        708 => uuid::uuid!("cbf5e8e7-c63e-53ee-b94d-b8b88ee4138d"),
        709 => uuid::uuid!("dd73f6ac-ca9d-5840-8445-6d8025860fa6"),
        800 => uuid::uuid!("0e5a2977-d96d-5d77-b645-bab609887ef8"),
        801 => uuid::uuid!("faf6f4c2-469e-574b-9438-6984f594ebac"),
        802 => uuid::uuid!("1a708ddd-dc3c-527d-8ba2-ec763a621a30"),
        803 => uuid::uuid!("f2c50155-110d-50cc-97f9-7b8d2c7d24ec"),
        804 => uuid::uuid!("49602c02-54f5-59e1-a84c-3de08de22329"),
        805 => uuid::uuid!("548323d3-d1b8-5c96-9784-6c0668430794"),
        806 => uuid::uuid!("8c221600-9464-5aa3-9789-60e571e665fa"),
        807 => uuid::uuid!("b6b915ff-0855-55c7-9f03-4cee6106a7d4"),
        808 => uuid::uuid!("454f9d17-21d9-55de-b5dd-dccacac606dd"),
        809 => uuid::uuid!("06cdfdcb-97ca-5734-ba7c-10a362a0af92"),
        811 => uuid::uuid!("02bf5bc8-8dba-51d8-88f4-b448ebc92b18"),
        812 => uuid::uuid!("4c7bebbb-2b21-5148-8262-f8b5cf6b6564"),
        813 => uuid::uuid!("3cbe3e91-2993-5e7f-9c86-bd713d87d854"),
        814 => uuid::uuid!("fe415725-ade9-5ed7-836c-549074ea9ca5"),
        815 => uuid::uuid!("6a7551ce-1232-518f-b039-a2359dddc9a1"),
        816 => uuid::uuid!("4d3755a3-a7a7-5ce5-b554-bcf7e4688d93"),
        817 => uuid::uuid!("459e9350-113f-5c08-90aa-421bd279bf00"),
        818 => uuid::uuid!("651054b8-d328-5dac-b372-e23d0b6392f0"),
        819 => uuid::uuid!("5f4815a4-85de-589d-93ee-cab2c1dbc6f9"),
        820 => uuid::uuid!("c1ed83db-d323-5818-89e6-ec688657955c"),
        821 => uuid::uuid!("5c4da702-a184-5c21-97f7-8b7f2162459d"),
        822 => uuid::uuid!("502e6b76-c829-510f-892d-056df5b2f526"),
        823 => uuid::uuid!("41658524-6896-50c3-be55-3a5e9eb6917e"),
        824 => uuid::uuid!("a1248c39-6950-5b7e-ad1c-65facd96ca99"),
        825 => uuid::uuid!("6a3cd60d-b3db-5bd0-b56e-c414abcd4135"),
        826 => uuid::uuid!("13edee18-f805-55ce-b76b-e272044171a3"),
        827 => uuid::uuid!("d5f0e2b4-dcc2-5852-9108-53c712993905"),
        828 => uuid::uuid!("1d4ff238-e1ac-5b59-bbd0-db382255d02d"),
        829 => uuid::uuid!("5e5ae665-eb12-5295-ab26-d606b4aa818f"),
        830 => uuid::uuid!("bc66a488-3750-53c7-af39-23f19edb36c1"),
        831 => uuid::uuid!("9eea0241-dd92-5281-91a4-05448ac6cd71"),
        832 => uuid::uuid!("00f4ad70-5bef-595a-a9d7-313619785346"),
        833 => uuid::uuid!("c0d08550-1d3f-5d2b-91f2-61a469ef83a8"),
        834 => uuid::uuid!("d0d27219-6d60-5b8c-9f33-0406acb71c73"),
        835 => uuid::uuid!("ff0f387d-915c-56d3-8549-4422cf685073"),
        836 => uuid::uuid!("c447ecef-7944-5c42-9081-66bfd850232d"),
        837 => uuid::uuid!("a426669e-ea0c-5299-ad91-6a8dee3d86ae"),
        838 => uuid::uuid!("8b2bf5a5-7fb9-56d4-a258-a7c33a033533"),
        900 => uuid::uuid!("af4c6944-b24a-5e6b-9e98-2d79c97b410e"),
        901 => uuid::uuid!("0d3d756e-c539-5024-939b-d606f00f01ab"),
        902 => uuid::uuid!("b591f2a6-0d30-5f01-883f-b74d3c4988b3"),
        903 => uuid::uuid!("a8f57c2b-ab92-5322-8b63-2860570dac63"),
        904 => uuid::uuid!("2a93155e-02cd-5d62-90b4-2ea7b4d9635d"),
        905 => uuid::uuid!("38501e0f-5452-58df-8253-53e00348cea9"),
        906 => uuid::uuid!("6721c59d-4b72-59e7-baf3-313ee1a0f0f5"),
        907 => uuid::uuid!("9085f1e3-cb20-515c-b2c6-c8a9d3b7ace0"),
        908 => uuid::uuid!("235070ec-463e-54c4-8e16-c75292ff67d6"),
        909 => uuid::uuid!("8fbfab72-f92d-5e12-85e4-67b532bd9efd"),
        910 => uuid::uuid!("62ee1190-e742-502a-b961-3d3e6e053eae"),
        912 => uuid::uuid!("6f0bdd70-d0f5-50cf-bfbf-181783a69aa6"),
        913 => uuid::uuid!("e41ea9d1-a801-52a2-850b-8d00b28acbb9"),
        914 => uuid::uuid!("ea33363b-1b52-401f-afa3-cf87a707941c"),
        915 => uuid::uuid!("2e06456f-d203-41df-ace0-7630c9cde268"),
        916 => uuid::uuid!("8e2d730c-a46b-49e3-8fab-a453367237b1"),
        917 => uuid::uuid!("469e831d-6c97-4c67-9600-c9868ce1df3a"),
        918 => uuid::uuid!("f1831b5c-0716-4b33-baba-d9f049f81514"),
        919 => uuid::uuid!("e2bf817b-cff8-402c-b1f0-bdc40f9d6ac7"),
        920 => uuid::uuid!("de732b16-bc1d-4a27-81d9-7bc471183865"),
        921 => uuid::uuid!("8372597c-bc36-428b-93ab-20811a777731"),
        922 => uuid::uuid!("22c3ca7e-5b5c-484b-94ea-bff9ee38930c"),
        923 => uuid::uuid!("72de7a99-ea72-426f-866b-63f189ec2c3f"),
        925 => uuid::uuid!("ec486cac-9d94-43d7-a63b-d46fc01e3338"),
        928 => uuid::uuid!("1bdfde0c-b6ca-4fa8-b12b-a79ff0c80123"),
        929 => uuid::uuid!("f3d24792-95d4-4f19-b84c-4fa7535ad115"),
        930 => uuid::uuid!("21c68657-1cb8-406a-911c-ae15dde7f35c"),
        932 => uuid::uuid!("8096eb3a-f530-481a-94b1-0860b52bd264"),
        936 => uuid::uuid!("27cf6f72-8318-4ede-a94a-3b715d43c59b"),
        939 => uuid::uuid!("03f500bf-f40d-44ad-acd5-98c12ac9d387"),
        940 => uuid::uuid!("8487df81-237c-4c5b-9b5b-99f7fd13c926"),
        942 => uuid::uuid!("9cab99a0-f6ec-4d29-b098-2685a7f130f4"),
        943 => uuid::uuid!("0f4e2408-3140-466c-b876-a123783fccd0"),
        944 => uuid::uuid!("50bf6173-8127-4424-9f3c-1a40c0a0f7c7"),
        945 => uuid::uuid!("95abe2a4-df38-4418-9813-5549bec29c55"),
        946 => uuid::uuid!("e3b11920-ef28-4c49-a7d6-da7e145c1674"),
        947 => uuid::uuid!("a91e7821-900c-4348-86ff-daf8b606249b"),
        952 => uuid::uuid!("d29371ea-10b8-4f5c-a406-96dbcd0ce721"),
        954 => uuid::uuid!("f8fc43e0-6292-492a-ad3f-54d5b0a2317f"),
        955 => uuid::uuid!("1e20a6ae-8c72-426d-a9f2-8cc92eb42778"),
        956 => uuid::uuid!("fb64a6dc-a5c8-49cf-9368-79940d253fb9"),
        957 => uuid::uuid!("ed1aeefb-a2c2-469b-8ae2-cc1a63ee549b"),
        _ => panic!("unknown source number: {}", number),
    }
}

pub fn source_name(number: u32) -> &'static str {
    match number {
        1 => "AP News",
        3 => "Reuters",
        4 => "BBC News",
        5 => "CNN",
        6 => "ABC News",
        7 => "CBS News",
        8 => "NBC News",
        9 => "USA Today",
        10 => "Sky News",
        11 => "France 24",
        12 => "Deutsche Welle",
        13 => "Al Jazeera",
        14 => "NPR",
        16 => "United Press International",
        17 => "Euronews",
        18 => "Al-Monitor",
        100 => "The Wall Street Journal",
        101 => "The Economist",
        102 => "Bloomberg",
        103 => "Financial Times",
        104 => "CNBC",
        105 => "Business Insider",
        106 => "Fortune",
        107 => "Forbes",
        108 => "Quartz",
        109 => "MarketWatch",
        111 => "Fast Company",
        112 => "Kiplinger",
        200 => "Wired",
        201 => "Ars Technica",
        202 => "TechCrunch",
        203 => "Engadget",
        204 => "The Verge",
        205 => "Gizmodo",
        206 => "Science Magazine",
        207 => "Nature",
        208 => "Scientific American",
        209 => "New Scientist",
        211 => "CNET",
        212 => "PCMag",
        213 => "VentureBeat",
        214 => "Mashable",
        300 => "New York Times",
        301 => "The Washington Post",
        302 => "The Chicago Tribune",
        303 => "The Los Angeles Times",
        305 => "The Houston Chronicle",
        306 => "The Philadelphia Inquirer",
        307 => "The Atlanta Journal-Constitution",
        308 => "The Dallas Morning News",
        309 => "The Denver Post",
        310 => "The Seattle Times",
        311 => "The Boston Globe",
        312 => "The Detroit Free Press",
        313 => "The Minneapolis Star Tribune",
        314 => "The Cleveland Plain Dealer",
        315 => "The Tampa Bay Times",
        316 => "The Orlando Sentinel",
        317 => "The Pittsburgh Post-Gazette",
        318 => "The Cincinnati Enquirer",
        319 => "The St. Louis Post-Dispatch",
        320 => "The Baltimore Sun",
        322 => "The Columbus Dispatch",
        323 => "The Indianapolis Star",
        324 => "The Louisville Courier-Journal",
        325 => "The Nashville Tennessean",
        326 => "The Oklahoma City Oklahoman",
        328 => "The Richmond Times-Dispatch",
        329 => "The Hartford Courant",
        330 => "The Providence Journal",
        331 => "The Charleston Gazette-Mail",
        332 => "The Charleston Post and Courier",
        333 => "The Charleston Gazette",
        334 => "WGN",
        337 => "The Oregonian",
        338 => "The Star-Ledger",
        400 => "The Globe and Mail",
        401 => "The Toronto Star",
        402 => "The National Post",
        403 => "The Vancouver Sun",
        404 => "The Calgary Herald",
        405 => "The Edmonton Journal",
        406 => "The Montreal Gazette",
        407 => "The Ottawa Citizen",
        408 => "The Winnipeg Free Press",
        409 => "The Halifax Chronicle Herald",
        410 => "The Regina Leader-Post",
        411 => "The Saskatoon StarPhoenix",
        412 => "The St. John's Telegram",
        413 => "Global News",
        415 => "La Presse",
        500 => "The Guardian",
        501 => "The Independent",
        502 => "The Times",
        503 => "The Telegraph",
        504 => "The Sun",
        505 => "The Mirror",
        506 => "The Daily Mail",
        507 => "The Express",
        508 => "The Irish Times",
        509 => "The Herald",
        510 => "The Belfast Telegraph",
        511 => "The Scotsman",
        600 => "South China Morning Post",
        601 => "The Japan Times",
        602 => "The Times of India",
        603 => "The Straits Times",
        604 => "Nikkei Asia",
        605 => "The Daily Telegraph",
        606 => "The Sydney Morning Herald",
        607 => "The Age",
        608 => "The Australian",
        610 => "Haaretz",
        611 => "The Jerusalem Post",
        612 => "Middle East Eye",
        614 => "The Bangkok Post",
        615 => "The Hindu",
        700 => "El País",
        701 => "Le Monde",
        702 => "Der Spiegel",
        703 => "The Kyiv Independent",
        704 => "The Moscow Times",
        705 => "Folha de S.Paulo",
        706 => "Clarín",
        707 => "Corriere della Sera",
        708 => "Frankfurter Allgemeine Zeitung",
        709 => "La Repubblica",
        800 => "The New Yorker",
        801 => "The Atlantic",
        802 => "Vox",
        803 => "The Intercept",
        804 => "The Daily Beast",
        805 => "Democracy Now!",
        806 => "The Young Turks",
        807 => "The Hill",
        808 => "The Daily Caller",
        809 => "The Blaze",
        811 => "The Huffington Post",
        812 => "The Daily Wire",
        813 => "The Daily Kos",
        814 => "Vice News",
        815 => "Politico",
        816 => "Axios",
        817 => "The New Republic",
        818 => "The Nation",
        819 => "The American Prospect",
        820 => "The New Statesman",
        821 => "The Spectator",
        822 => "The New York Post",
        823 => "MSNBC",
        824 => "Fox News",
        825 => "Mother Jones",
        826 => "Reason",
        827 => "ProPublica",
        828 => "Jacobin",
        829 => "National Review",
        830 => "Newsweek",
        831 => "Time",
        832 => "Foreign Policy",
        833 => "The Christian Science Monitor",
        834 => "The Bulwark",
        835 => "Common Dreams",
        836 => "Drop Site News",
        837 => "Zeteo",
        838 => "The American Conservative",
        900 => "Variety",
        901 => "The Hollywood Reporter",
        902 => "Rolling Stone",
        903 => "Kotaku",
        904 => "Polygon",
        905 => "IGN",
        906 => "Billboard",
        907 => "Pitchfork",
        908 => "Empire",
        909 => "The Art Newspaper",
        910 => "GameSpot",
        912 => "Screen Rant",
        913 => "NME",
        914 => "The Texas Tribune",
        915 => "The Conversation",
        916 => "Salon",
        917 => "Alternet",
        918 => "Raw Story",
        919 => "Truthout",
        920 => "The Intercept - First Look",
        921 => "Nikkei Asia (English)",
        922 => "Bangkok Post - World",
        923 => "Bangkok Post - Business",
        925 => "Tom's Guide",
        928 => "Techdirt",
        929 => "Slashdot",
        930 => "Techmeme",
        932 => "Military Times",
        936 => "Investor's Business Daily",
        939 => "Benzinga",
        940 => "NerdWallet",
        942 => "The Independent (UK)",
        943 => "The Mirror (UK)",
        944 => "The Telegraph (UK)",
        945 => "France Info",
        946 => "Radio France Internationale",
        947 => "NHK World",
        952 => "Arab News",
        954 => "Cyprus Mail",
        955 => "Balkan Insight",
        956 => "Eurasianet",
        957 => "Global Voices",
        _ => panic!("unknown source number: {}", number),
    }
}

macro_rules! rss_source {
    ($name:ident, $number:expr, $scope:expr, $url:expr) => {
        pub struct $name;

        impl $crate::source::Source for $name {
            fn id() -> uuid::Uuid {
                $crate::source::feed::source_id($number)
            }
            fn name() -> &'static str {
                $crate::source::feed::source_name($number)
            }
            fn endpoints() -> Vec<$crate::source::endpoint::Endpoint> {
                $crate::source::feed::rss_endpoints(&[($scope, $url)])
            }
        }
    };
}

macro_rules! news_sitemap_source {
    ($name:ident, $number:expr, $scope:expr, $url:expr) => {
        pub struct $name;

        impl $crate::source::Source for $name {
            fn id() -> uuid::Uuid {
                $crate::source::feed::source_id($number)
            }
            fn name() -> &'static str {
                $crate::source::feed::source_name($number)
            }
            fn endpoints() -> Vec<$crate::source::endpoint::Endpoint> {
                $crate::source::feed::news_sitemap_endpoints(&[($scope, $url)])
            }
        }
    };
}

pub(crate) use {news_sitemap_source, rss_source};

pub(crate) fn rss_endpoints(feeds: &[(EndpointScope, &str)]) -> Vec<Endpoint> {
    feeds
        .iter()
        .map(|(scope, url)| rss_endpoint(scope.clone(), url))
        .collect()
}

pub(crate) fn news_sitemap_endpoints(feeds: &[(EndpointScope, &str)]) -> Vec<Endpoint> {
    feeds
        .iter()
        .map(|(scope, url)| news_sitemap_endpoint(scope.clone(), url))
        .collect()
}

pub(crate) fn wordpress_endpoints(feeds: &[(EndpointScope, &str)]) -> Vec<Endpoint> {
    feeds
        .iter()
        .map(|(scope, url)| wordpress_endpoint(scope.clone(), url))
        .collect()
}

pub(crate) fn ssr_json_endpoints(
    feeds: &[(EndpointScope, &str)],
    function: fn(&str) -> Vec<Article>,
) -> Vec<Endpoint> {
    feeds
        .iter()
        .map(|(scope, url)| ssr_json_endpoint(scope.clone(), url, function))
        .collect()
}

fn rss_endpoint(scope: EndpointScope, url: &str) -> Endpoint {
    Endpoint {
        url: url.parse().unwrap(),
        format: Format::RSS,
        scope,
        rules: Vec::new(),
    }
}

fn news_sitemap_endpoint(scope: EndpointScope, url: &str) -> Endpoint {
    Endpoint {
        url: url.parse().unwrap(),
        format: Format::GoogleNewsSitemap,
        scope,
        rules: Vec::new(),
    }
}

fn wordpress_endpoint(scope: EndpointScope, base_url: &str) -> Endpoint {
    Endpoint {
        url: wordpress_posts_url(base_url),
        format: Format::JSON,
        scope,
        rules: vec![Rule {
            section: ParseSection::AreaOfInterest,
            approach: ParseApproach::UseJSONParser {
                function: crate::parse::json::wordpress::parse_posts,
                headers: vec![
                    ("accept".to_string(), "application/json".to_string()),
                    ("user-agent".to_string(), USER_AGENT.to_string()),
                ],
                http1_only: false,
            },
        }],
    }
}

fn wordpress_posts_url(base_url: &str) -> reqwest::Url {
    let base_url = base_url.trim_end_matches('/');
    format!(
        "{base_url}/wp-json/wp/v2/posts?per_page=20&_embed=wp:featuredmedia&_fields=date,date_gmt,link,title,jetpack_featured_media_url,yoast_head_json.author,yoast_head_json.og_image,_embedded.wp:featuredmedia.source_url,_links.self"
    )
    .parse()
    .unwrap()
}

fn ssr_json_endpoint(
    scope: EndpointScope,
    url: &str,
    function: fn(&str) -> Vec<Article>,
) -> Endpoint {
    Endpoint {
        url: url.parse().unwrap(),
        format: Format::JSON,
        scope,
        rules: vec![Rule {
            section: ParseSection::AreaOfInterest,
            approach: ParseApproach::UseJSONParser {
                function,
                headers: vec![
                    (
                        "accept".to_string(),
                        "text/html,application/xhtml+xml,application/json".to_string(),
                    ),
                    ("user-agent".to_string(), USER_AGENT.to_string()),
                ],
                http1_only: false,
            },
        }],
    }
}
