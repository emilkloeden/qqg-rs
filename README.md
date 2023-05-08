# qqg

A small CLI search tool.

## Installation

Download the latest binary for Linux or Windows on the repository [releases](https://github.com/emilkloeden/qqg-rs/releases) page.

## Usage

```usage
USAGE:
    qqg [OPTIONS] <QUERY>

ARGS:
    <QUERY>    The search query

OPTIONS:
    -h, --help       Print help information
    -H, --headers    Output only the headers of the search results
    -j, --json       Output results as JSON
    -V, --version    Print version information
```

#### Examples:

Standard (markdown) output

```$ qqg "tokyo disney"

## [Official site](https://www.tokyodisneyresort.jp/)

Tokyo Disney Resort


## [[Official]Tokyo Disney Resort Official WebSite|Tokyo Disney Resort](https://www.tokyodisneyresort.jp/en/)

Tokyo Disney Resort. Anniversary Event "Tokyo Disney Resort 40th 'Dream-Go-Round'" Apr 15, 2023 ～ Mar 31, 2024


## [Tokyo Disney Resort Online Reservations & Tickets](https://reserve.tokyodisneyresort.jp/en/about/vp_01)

Attraction Tickets included in the Tokyo Disney Resort Vacation Packages allow guests to select an attraction when making a reservation/purchase for a Vacation Package plan. With this ticket, guests can experience popular attractions with a minimal wait. There are two types of Attraction Tickets: One where guests select a time to experience an ...


## [Tokyo Disneyland (Maihama): All You Need to Know BEFORE You Go](https://www.tripadvisor.com.au/Attraction_Review-g14134868-d320634-Reviews-Tokyo_Disneyland-Maihama_Urayasu_Chiba_Prefecture_Kanto.html)

About. Tokyo Disneyland—the Kingdom of Dreams and Magic. This is one of the world's largest theme park with great access from central Tokyo, located only 15 minutes away by train from Tokyo Station. You'll find numerous exciting attractions that will sweep you away into the world of the Disney films, enthralling entertainment that will ...

...

```

Headers only (note that this returns lowercased text)

```
$ qqg -H "tokyo disney"

[official]tokyo disney resort official website|tokyo disney resort
tokyo disney resort online reservations & tickets
tokyo disneyland (maihama): all you need to know before you go
tokyo disneyland - tokyo disney resort guide - japan-guide.com
tokyo disneyland (maihama) - all you need to know before you go
...

```

JSON output

```
$ qqg -j "tokyo disney"
[
    {"title":"Tokyo Disney Resort Park tickets: 1-Day passport - Klook","href":"https://www.klook.com/activity/695-tokyo-disney-resort-1-day-pass-tokyo/","description":"Spend 1 day at Tokyo Disneyland or Tokyo DisneySea with pre-booked e-tickets to ensure convenient entry. Get full access to the park including all the shows and entertainment with a Tokyo Disney Ticket. Enjoy utmost convenience with no printing required - just scan the QR code, get your re-entry stamp, and enjoy!"},
    ...
]


```
