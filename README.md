# elega_core

### What is this?
This is meant to be code that is shared by Elega Corporation projects written in the Rust programming language. It includes data structures for managing memory, utilities for reading and parsing file formats such as comma-delimited data (CSV files), and is meant to be lightweight without a tangled dependency tree on other crates.

The goals of elega_core:
* Be a general purpose codebase for starting projects that utilize memory efficiently

* Avoid creating a long dependency tree of crates in Cargo.toml, which results in longer compile times and obscures deeper understanding of the code.

* Design with values that are closely aligned with the [handmade manifesto](https://handmade.network/manifesto), following the principle of ease-of-understanding to future developers, faster performance, and focusing on quality.

* Utilize design concepts that are found to work well in practical applications and real world experience.


This code is freely available to be used by others under the MIT license (see the LICENSE file). However, before using this code or adapting it to your own use, please note that we do not consider this as officially released and this is a pre-alpha period where the code is unstable and changing rapidly.

### Who uses this, or what is its use case?
At the time of this writing, Elega Corporation uses this code in its own static site generator for its website, tools and scripts for working with its financial & web scraped data, and with a proprietary game engine it has begun developing. 

The code in this repository can be utilized for a variety of programs, including those with a graphical user interface (GUI), tools that process massive data volumes, or any tools that require few dependencies, high performance, or the other general utilities found here.

### Should I use elega_core?
It's recommend you consider using elega_core when it is closer to a final release. For now, periodically check up on [ElegaCorp.com](https://www.elegacorp.com) to follow updates regarding an official release of this code. 

### Will this become a Rust crate or library?
It is possible this project will eventually become a library and Rust crate available through the Rust package index when it's more stable. For now,it is meant to be copied directly into Rust projects. For example, a `cargo init` started project creates the `src` folder, where the `common` folder from this repository can be copied. This allows the developer to see exactly what is implemented in this code, how it's done, and provides them the option of changing it for their use in their given program, as well as providing assurance that nothing is changing due to crate versioning without their knowing.

When and if this is introduced as a crate / library, it will need to consider some of these stability and comprehension issues.