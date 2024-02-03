# SRT Subtitle Cleaner
## What is it?
This app parses SRT subtitle files and strips all CSS formatting from the text.

#### Example
Consider the below line
```
1
00:00:03,433 --> 00:00:05,766
<font face="Open Sans" size="72">Hey everyone, my name is claw726.</font>
```

Sometimes, the application will not parse the CSS formatting and instead display it to the viewer, showing the below exerpt on their screen:

```<font face="Open Sans" size="72">Hey everyone, my name is claw726.</font>```

Obviously, they don't want to see this CSS scripting.

So, what this app does is remove it from the file, so we get the new text below:

```
1
00:00:03,433 --> 00:00:05,766
Hey everyone, my name is claw726.
```

Much cleaner.

## How do I use it?
Simple. Either download an executable from the releases page, or compile it yourself.

### Compile
0. Download and install Rust: https://www.rust-lang.org/tools/install
1. Clone the repository using `git`
2. `cd` into the repository

    ```cd ~\SRT_Subtitle_Cleaner```
3. Compile the app

    ```cargo build --release```

4. Run the app

    ``` cargo run```

### Release:
To use the executable file, run the executable and select the `.srt` file youd like to clean.

