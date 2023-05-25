pub trait Summary {
    fn length(&self) -> usize;
    fn def_summary(&self) -> () {
        println!("Default Summary");
    }
}

trait SummaryOne {
    fn def_len(&self) -> () {
        println!("3");
    }
}

struct NewsArticle {
    content: String,
}

impl Summary for NewsArticle {
    fn length(&self) -> usize {
        *&self.content.len()
    }
}

//Trait Parameter
fn read_article(article: &impl Summary) {
    article.def_summary();
}

//Trait Bound
fn read_article_one<T: Summary>(article: &T) {
    article.def_summary();
}

//Multiple Trait Bound
fn read_article_two<T: Summary + SummaryOne>(article: &T) {
    article.def_summary();
}

//Multiple params with Trait Bound
fn read_article_three<T: Summary + SummaryOne, U: Summary>(article: &T, article_one: &U) {
    article.def_summary();
}

//Multiple params with Trait Bound with where clauses
fn read_article_four<T, U>(article: &T, article_one: &U)
    where
        T: Summary + SummaryOne,
        U: Summary

{
    article.def_len();
    article_one.def_summary();
}

pub fn main() {
    let article_one = NewsArticle {
        content: String::from("Siva"),
    };
    println!("{}", article_one.length());
    article_one.def_summary();

    read_article(&article_one);
}