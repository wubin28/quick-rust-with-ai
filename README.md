# From Rust Rookie to AI Full Stack：The No-Frills Edition
[中文版](README_zh.md)

Author: Bin Wu

![image_cover.png](./image-cover.png)

**Illustration: AI-generated.**

## Introduction

["From Rust Rookie to AI Full Stack: The No-Frills Edition"](https://github.com/wubin28/from-rust-rookie-to-ai-full-stack) is a free e-book available under the CC BY-NC-ND 4.0 license. Through creating a series of engaging mini-games, this book helps readers quickly progress from Rust beginners to full-stack engineers who can master AI skills and solve real-world problems.

This book takes a unique approach to **teaching the Rust programming language: it guides readers through creating various types of games, including LED games, command-line games, web games, WebAssembly games, and AI model training games**. Throughout the game development process, we will demonstrate how to progressively ask AI the right questions, leading readers through the complete journey from requirements analysis to deployment. This not only helps understand how Rust concepts solve specific problems but also teaches readers how to delegate AI-manageable details to AI.

The "Rust Rookie" in the book's title indicates that it's particularly suitable for two types of beginners: complete newcomers with no programming experience, and developers who have programming experience but haven't worked with Rust before. For programmers interested in learning Rust embedded development, they can transition to other specialized Rust embedded development books after completing the first chapter and lighting up their first LED. While the first part of this book involves Rust embedded development, the overall focus remains on the Rust programming language itself. We chose to start with micro:bit v2 embedded development because controlling twinkling LEDs and cute little buttons with Rust code is simply irresistible.

This book divides Rust fundamentals into five parts, focusing on embedded systems, command line, Web, WebAssembly, and AI model training game development. Each chapter carefully incorporates games designed around specific Rust concepts. This organizational approach offers a significant advantage: readers can grow rapidly into AI full-stack engineers through hands-on practice with Rust projects across 5 different domains.

Compared to other Rust programming introductory books, this book offers the following unique features to create an exceptional learning experience for Rust beginners:

- **Learn and Apply Immediately**: Allowing beginners to experience a sense of achievement quickly during the learning process
- **Problem-Solving First**: Every new concept and tool corresponds to specific problems, helping you understand their practical applications
- **Ask AI Good Questions**: Providing effective questioning techniques to improve AI response accuracy

**If you like this ebook, please give it a star⭐️!**

The chapter titles will be refined and adjusted throughout the writing process.

## Table of Contents

### Part One: Lighting Up LEDs

- [Chapter 1: Lighting Your First LED and the Joy of Rust](./ch01/ch01.md)
- [Chapter 2: Making Your First LED Blink and Programming Tools](./ch02/ch02.md)
- [Chapter 3: Memory Challenge Game and Variables](./ch03/ch03.md)
- Chapter 4: Virtual Pet Game and Ownership
- Chapter 5: Whack-a-Mole Game and Structures

### Part Two: Creating Fun Command Line Games

- Chapter 6: Endless Tic-Tac-Toe and Error Handling
- Chapter 7: Typing Speed Test Game and Lifetimes
- Chapter 8: Maze Generation and Automated Testing
- Chapter 9: Task Management Simulation Game and Cargo
- Chapter 10: Publishing a Text Adventure Game to [crates.io](http://crates.io) and Sharing the Joy

### Part Three: Building Multi-threaded Web Games

- Chapter 11: Multiplayer Chat Room and Packages, Crates, and Modules
- Chapter 12: Customizable Card Battle Game and Generics
- Chapter 13: Online Multiplayer Werewolf and Traits
- Chapter 14: Collaborative Puzzle Game and Smart Pointers
- Chapter 15: Real-time Leaderboard and Concurrency
- Chapter 16: Real-time Multiplayer Shooter and Unsafe Rust
- Chapter 17: Real-time Strategy Game Server and Asynchronous Programming

### Part Four: Creating Fun WebAssembly Games

- Chapter 18: Tetris and Common Collection Types
- Chapter 19: Board Games and Enums
- Chapter 20: Event-Driven Games and Pattern Matching
- Chapter 21: Virtual Pet Simulator and Object-Oriented Programming

### Part Five: Igniting AI Model Training

- Chapter 22: Data Loading and Iterators
- Chapter 23: Building Neural Networks and Closures
- Chapter 24: Automated Model Training and Macros
- Chapter 25: AI Model Encapsulation and Advanced Types

## Copyright License Agreement

["From Rust Rookie to AI Full Stack: The No-Frills Edition"](https://github.com/wubin28/from-rust-rookie-to-ai-full-stack) © 2025 by [Wu Zhenben](https://github.com/wubin28) is licensed under [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/?ref=chooser-v1).

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

The code for this book can be downloaded from the GitHub repository at https://github.com/wubin28/from-rust-rookie-to-ai-full-stack. Each code listing is marked with its specific location in the repository for easy reference and execution. The repository is organized by chapters in the format chxx (where xx is the chapter number).

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

- ❓ Asking AI Good Questions
- 💡 Key Points from AI Responses
- 👍 How Rust Concepts Solve Specific Problems
- 🤖 Rust Concept Details That Can Be Handled by AI Without Memorization
- ✅ Actions and Benefits
- ⚠️ Important Pitfalls to Watch Out For and How to Avoid Them
- 🔎 Deep Dive Topics — Optional but Knowledge-Expanding

## Acknowledgements

I extend my gratitude to Ms. Hailing Yang, editor at People's Posts and Telecommunications Press. Her keen perception of Rust programming as a trending topic, professional annotations for the initial chapters, and suggestion to share the e-book version for reader feedback were instrumental in shaping this work. Her efforts were the catalyst for creating this e-book.

I'm also thankful to the online friend "Don't Hit Xiao Lan" for their valuable input on the book's content. They offered three inspiring insights: the desire of programming beginners to complete a full project when learning Rust, the importance of discussing why beginners choose Rust, and the suggestion to create an English version to gather feedback from the international community where Rust might have a larger following. These perspectives led me to broaden the book's target audience from "readers with prior programming experience" to "complete beginners in programming or embedded development," and to prioritize the English version as the primary language.

## Frequently Asked Questions

### 1. Is this book suitable for programming beginners?

This book is especially suitable for two types of beginners: those who are completely new to programming, and those who have programming experience but haven't worked with Rust or embedded development before.

### 2. How does this book differ from other Rust learning resources?

This book takes a unique approach by teaching Rust concepts through creating colorful games (especially embedded games in Part 1). Additionally, it explores how to effectively summon AI as your personal Rust tutor to help readers gain a deeper understanding of Rust's features and best practices.

### 3. Is this book complete?

This book is currently being written and is expected to be completed around the end of 2025. We regularly add and update chapters. Please check back frequently for the latest content and updates.

### 4. How can I contribute to this book?

We welcome contributions of all kinds! If you spot errors, have ideas for improvements, or want to add content, please join our discussions, submit pull requests, or open issues on the GitHub repository. Your feedback will help improve this book for all Rust beginners. If your contribution makes a significant impact, I'll acknowledge you personally in the book's acknowledgments section.

### 5. Why is this a "No-Frills Edition"?

This book is planned to be published through a publishing house. During the creative process, we are using a limited-sharing simplified version to gather reader feedback and improve the content. The publisher's version will contain more comprehensive content. The main difference between the two versions is: the simplified version provides core essentials of the complete software development process for beginners; while the publisher's version adds value by including additional content such as how to ask AI good questions and key points from AI responses, how Rust concepts solve specific problems, and which Rust concept details can be handled by AI without the need for rote memorization.

### 6. Is this book available in other languages?

Currently, we're writing the book in Chinese. If you're interested in translating it into other languages, please contact the author by opening an issue on our GitHub repository.

---

Thank you for your interest in ["From Rust Rookie to AI Full Stack: The No-Frills Edition"](https://github.com/wubin28/from-rust-rookie-to-ai-full-stack)! I hope this book becomes a valuable companion on your Rust learning journey. Remember, with the skills to ask AI the right questions and your curiosity, you can quickly grow from a Rust beginner into an AI full-stack engineer.
