# TAPM

### The Amnesic Password Manager (inspired by [Tails](https://tails.net))

TAPM is a password manager that never stores any passwords nor metadata, not even locally.

> [!WARNING]
> TAPM is safer than most password managers, but it's NOT magic.
>
> Learn about [ways attackers can gain access to your passwords.](#possible-risks)

## Table of Contents

- [Why us](#why-us)
- [How it works](#how-it-works)
- [Install guide](#install-guide)
- [Getting started](#getting-started)
- [Configuring](#configuring)
- [Possible risks](#possible-risks)
- [Creating a strong master password](#creating-a-strong-master-password)
- [License](#license)
- [Contributing](#contributing)
- [ToDo](#todo)

## Why us

Have you ever felt scared of the trust you put into your password manager? Don't worry, we got you!

TAPM has total amnesia out of the box. Your passwords, usernames, and any other metadata are never saved.

## How it works

All data outputed by TAPM is generated in real time with hashes. (SHA3-512)

The basic overview is:

```
hash(mpasswd + site + id)
```

## Install guide

1. Clone this repo with: `git clone https://github.com/salsa-trinity/tapm-cli`
2. Cd into the directory.
3. To compile, run: `cargo build --release`
4. Add to path with: `mv target/relese/tapm ~/.cargo/bin/tapm`

## Getting started

To open tapm, use the `tapm` command.
You will be prompted for the following things:

> [!NOTE]
> Just as when typing a sudo password, the entry isn't shown.

- Master: master password.
- Site: the website for this password. (it doesn't need to be the url)
- Id(optional): This is the id for accounts you have got in the same site, this is only useful if you've got more than one account.
- Flags(optional): Flags can either be sent directly with the `tapm` command, or be provided as another argument that you'll be prompted for.

Once provided with this arguments, TAPM will copy the output to the clipboard.

The list for available flags is:

- `tapm -h`: print this menu.
- `tapm -l`: specify the desired lenght of the outupt. Overwrites the config property.
- `tapm -p`: print the output, prevents from copying to the clipboard.
- `tapm -s`: ask for a single entry.
- `tapm -u`: output a username.

For more info, check out the README.md

> [!NOTE]
> If no config file is found, TAPM will default to use `wl-copy -o` for copying text to the clipboard.
>
> See [configuring](#configuring) for config guide.

## Configuring

The config file is stored in `~/.config/tapm/config.toml`. The config includes the following options:

See the [default config.](./docs/config.toml)

| Option | Data Type | Default | Description |
|:------:|:---------:|:-------:|-------------|
| copy_cmd | String | wl-copy -o | the command that gets executed when copying to the clipboard. |
| out_len | i32 | 25 | the maximum lenght an output is allowed to have. |

## Possible risks

Whilst TAPM is safer than most other password managers, there are still ways attackers can gain access to your data.

- Weak master password: If you use a weak master password, you can become vulnerable to a [brute force attack.](https://www.fortinet.com/resources/cyberglossary/brute-force-attack) Learn how to [generate a strong password](#creating-a-strong-master-password)
- Indiscretion with the master password: We highly recommend you **not** to write it anywhere, nor tell anyone about associated things to it.
- Peeking: Someone peeking into your computer keyboard while you type your master password.
- Spyware: Your computer could have been infected with malware, or spyware, such as a keylogger. If you are worried about spyware on a computer you don't own, check out [Tails.](https://tails.net)

## Creating a strong master password

To create a strong master password, it needs to be a long, randomized password, completely unrelated to you. But, you also need to be able to remember it.

My personal recommendation is:

- Grab 5 dice and throw them.
- Go to [the EFF diceware](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt) and look for the word it landed on.
- Slightly change it to be different, ex. Change some letters for numbers or special characters, capitalize some letters, omit or add random characters...
- Now, you have got the first word, keep repeating this until you have got at least 6 words.

## License

This project is licensed under the terms of [The MIT License.](https://mit-license.org/)

## Contributing

If you have any suggestions or questions, please open an issue, or make a pull request.

In the case that you find an exploitable vulnerability, please send an email to <salsa-trinity-wavy@duck.com>

Make sure the subject of the email is 'tapm vulnerability'.

