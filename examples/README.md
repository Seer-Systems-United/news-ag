# news-ag Examples
The examples are based around dioxus, but the core library is framework agnostic and can be used in any Rust project.

## Dioxus Examples
There are 3 major examples.
- `web`: This is a simple web app that demonstrates how to use news-ag in a dioxus web application. news-ag runs server side.
- `desktop`: This is a simple desktop app that demonstrates how to use news-ag in a dioxus desktop application. news-ag runs client side.
- `mobile`: This is a simple mobile app that demonstrates how to use news-ag in a dioxus mobile application. news-ag runs client side.

## Running the Examples
To run the examples, you will need to have Rust and Cargo installed. You will need to have the dioxus CLI installed as well. You can install it with the following command:

```bash
cargo install dioxus-cli
```

### Web Example
To run the web example, navigate to the `web` directory and run the following command:
```bash
dx serve --package web
```

### Desktop Example
To run the desktop example, navigate to the `desktop` directory and run the following command:
```bash
dx run --package desktop
```

### Mobile Example
To run the mobile example, navigate to the `mobile` directory and run the following command:
```bash
dx run --package mobile
```
