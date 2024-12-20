# Learn Rust by Games：The No-Frills Edition
[中文版](README_zh.md)

Author: Bin Wu

![image_cover.webp](./image_cover_learn_rust_by_games_3_cartoon_style.jpeg)

**Illustration: AI-generated. A crab and a micro:bit v2 development board sit beside a green leaf showing clear signs of rust fungus infection. Graydon Hoare, Rust language's creator, named it after rust fungus. These plant pathogens have complex life cycles with multiple hosts, enhancing their survival across various environments. Similarly, Rust ensures software stability through its strict, "over-engineered" enforcement of memory safety rules.**

## Introduction

"Learn Rust by Games: The No-Frills Edition" is a free limited-sharing ebook released under the CC BY-NC-ND 4.0 license. The book takes a unique approach to **teaching Rust by having readers create LED games, web games, and command-line games**. Throughout the learning process, we explore common programming pitfalls and their solutions. We also show how to leverage AI tools to assist with programming, making the learning journey both practical and engaging. Our goal is to help you master Rust while having fun.

This book caters to two types of beginners: those with no programming experience at all, and developers who know other programming languages but are new to Rust. After completing Chapter 1 and lighting up their first LED, readers interested in Rust embedded development can explore other specialized books on that topic. While the first part includes embedded development, our main focus is teaching the Rust programming language itself. We chose the micro:bit v2 platform for our starting point because there's something magical about controlling twinkling LEDs and cute little buttons with Rust code.

The content is divided into three parts—embedded, web, and command-line game development. Each chapter introduces specific Rust concepts through hands-on game projects. This practical approach allows readers to immediately apply what they've learned, reinforcing their understanding of Rust.

What sets this book apart from other Rust programming guides is its focus on three key principles for beginners:

- **Learn and apply immediately**: Experience quick wins that boost confidence and motivation
- **Make benefits visible**: See the practical value of each new concept and tool
- **Ask AI good questions**: Learn effective techniques for getting helpful AI responses

**If you like this ebook, please give it a star⭐️!**

The chapter titles will be refined and adjusted throughout the writing process.

## Table of Contents

### Part One: Lighting Up LEDs

- [Chapter 1: Lighting Up Your First LED and the Joy of Rust](./ch01/ch01.md)
- [Chapter 2: Making Your First LED Blink and Programming Supertools](./ch02/ch02.md)
- [Chapter 3: Memory Challenge Game and Variables](./ch03/ch03.md)
- Chapter 4: Virtual Pet Game and Ownership
- Chapter 5: Whack-a-Mole Game and Structs
- Chapter 6: Anti-Theft Alarm System and Error Handling
- Chapter 7: Reaction Test Game and Lifetimes
- Chapter 8: LED Text Scrolling Display and Automated Testing
- Chapter 9: Snake Game and Unsafe Rust

### Part Two: Building Multi-threaded Web Games

- Chapter 10: Online Tic-Tac-Toe Game and Cargo
- Chapter 11: Multi-User Chat Room and Packages, Crates, and Modules
- Chapter 12: Customizable Card Battle Game and Generics
- Chapter 13: Online Multiplayer Werewolf Game and Traits
- Chapter 14: Collaborative Puzzle Game and Smart Pointers
- Chapter 15: Real-time Leaderboard and Concurrency
- Chapter 16: Real-time Strategy Game Server and Asynchronous Programming

### Part Three: Creating Fun Command-Line Games

- Chapter 17: Sudoku Game and Common Collection Types
- Chapter 18: Text Adventure Game and Enums
- Chapter 19: Riddle Guessing Game and Pattern Matching
- Chapter 20: Word Frequency Counter and Iterators
- Chapter 21: Custom Calculator and Closures
- Chapter 22: Command-Line Mini Game Engine and Macros
- Chapter 23: Role-Playing Game and Object-Oriented Programming
- Chapter 24: Multi-functional Command-Line Tool and Advanced Types
- Chapter 25: Publishing Games to [crates.io](http://crates.io) and Sharing the Joy
- Appendix

## Copyright License Agreement

["Learn Rust by Games: The No-Frills Edition"](https://github.com/wubin28/learn_rust_by_games) © 2024 by Author [Bin Wu](https://github.com/wubin28) is licensed under [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/?ref=chooser-v1).

This book is licensed under the Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International License (CC BY-NC-ND 4.0).

This license allows you to share the book, but with the following strict limitations:

- Attribution (BY): When sharing, you must credit Bin Wu as the original author. This information must not be concealed or altered.
- NonCommercial (NC): The book is for non-commercial use only and may not be used for profit or commercial projects.
- NoDerivatives (ND): You may share the original version of this book, but you may not adapt, modify, or create derivative works. No changes or additions to the book's content are allowed.

Specifically, this license means:

- Share, but don't modify: You may share this book online, but it must remain in its original form without any modifications.
- No commercial use: The book must not be used in any commercial context, such as advertisements, publications, or paid projects.
- Protecting the original work's integrity: This license helps the author maintain the work's integrity and originality, preventing others from creating derivative works or using it commercially.

In essence, CC BY-NC-ND 4.0 is a relatively strict license. It allows free sharing of the book but prohibits any form of adaptation or commercial use.

## Book Code

You can download the code for this book from the GitHub repository at [https://github.com/wubin28/learn-rust-by-games](https://github.com/wubin28/learn-rust-by-games). Each code listing includes its specific location in the repository, making it easy for readers to find and run. The repository is organized by chapters, using the format chxx (where xx represents the chapter number).

## Development Environment Setup

To run the code in this book, please prepare the following devices and software according to your computer's operating system:

| **Your Computer's Operating System** | **Linux (Native or in WSL2 on Windows 10/11)** | **macOS** | **Windows 10/11** |
| --- | --- | --- | --- |
| micro:bit v2 development board | 1 piece | (Same as left) | (Same as left) |
| USB micro data cable | 1 piece | (Same as left) | (Same as left) |
| Command line shell | bash | zsh (macOS Catalina 10.15 and later); bash (before macOS Catalina 10.15) | cmd (Command Prompt) |
| Rust | Required | (Same as left) | (Same as left) |

Note: This book won't cover the setup of Linux in WSL2 on Windows 10/11 as it can be complex for beginners. For all other operating system configurations, please refer to Chapter 1.

## Icons We Use

Throughout this book, we use specific icons to highlight different types of information:

- ✅ Key actions and their concrete benefits
- ⚠️ Common pitfalls and how to avoid them
- 🔎 Optional deep dives for advanced understanding
- 🧠 Practice exercises and AI prompting techniques
- 💡 Solution hints and tips

## Acknowledgements

I extend my gratitude to Ms. Hailing Yang, editor at People's Posts and Telecommunications Press. Her keen perception of Rust programming as a trending topic, professional annotations for the initial chapters, and suggestion to share the e-book version for reader feedback were instrumental in shaping this work. Her efforts were the catalyst for creating this e-book.

I'm also thankful to the online friend "Don't Hit Xiao Lan" for their valuable input on the book's content. They offered three inspiring insights: the desire of programming beginners to complete a full project when learning Rust, the importance of discussing why beginners choose Rust, and the suggestion to create an English version to gather feedback from the international community where Rust might have a larger following. These perspectives led me to broaden the book's target audience from "readers with prior programming experience" to "complete beginners in programming or embedded development," and to prioritize the English version as the primary language.

## Frequently Asked Questions

### 1. Is this book suitable for programming beginners?

This book is ideal for two types of beginners: those with no programming experience whatsoever, and those  know other programming languages but are new to Rust. 

### 2. How does this book differ from other Rust learning resources?

We take a unique approach by teaching Rust concepts through the creation of engaging games, with a focus on embedded systems in Part One. Additionally, we explore common errors and pitfalls in learning Rust, helping readers gain a deeper understanding of the language's features and best practices.

### 3. Is this book complete?

The book is a work in progress, with an expected completion date around the end of July 2025. We regularly add and update chapters, so please check back often for the latest content.

### 4. How can I contribute to this book?

We welcome contributions of all kinds! If you spot errors, have ideas for improvements, or want to add content, please join our discussions, submit pull requests, or open issues on the GitHub repository. Your feedback will help improve this book for all Rust beginners. If your contribution makes a significant impact, I'll acknowledge you personally in the book's acknowledgments section.

### 5. Why is this a "No-Frills Edition"?

This edition is a precursor to a more comprehensive version planned for publication. We're sharing this limited version to gather reader feedback and refine the content. While this edition covers the core aspects of software project development for beginners, the published version will include additional content such as tips for using generative AI, common Rust learning pitfalls, and strategies to avoid them.

### 6. Is this book available in other languages?

Currently, we're writing the book in English and Chinese. If you're interested in translating it into other languages, please contact the author by opening an issue on our GitHub repository.

---

Thank you for your interest in "Learn Rust by Games: The No-Frills Edition"! We hope this book becomes a valuable resource on your Rust learning journey. Remember, the more pitfalls you encounter—and overcome—while writing games to learn Rust, the faster you'll progress. After all, those "pitfalls" often turn out to be our best teachers!