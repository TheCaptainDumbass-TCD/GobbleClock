# Gobble Clock

Desktop app for Debian 13 and Ubuntu that plans every step of a slow cooked Thanksgiving turkey. Enter the weight and the time you want to carve, and Gobble Clock works backward to tell you when to take the turkey out of the refrigerator, when it goes in the oven and when to turn the oven down. It runs entirely on your own machine.

## The method

Gobble Clock follows the slow cook method from [howtocookathanksgivingturkey.com](https://howtocookathanksgivingturkey.com/) and the Turkey Schedule spreadsheet it replaces.

1. Take the turkey out of the refrigerator 1 hour before cooking.
2. Prep it for 20 minutes while the oven preheats to 475°F.
3. Roast uncovered at 475°F for 20 minutes.
4. Turn the oven down to 250°F without opening the door. Cook 20 minutes per pound, timed from the moment you turn it down.
5. Put the lid on 15 minutes after turning it down, and take it off for the last 20 minutes to brown.
6. Take it out, check the temperature (170°F breast, 180°F thigh) and rest 20 minutes before carving.

Weights round to the nearest pound for timing: under half a pound rounds down, half a pound or more rounds up. The website recommends this method only for turkeys of 20 pounds or less.

Always check the turkey with a meat thermometer. Timing is a guide. The thermometer is the final word.

## Features

- Full schedule from weight and carving time, with the day shown when a step falls on a different day
- Live countdown, the current pound and a desktop notification with a chime at every step
- A cook in progress survives a restart, and Gobble Clock asks before quitting during a cook
- One page printable checklist for the refrigerator, with a checkbox and time for every step
- Save the checklist as text or as a calendar file with an alarm for every step
- Thaw planner that gives the day to move a frozen turkey to the refrigerator
- Fresh turkey mode with an early temperature check
- Thawing window with refrigerator thaw times and safety tips
- Recipe tab that credits howtocookathanksgivingturkey.com and links to it
- Turkey breast only mode: breast only thermometer check and an early check 30 minutes before it comes out
- Optional wiggle room so the turkey is ready early
- Lid options: cover then brown, covered or uncovered
- History of every year with ratings and notes, starting with 2020, 2021, 2024 and 2025 from the spreadsheet
- °F or °C and lb or kg
- Harvest light, fireside dark and system themes
- Advanced method times, with Reset to recipe

## Install

```sh
sudo apt install ./gobbleclock_0.1.3_amd64.deb
```

Your data lives in `~/.local/share/dev.quietwire.gobbleclock`. Removing the package leaves it in place.

## Build

See `CONTRIBUTING.md`.

## Credits

The slow cook method comes from [How to Cook a Thanksgiving Turkey](https://howtocookathanksgivingturkey.com/). All credit for the recipe goes to the site's author. The tips in the app are short summaries in new wording, and the full recipe is on the site. Gobble Clock is an independent app and is not affiliated with or endorsed by the website or its author.

## Food safety

Gobble Clock times the steps, but ovens and turkeys vary. Always check the turkey with a meat thermometer before serving. The app comes with no warranty, as stated in the license.

## License

Copyright (C) 2026 Dennis Ayotte. GPL-3.0-or-later. See `LICENSE`.
