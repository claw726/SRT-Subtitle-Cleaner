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
Simple. Either download an executable fromm the releases page, or compile it yourself.

### Compile
0. Download and install Rust: https://www.rust-lang.org/tools/install
1. Clone the repository using `git`
2. `cd` into the repository

    ```cd ~\SRT_Subtitle_Cleaner```
3. Compile the app

    ```cargo build --release```

4. Run the app

    ``` cargo run <SRT FILE PATH>```

### Release:
To use the executable file, simply place your `.srt` subtitle file you want to clean in the same folder as the executable file. Open your terminal and `cd` to the same folder.

#### Example:
```
cd ~\SRT_Subtitle_Cleaner\
.\subtitle_cleaner.exe <SRT_File>
```
Where `<SRT_FILE>` is the name and extension of the `.srt` file you want to clean
