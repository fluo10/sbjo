mod config;
mod journal;
mod page;
mod data;

pub use config::Config;
pub use journal::Journal;
//pub use journal::Journal;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let config = super::Config::new();
        config.show();
        let journal = super::Journal::from_config(config).unwrap();
        //journal.load_pages();
    }
    #[test]
    fn get_markdown () {
        let config = super::Config::new();
        let journal = super::Journal::from_config(config).unwrap();
        assert_eq!(journal.data.pages.len(), 1);
        
    }
}