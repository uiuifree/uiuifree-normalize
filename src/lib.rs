use kana::wide2ascii;
use regex::Regex;
use voca_rs::*;
use lazy_static::lazy_static; // 1.3.0

pub fn normalize_search_text(value: String) -> String {
    remove_html(value).replace("・", "")
        .replace("株式会会社", "株式会社")
        .replace("社会福祉法人", "")
}

pub fn normalize_search_address(value: String) -> String {
    address_text(value).replace("丁目", "-")
        .replace("−", "-")
        .replace("番地の", "-")
        .replace("番地", "-")
        .replace("番", "-")
        .replace("号", "").trim_matches('-').to_string()
}

lazy_static! {
    static ref RE_NN: Regex = Regex::new(r"\n\n+").unwrap();
    static ref RE_SPACE: Regex = Regex::new(r"(\n(\s*|\n+)\n+(\s*))|(\n\s*)").unwrap();

}

pub fn remove_space(value: String) -> String {
    return RE_SPACE.replace_all(value.as_str(), "\n").to_string();
}

pub fn remove_html(value: String) -> String {
    let mut re_value = strip::strip_tags(value.as_str()).to_string();

    // re_value = Regex::new(r"</[^>]*?>").unwrap()
    //     .replace(caps.as_str(), "").to_string();

    re_value = re_value.replace("&nbsp;", " ")
        .replace("?", " ")
        .replace("　", " ")
        .replace("\t", " ")
        .replace("\n \n", "\n\n");

    return RE_NN.replace_all(re_value.as_str(), "\n\n").to_string();
}

pub fn free_text(value: String) -> String {
    // 全角を半角に統一
    return wide2ascii(value.replace("　", " ").as_str());
}

pub fn space(value: String) -> String {
    // 全角を半角に統一
    return value.replace("　", " ").as_str().to_string();
}


pub fn address_text(value: String) -> String {
    // 全角を半角に統一
    return wide2ascii(value.replace("　", " ").as_str())

        .replace("一", "1")
        .replace("二", "2")
        .replace("三", "3")
        .replace("四", "4")
        .replace("五", "5")
        .replace("六", "6")
        .replace("七", "7")
        .replace("八", "8")
        .replace("九", "9")
        .replace("〇", "10").trim().to_string();
}


#[cfg(test)]
mod tests {
    use crate::{free_text, remove_html};

    #[test]
    fn it_remove_html() {
        assert_eq!(remove_html(String::from("<div>a?</div>")), "a ");
    }

    #[test]
    fn it_works()
    {
        assert!("test hoge".contains("hoge"));
        assert_eq!(free_text(String::from("（）")), "()");
        assert_eq!(free_text(String::from("１２３ＡＢＣ")), "123ABC");
        assert_eq!(free_text(String::from("全　角")), "全 角");
        assert_eq!(free_text(String::from("AAa１２３４")), "AAa1234");
    }
}
