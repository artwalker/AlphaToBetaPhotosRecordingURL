# AlphaToBetaPhotosRecordingURL

## The Video URL
[Build Your First Command Line To Do App with Rust - Microsoft Student Ambassadors Alpha to Beta](https://stdntpartners.sharepoint.com/:v:/s/MSPOpen2/EYdCGNhlUqxHtYsgHUWThPQBR-kuj5H9McB3ByOnVG3otg?e=8FXsos&nav=eyJyZWZlcnJhbEluZm8iOnsicmVmZXJyYWxBcHAiOiJTdHJlYW1XZWJBcHAiLCJyZWZlcnJhbFZpZXciOiJTaGFyZURpYWxvZy1MaW5rIiwicmVmZXJyYWxBcHBQbGF0Zm9ybSI6IldlYiIsInJlZmVycmFsTW9kZSI6InZpZXcifX0%3D)
[Youtube URL](https://www.youtube.com/watch?v=jDHHkzcnFks)
## Repository Introduction

Welcome to the repository documenting my journey as a Microsoft Student Ambassador, from Alpha to Beta stage! This repository contains records of the events I hosted, learning materials, and various activities undertaken during this journey.

### Contents

1. **Event Records**:

   - Detailed descriptions of each event, including themes, objectives, participants, content, outcomes, and feedback.
   - Photos, videos, and other multimedia materials from the events.
2. **Learning Materials**:

   - Documentation of the process of learning and using Microsoft technologies and tools.
   - Personal summaries and reflections.
3. **Projects and Examples**:

   - Project code and examples showcased and used during the events.
   - Outstanding projects and works submitted by participants.
4. **Resources and Links**:

   - Links to official Microsoft learning documents and courses.
   - Other useful learning resources and tools.

### Official Learning Resources

In this repository, you can find links to relevant learning materials and resources for the activities. For example, you can learn the basics of the Rust programming language through this link: [Rust First Steps](https://learn.microsoft.com/en-us/training/paths/rust-first-steps/).

### How TO USE

```rust

# In the cargo run command, arguments after -- are passed to your program, not to cargo. 
# If you run cargo run --help, the --help argument will be handled by cargo, not by your program.
$ cargo run -- -j test-journal.json add "buy milk"

$ cargo run -- -j test-journal.json add "take the dog for a walk"

$ cargo run -- -j test-journal.json add "water the plants"

$ cargo run -- -j test-journal.json list
1: buy milk                                           [2024-06-09 16:39]
2: take the dog for a walk                            [2024-06-09 16:39]
3: water the plants                                   [2024-06-09 16:39]

$ cargo run -- -j test-journal.json done 2

$ cargo run -- -j test-journal.json list
1: buy milk                                           [2024-06-09 16:39]
2: water the plants                                   [2024-06-09 16:39]

# create a new file .rust-journal.json in home directory
$ cargo run -- add "buy a surface pro."
$ cargo run -- list
$ cargo run -- done 1
```

### Contribution Guidelines

If you are interested in contributing or sharing your experiences and materials, feel free to submit a Pull Request or an Issue. Thank you for your participation and support!
