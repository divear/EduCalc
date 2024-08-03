# EduPage grade average calculator

For some reason edupage doesn't have this built in, so I made this small script to calculate what would happen to your grade average if you received a new grade.



- Internet connection required


## How do I use this?

1. Clone this repo, create a `.env` file in root directory and add your EduPage credentials:

```env
USERNAME=your_username
PASSWORD=your_password

```

2. Run it with:

```
cargo run
```

## How does this work?

It just scrapes the site and calculates it.

## Known issues

- Edupage is unreliable and sometimes the results simply can't be scraped
