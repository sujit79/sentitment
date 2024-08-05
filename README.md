## Experimental Code

Code maps input string to array of strings and performs sentiment analysis.
Not tested....

# Sentiment

This project was generated using the Kickstart template.

You can run this project with or without Hadoop, using the following shell instructions:

```shell
# build the binaries
$ cargo build --release

# run with Hadoop Streaming
$ hadoop jar hadoop-streaming-2.8.2.jar \
    -input <INPUT> \
    -output <OUTPUT> \
    -mapper ./target/release/sentiment_mapper \
    -reducer ./target/release/sentiment_reducer

# run with Unix command shimming
$ cat <INPUT> | \
    ./target/release/sentiment_mapper | \
    sort -k1,1 | \
    ./target/release/sentiment_reducer \
    > <OUTPUT>
```
