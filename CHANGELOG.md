# Changelog

Gobble Clock follows semantic versioning. Versions below 1.0.0 are pre-release builds.

## 1.1.0

- Settings has a new button that opens the Gobble Clock website
- The package homepage is now the Gobble Clock website

## 1.0.0

First public release.

- New Reset button on the Plan tab that clears the weight and puts the day, carving time and checkboxes back to the defaults. It is grayed out while a cook is running or when there is nothing to reset
- Settings shows the app version and has a button that opens the Gobble Clock GitHub page
- Settings and carving dates that could not come from the Settings screen, such as a hand edited file, are refused with a clear message instead of producing a broken schedule
- Recipe links open through the Tauri opener and only the three recipe pages are allowed
- Reminders keep running even if an earlier error left the app data locked
- Windows 11 installer configuration with a Windows reminder sound

## 0.1.3

- New Thawing button in the planner with the refrigerator thaw times table, your turkey's thaw date and safety tips
- New Recipe tab that credits howtocookathanksgivingturkey.com, links to the recipe, the roasting time chart and the convection page, and sums up the tips in short form

## 0.1.2

- New "Turkey breast only" option for a breast with no legs or wings. The method and times stay the same, the thermometer check is the breast only at 170°F and there is an extra temperature check 30 minutes before it comes out
- The breast only choice is saved in history and shown on the printout and exports

## 0.1.1

- The weight box now accepts decimals such as 7.35. A comma also works as the decimal point

## 0.1.0

- Plan every step of a slow cooked turkey from its weight and carving time
- Recipe defaults from howtocookathanksgivingturkey.com: 475°F for 20 minutes, then 250°F at 20 minutes per pound with the door closed
- Weights round to the nearest pound: under half rounds down, half or more rounds up
- Thermometer reminder on every schedule, printout and export: 170°F breast, 180°F thigh
- Live countdown with desktop notifications and a chime at every step, resumed after a restart
- One page printable refrigerator checklist, text export and calendar export with alarms
- Thaw planner, fresh turkey mode, optional wiggle room and lid options
- History with ratings and notes, seeded with 2020, 2021, 2024 and 2025 from the spreadsheet
- °F or °C, lb or kg, light, dark and system themes
- Advanced method times with Reset to recipe
