<a name="readme-top"></a>

<!-- [![Stargazers][stars-shield]][stars-url] -->
<!-- [![Issues][issues-shield]][issues-url] -->
<!-- [![MIT License][license-shield]][license-url] -->
<!-- ![Tokei][tokei-shield] -->
![MSRV][msrv-shield]
[![Dependency Status][dependency-shield]][dependency-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/ace-skoric/ttt-server">
    <img src="assets/logo.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">Tic Tac Toe</h3>

  <p align="center">
    Authoritative server
    <br />
    <br />
    <a href="https://github.com/ace-skoric/ttt-client">Client repo</a>
    ·
    <a href="https://github.com/ace-skoric/ttt-server/issues">Report Bug</a>
    ·
    <a href="https://github.com/ace-skoric/ttt-server/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#setup">Setup</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#running-server">Running server</a></li>
        <li><a href="#running-server-with-docker">Running server with Docker</a></li>
      </ul>
    </li>
    <li><a href="#features">Features</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->
## About The Project

An authoritative server for a [Tic-Tac-Toe (Noughts and Crosses)](https://en.wikipedia.org/wiki/Tic-tac-toe) game. It features user and guest accounts, elo based matchmaking and more.

Written in rust using mostly [actix-web](https://actix.rs/) and [sea-orm](https://www.sea-ql.org/SeaORM/). It relies on PostgreSQL for data storage and Redis for storing user sessions and matchmaking.

Theoreticaly by rewriting just `ttt-game` crate it could be adapted for plethora of other 2 player games, such as chess.

You can check out the example client made with [Godot](https://godotengine.org/) at <https://github.com/ace-skoric/ttt-client>.



<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- SETUP -->
## Setup

To setup local developing environment you will need rust toolchain installed along with postgresql and redis.
Alternatively you can setup environment with the included docker compose file.

### Prerequisites

1. First clone this repo

    ```sh
    git clone https://github.com/ace-skoric/ttt-server.git
    ```

2. Move into the project directory and copy the provided `.env.example` file as `.env`

    ```sh
    cd ttt-server
    cp .env.example .env
    ```

3. Tweak `.env` file to your likings

### Running server

If you've setup rust toolchain, postgresql and redis you can run server simply by running

```sh
cargo run
```

or you could install [cargo-watch](https://github.com/watchexec/cargo-watch) and have server restart on file change

```sh
# Install cargo watch
cargo install cargo-watch
# Run server
cargo watch -x run
```

### Running server with Docker

Docker way is much easier as it sets up all prerequisites for you.
Just [enable docker buildkit](https://docs.docker.com/build/buildkit/) and run

```sh
sudo docker compose up
```


<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- FEATURES -->
## Features

* [x] User accounts
* [x] Email verification
* [x] Guest accounts
* [x] Claiming guest accounts
* [x] Real-time Elo based matchmaking
* [ ] Reconnecting to game
* [ ] Solo play vs AI mode
* [ ] Automated tests
* [ ] CI/CD

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` file for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTACT -->
## Contact

Aleksandar Skoric - askoric@protonmail.com

Project Link: [https://github.com/ace-skoric/ttt-server](https://github.com/ace-skoric/ttt-server)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

[Icon created by Freepik]("https://www.flaticon.com/free-icons/tic-tac-toe")

[Matcha](https://github.com/redis-developer/matcha) - Inspiration for matchmaking logic

<p align="right">(<a href="#readme-top">back to top</a>)</p>
<!-- MARKDOWN LINKS & IMAGES -->

<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/ace-skoric/ttt-server.svg?style=flat
[contributors-url]: https://github.com/ace-skoric/ttt-server/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/ace-skoric/ttt-server.svg?style=flat
[forks-url]: https://github.com/ace-skoric/ttt-server/network/members
[stars-shield]: https://img.shields.io/github/stars/ace-skoric/ttt-server.svg?style=flat
[stars-url]: https://github.com/ace-skoric/ttt-server/stargazers
[issues-shield]: https://img.shields.io/github/issues/ace-skoric/ttt-server.svg?style=flat
[issues-url]: https://github.com/ace-skoric/ttt-server/issues
[license-shield]: https://img.shields.io/github/license/ace-skoric/ttt-server.svg?style=flat
[license-url]: https://github.com/ace-skoric/ttt-server/blob/master/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=flat&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/askoric
[dependency-shield]: https://deps.rs/repo/github/ace-skoric/ttt-server/status.svg
[dependency-url]: https://deps.rs/repo/github/ace-skoric/ttt-server
[msrv-shield]: https://img.shields.io/badge/rustc-1.65+-ab6000.svg?style=flat
[tokei-shield]: https://img.shields.io/tokei/lines/github/ace-skoric/ttt-server