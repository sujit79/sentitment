//! `Reducer` implementation for the sentiment project.
extern crate efflux;

use efflux::prelude::{Context, Reducer};
extern crate vader_sentiment;

fn main() {
    // execute the reduction phase
    efflux::run_reducer(SentimentReducer);
}

/// The struct which will implement the `Reducer` trait.
struct SentimentReducer;

fn flatten_slice(input: &[&[u8]]) -> Vec<u8> {
    input.iter().flat_map(|&slice| slice.iter()).copied().collect()
}
fn option_string_to_u8(opt: Option<String>) -> Option<u8> {
    opt.and_then(|s| s.parse::<u8>().ok())
}
fn option_to_u8(opt: Option<u8>) -> u8 {
    opt.unwrap_or(0)
}
fn option_f64_to_string(opt: Option<&f64>) -> Option<String> {
    opt.map(|&value| value.to_string())
}
/// An empty implementation of the `Reducer` trait.
impl Reducer for SentimentReducer {
   
    fn setup(&mut self, _ctx: &mut Context) {
        // Carry out any setup required in this block.
    }

    fn reduce(&mut self, key: &[u8], values: &[&[u8]], ctx: &mut Context) {
        
        // Carry out the main reducer tasks inside this block.

        let binding_value = flatten_slice(values);
        let sentiment_text: &str = std::str::from_utf8(&binding_value).unwrap();
        
        let analyzer = vader_sentiment::SentimentIntensityAnalyzer::new();
    
        let binding_score = analyzer.polarity_scores(sentiment_text);
        let polarity_scores: Option<&f64> = binding_score.get(&"compound");
 
        let value: u8 = option_to_u8(option_string_to_u8(option_f64_to_string(polarity_scores)));
        ctx.write(key, &[value]);
    }

    fn cleanup(&mut self, _ctx: &mut Context) {
        // Carry out any cleanup required in this block.
    }
}
