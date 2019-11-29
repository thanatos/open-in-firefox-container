`open-in-firefox-container`
===========================

The excellent add-on
["Open external links in a container"](https://addons.mozilla.org/en-US/firefox/addon/open-url-in-container/)
adds a URL scheme that allows opening links in a container. These URLs look
like this:

    ext+container:name=MyContainer&url=https://mozilla.org

I use this extension to open normal URLs clicked outside the browser in a
container. However, a small program is needed to translate
`https://mozilla.org` into the above format before passing it to Firefox; this
repository contains such a program.

On Linux, this program takes a single argument, the URL to open.

On OS X, this program takes no arguments; it registers itself as a handler for
http/https URL events and receives those events, and exits.

Installing
----------

First, get the
[extension mentioned above](https://addons.mozilla.org/en-US/firefox/addon/open-url-in-container/).

Then follow the platform-specific directions.


### Linux

Build the app; you will need Rust for this.

```
cargo build
```

Create a configuration for it at `~/.config/open-in-firefox-container.toml`:

```toml
firefox_command = "firefox-developer-edition"
container = "Work"
```

Of course, you can change the container name, or which Firefox to use. (The
above uses Firefox Developer Edition; if you have the normal Firefox, change
the command to have `firefox` instead.)

The application takes a single argument, the URL to open:

```
open-in-firefox-container 'https://google.com'
```


### OS X

Build the app; you will need Rust for this.

```
./macos/build_osx_app
```

Create the following configuration file at
`~/Library/Preferences/open-in-firefox-container.toml`:

```toml
firefox_command = "/Applications/Firefox Developer Edition.app/Contents/MacOS/firefox"
container = "Work"
```

Of course, you can change the container name, or which Firefox to use. (The
above uses Firefox Developer Edition; if you have the normal Firefox, change
the path to have `Firefox.app` instead.)


The build will have created `OpenInFirefoxContainer.app`. Run that, to register
it as the URL handler / "browser":

```
open ./OpenInFirefoxContainer.app
```

Click a link in some non-browser application; it should open in Firefox, in the
specified container.
