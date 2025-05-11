use uuid::Uuid;

pub struct Config {
    pub uuid: Uuid,
    pub host: String,
    pub proxy_addr: String,
    pub proxy_port: u16,

    pub main_page_url: String,
    pub sub_page_url: String,
    pub link_page_url: String,
    pub checkip_page_url: String,
    pub vmess_page_url: String,
    pub vless_page_url: String,
    pub trojan_page_url: String,
    pub ss_page_url: String,
    pub converter_page_url: String,
    
}
