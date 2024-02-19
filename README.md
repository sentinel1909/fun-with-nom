# Fun with Nom

This repo is created as a learning resource for the [Nom](https://crates.io/crates/nom) crate for Rust. Nom is a parser combinators library. It can be used to parse just about anything, text, binary, whatever.

Right now, there isn't much to this project, but it will grow over time. I've spent the last week deep diving into this crate, mostly due to the desire to solve Day 6 of the [Shuttle Christmas Code Hunt](https://www.shuttle.rs/cch) challenge (now called Shuttlings)

## Concepts

*Coming Soon...*

## Where I'm At

I understand how to compose the simple building block parsers.

## Challenges to Overcome

I'm beginning to understand combinators. There is now a `/count` endpoint which accepts a plain text string of any length. The endpoint responds with the word count. Nom is leveraged here via calling a parser which returns the text input as a vector of words. The `.len()` method is then called to get the word count.

## Nom Resources

I feel I've "reached the end of the internet" on what folks have written about regarding this crate. There just isn't that much out there. You have to be careful because much of the content is outdated. Better articles are:

- [Parsing Text with Nom](https://blog.adamchalmers.com/nom-chars/)
- [Parsing Data in Rust with Nom](https://hector.dev/2022/12/23/parsing-data-in-rust-with-nom/)
- [Planespotting with Rust: Using Nom to Parse ADS-B Messages](https://www.kirillvasiltsov.com/writing/parse-ads-b/)
- [Creating a Bencode Parser with Nom](https://edgarluque.com/blog/bencode-parser-with-nom/)

## Hand Crafting a Parser

It's worth knowing how to do this. I'm all hot to trot about Nom, but I think a good chunk of the issues I'm having stem from not knowing the basics.

- [You already know how to parse by hand](https://vfoley.xyz/parsing/)