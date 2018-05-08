## Wonder Cat Kyuu-chan Bot

I recently was introduced to [this amazingly wholesome 4-cell webcomic](http://helveticascans.com/r/read/wonder-cat-kyuu-chan/) and discovered that new strips are uploaded every day! As an avid telegram user, I decided I would set out to make a telegram bot to send me strips whenever they are available!

## [Follow it on telegram!](https://t.me/wondercatkyuu)

## Setup

1. Copy `Config.toml.default` to `Config.toml` and change the necessary fields.

## Running

1. Have [rust](https://www.rust-lang.org)
2. `cargo run`

## Running with Docker

* Build: `docker build -t wonder-cat-bot .`
* Run: `docker run --name telegrambot --restart always -v $(pwd)/last_comic:/usr/src/wondercatbot/last_comic -d wonder-cat-bot`
    * Make sure you `touch last_comic` before you run this or it will make a folder instead of a file!
* Stop: `docker kill telegrambot && docker rm -f telegrambot`

