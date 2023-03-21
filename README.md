Finds the optimal material combination and invention level for a given set of perks.

Use `perk_solver --help` to display the help text.

## Options
Usage: `perk_solver [OPTIONS] --type <GIZMO_TYPE> --level <INVENTION_LEVEL> <COMMAND>`

| Command          | Description                                                    |
|------------------|----------------------------------------------------------------|
| `gizmo`          | Find the optimal material combination of a given gizmo.        |
| `material-input` | Show the gizmo probabilities for a given material combination. |

Options:
| Short | Long                        | Description                                                | Required |
|:-----:|-----------------------------|------------------------------------------------------------|:--------:|
| `-t`  | `--type <GIZMO TYPE>`       | Possible values: `weapon`, `w`, `armour`, `a`, `tool`, `t` |    Yes   |
| `-l`  | `--level <INVENTION LEVEL>` | Use two values separated by a comma to search in a range.  |    Yes   |
| `-a`  | `--ancient`                 | For ancient gizmos                                         |    No    |

### Gizmo command
Usage: `perk_solver.exe --type <GIZMO_TYPE> --level <INVENTION_LEVEL> gizmo [OPTIONS] <PERK> [RANK] [PERK_TWO] [RANK_TWO]`

| Argument     | Description                                                                                                                                               | Required |
|--------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------|:--------:|
| `<PERK>`     | Perk to look for. Not case sensitive. use `"` quotes if it contains spaces.                                                                               |    Yes   |
| `[RANK]`     | Rank of the first perk. Use `1` for binary perks like mobile [default: `1`]                                                                               |    No    |
| `[PERK TWO]` | Second perk in the gizmo. Use `any` if you don't care what the second perk is. Leaving this field empty or using the string `empty` means no second perk. |    No    |
| `[RANK TWO]` | Rank of the second perk [default: `1`]                                                                                                                    |    No    |

Options:
| Short | Long                        | Description                                                                                                                                                                                                                 | Required |
|:-----:|-----------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|:--------:|
|  `-f` | `--fuzzy`                   | Use this if you don't care what the second perk is. Is set automatically is second perk is `any`.                                                                                                                           |    No    |
|  `-e` | `--exclude <EXCLUDE>`       | Comma separated list of material values to exclude. Uses basic substring matching. Put `"` quotes around the entire list if it contains spaces.                                                                             |    No    |
|  `-s` | `--sort-type <SORT TYPE>`   | Sort the result on probability per consumed gizmo, probability per attempt, or on estimated price [default: `price`] [possible values: `gizmo`, `attempt`, `price`]                                                         |    No    |
|  `-A` | `--alt-count <ALT COUNT>`   | Amount of alternative combinations to show (second best, third best, ...) [default: `0`]                                                                                                                                    |    No    |
|       | `--out-file <OUT FILE>`     | Output file name. Set to `false` to disable output [default: `out.csv`]                                                                                                                                                     |    No    |
|       | `--price-file <PRICE FILE>` | Prices file name. If the file already exist prices are loaded form the file; if not, they are loaded from the wiki. Set to `false` to disable. When disabled prices are always loaded from the wiki [default: `prices.txt`] |    No    |

### Material intput command
Usage: `perk_solver.exe --type <GIZMO_TYPE> --level <INVENTION_LEVEL> material-input <MATS>...`

| Argument | Description                                                                                                                        | Required |
|----------|------------------------------------------------------------------------------------------------------------------------------------|:--------:|
| `<MATS>` | Comma separated list of materials. Not case sensitive. Shorter names are accepted (e.g. `precise` instead of `Precise components`) |    Yes   |

Options:
| Short | Long                        | Description                                                | Required |
|:-----:|-----------------------------|------------------------------------------------------------|:--------:|
| `-t`  | `--type <GIZMO TYPE>`       | Possible values: `weapon`, `w`, `armour`, `a`, `tool`, `t` |    Yes   |
| `-l`  | `--level <INVENTION LEVEL>` | Single value.                                              |    Yes   |
| `-a`  | `--ancient`                 | For ancient gizmos                                         |    No    |

## Example
```sh
$ perk_solver -t weapon -l 50,80 -a gizmo equilibrium 4 mobile -e connector,delicate,flexible -A 5
```
![](./images/Example_output.png)

## What are conflict materials
Conflict materials are materials that can generate perks ranks with an equal cost value as one of the wanted perk ranks.
This matters as equal cost values can cause unstable sorting results so for these material combinations the order
of the materials might matter whereas gizmos made entirely from non-conflict materials are always position independent.
The result is that the amount of conflict materials has a greater impact in the total number of combinations to check.
So if the search takes too long it is more effective to exclude conflict materials.

## Build from source
* Clone this repo `git clone https://github.com/CephHunter/Runescape-perk-solver.git`
* Make sure [Rust](https://www.rust-lang.org/) is installed
* Build with `build.sh` or `build.bat`
