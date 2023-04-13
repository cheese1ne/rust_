use serde::ser::{Serialize, SerializeMap, Serializer};
use urlencoding::encode;

/**
 * Singer结构体
 */
#[derive(Debug)]
pub struct Singer {
    pub id: String,
    pub name: String,
}

/**
 * Singer实现序列化
 */
impl Serialize for Singer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("name", &self.name)?;
        map.end()
    }
}

/**
 * Album结构体
 */
#[derive(Debug)]
pub struct Album {
    pub id: String,
    pub name: String,
}

/**
 * Album实现序列化
 */
impl Serialize for Album {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.name.len()))?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("name", &self.name)?;
        map.end()
    }
}

/**
 * Song结构体
 */
#[derive(Debug)]
pub struct Song {
    pub id: String,
    pub name: String,
    pub image_url: String,
    pub download_url: String,
    pub singers: Vec<Singer>,
    pub albums: Vec<Album>,
}

/**
 * Song实现序列化
 */
impl Serialize for Song {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.name.len()))?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("image_url", &self.image_url)?;
        map.serialize_entry("download_url", &self.download_url)?;
        map.serialize_entry("singers", &self.singers)?;
        map.serialize_entry("albums", &self.albums)?;
        map.end()
    }
}

/**
 * Page结构体
 */
#[derive(Debug)]
pub struct Page {
    pub page: u32,
    pub page_size: u32,
    pub total: u32,
    pub songs: Vec<Song>,
}

/**
 * Page实现序列化
 */
impl Serialize for Page {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.total as usize))?;
        map.serialize_entry("page", &self.page)?;
        map.serialize_entry("page_size", &self.page_size)?;
        map.serialize_entry("total", &self.total)?;
        map.serialize_entry("songs", &self.songs)?;
        map.end()
    }
}

pub fn search_music() {
    let result = search("周杰伦", 1, 3, "");

    println!("the result is {:?}", result);
}

pub fn search(
    key_word: &str,
    current_page: u8,
    page_size: u8,
    quality: &str,
) -> Result<Page, String> {
    // 输出请求参数
    println!(
        "search {} {} {} {}",
        key_word, current_page, page_size, quality
    );
    // 组装请求url和参数
    let switch =
        r#"{"song":1,"album":0,"singer":0,"tagSong":0,"mvSong":0,"songlist":0,"bestShow":1}"#;
    let url = format!("http://pd.musicapp.migu.cn/MIGUM2.0/v1.0/content/search_all.do?ua=Android_migu&version=5.0.1&pageNo={}&pageSize={}&text={}&searchSwitch={}", current_page, page_size, (key_word), encode(switch));

    // 发起请求 获取结果
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    // 输出请求结果
    // println!("{:?}", response);
    // 对结果进行解析
    let data = json::parse(&response).unwrap();
    // println!("{:?}", data);
    // 返回状态
    let code = &data["code"].as_str().unwrap().parse::<i32>().unwrap();
    println!("code: {:?}", code);
    if *code != 0 {
        return Err(format!("code: {}", code));
    }
    if data["songResultData"]["totalCount"].is_null() {
        return Err(format!("no result"));
    }
    // 总条数
    let total_count = &data["songResultData"]["totalCount"].as_str().unwrap();
    // 歌曲详情列表
    let mut songs = Vec::<Song>::new();
    for song_info in data["songResultData"]["result"].members() {
        // 歌手
        let mut singers = Vec::<Singer>::new();
        for singer in song_info["singers"].members() {
            singers.push(Singer {
                id: singer["id"].as_str().unwrap().to_string(),
                name: singer["name"].as_str().unwrap().to_string(),
            })
        }
        // 专辑
        let mut albums = Vec::<Album>::new();
        for album in song_info["albums"].members() {
            albums.push(Album {
                id: album["id"].as_str().unwrap().to_string(),
                name: album["name"].as_str().unwrap().to_string(),
            })
        }
        // 歌曲
        let song_id = song_info["id"].as_str().unwrap().to_string();
        let song_name = song_info["name"].as_str().unwrap().to_string();
        let song_image_url = if song_info["imgItems"].members().len() > 0 {
            song_info["imgItems"][0]["img"]
                .as_str()
                .unwrap()
                .to_string()
        } else {
            "".to_string()
        };
        let mut song_tone_type = "SQ&formatType=SQ&resourceType=E";
        if quality == "HQ" {
            song_tone_type = "HQ&formatType=HQ&resourceType=2";
        }
        // 下载地址
        let song_download_url = format!("http://218.205.239.34/MIGUM2.0/v1.0/content/sub/listenSong.do?toneFlag={}&netType=00&copyrightId=0&&contentId={}&channel=0", song_tone_type, song_info["contentId"].as_str().unwrap().to_string());
        songs.push(Song {
            id: song_id,
            name: song_name,
            image_url: song_image_url,
            download_url: song_download_url,
            singers: singers,
            albums: albums,
        })
    }
    Ok(Page {
        page: current_page as u32,
        page_size: page_size as u32,
        total: total_count.parse::<u32>().unwrap(),
        songs: songs,
    })
}
