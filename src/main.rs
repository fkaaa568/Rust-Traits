extern crate aggregrator; 
use crate::aggregrator::my_lib::summary;
use aggregrator::my_lib::newArticles;
use aggregrator::my_lib::Tweet;

fn main() {
    let article_01 = newArticles{
        Headline:"The Twenty world cup win".to_string(),
        Location:"Pakistan,Karachi,National Stadium".to_string(),
        Author:"Arthur Conan Doyle".to_string().to_string(),
    };

    println!("The sumary is now that {:#?}",article_01.summarized());

    
    let tweet_01 = Tweet{
        Username:"Fkaaa560@gmail.com".to_string(),
        Content:"The Exciting Cricket Match".to_string(),
        Reply:false,
        Retweet:false,
        
    };

    println!("The sumary is now that {:#?}",tweet_01.summarized());

}
