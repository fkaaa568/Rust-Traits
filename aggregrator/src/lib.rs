pub mod my_lib{
    #[derive(Debug)]
    pub struct newArticles{
        pub Headline:String,
        pub Location:String,
        pub Author:String,
    }

     pub trait summary{
          fn summarized(&self)-> String;
    }
    impl summary for newArticles{
           fn summarized(&self)-> String{
            let mut result_01 = format!("The Headline is : {} , by The Location is : {} ({}) ",self.Headline,self.Location,self.Author);
            result_01
        }
    }

    #[derive(Debug)]
    pub struct Tweet {
        pub Username: String,
        pub Content: String,
        pub Reply : bool,
        pub Retweet : bool,
    }


    impl summary for Tweet {
        fn summarized(&self)-> String{
            let mut result_02 = format!("The User Name is : {} , The Content is : {}",self.Username,self.Content);
            result_02
        }
}
}