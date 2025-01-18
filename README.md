# Scrcpy Wrapper

A simple wrapper for [scrcpy](https://github.com/Genymobile/scrcpy).

## Usage

Download from the [releases](https://github.com/Bluemangoo/scrcpy-wrapper/releases) page and extract the zip file (recommended to extract to scrcpy folder).

*It will automatically search for the scrcpy executable from the same folder and the PATH.*

Run the executable.

### Settings

It will save the settings to `scrcpy-wrapper.toml` under `$XDG_CONFIG_HOME`(if exists) or `~/.config` .

### Language

We support English and Chinese (Simplified).

If you want to change the language, you can modify `language` in `scrcpy-wrapper.toml` to `en` or `zh`.

```toml
language = "en"
```

### Reset

If you want to reset the settings, just delete the `scrcpy-wrapper.toml` file.
