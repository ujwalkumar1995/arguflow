<p align="center">
  <img height="100" src="https://raw.githubusercontent.com/arguflow/blog/5ef439020707b0e27bf901c8f6b4fb1f487a78d4/apps/frontend/public/assets/horizontal-logo.svg" alt="Arguflow">
</p>
<p align="center">
<strong><a href="https://docs.arguflow.ai">Documentation</a> | <a href="https://blog.arguflow.ai">Blog</a> | <a href="https://www.youtube.com/watch?v=jX84z2QkfUM&t=7s">Demo Video</a>
</strong>
</p>

<p align="center">
    <a href="https://github.com/arguflow/arguflow/stargazers">
        <img src="https://img.shields.io/github/stars/arguflow/arguflow.svg?style=flat&color=yellow" alt="Github stars"/>
    </a>
    <a href="https://github.com/arguflow/arguflow">
        <img src="https://img.shields.io/github/last-commit/arguflow/arguflow.svg?style=flat&color=blue" alt="GitHub last commit"/>
    </a>
    <a href="https://github.com/arguflow/arguflow/issues">
        <img src="https://img.shields.io/github/issues/arguflow/arguflow.svg?style=flat&color=success" alt="GitHub issues"/>
    </a>
    <a href="https://discord.gg/CuJVfgZf54">
        <img src="https://img.shields.io/discord/1130153053056684123.svg?label=Discord&logo=Discord&colorB=7289da&style=flat" alt="Join Discord"/>
    </a>
    <a href="https://matrix.to/#/#arguflow-general:matrix.zerodao.gg">
        <img src="https://img.shields.io/badge/matrix-join-purple?style=flat&logo=matrix&logocolor=white" alt="Join Matrix"/>
    </a>
    <a href="https://t.me/+vUOq6omKOn5lY2Zh">
        <img src="https://img.shields.io/badge/telegram-join-purple?style=flat&logo=telegram&logocolor=white" alt="Join Matrix"/>
    </a>
</p>

<p align="center">
    <b>Arguflow is a truly all-in-one service for hosting AI powered semantic search and LLM retrieval-augmented generation (RAG) on your data.</b>
</p>
<a href="https://www.youtube.com/watch?v=jX84z2QkfUM&t=7s">

![arguflow architecture diagram](/assets/arguflow-system-diagram.png)
</a>

## Live Demos

- [OpenCaselist search](https://search.arguflow.ai)
- [OpenCaselist RAG](https://chat.arguflow.ai)
- [Enron Corpus search](https://enron-search.arguflow.ai)
- [Enron Corpus RAG](https://enron-chat.arguflow.ai)

## How to contribute

1. Find an issue in the [issues tab](https://github.com/arguflow/arguflow/issues) that you would like to work on.
2. Fork the repository and clone it to your local machine
3. Create a new branch with a descriptive name: git checkout -b your-branch-name
4. Solve the issue by adding or removing code on your forked branch.
5. Test your changes locally to ensure that they do not break anything
6. Commit your changes with a descriptive commit message: git commit -m "Add descriptive commit message here"
7. Push your changes to your forked repository: git push origin your-branch-name
8. Open a pull request to the main repository and describe your changes in the PR description

## Self-hosting the API and UI's

We have a full self-hosting guide available on our [documentation page here](https://docs.arguflow.ai/self_hosting).

## Local development

### Install apt packages
```
sudo apt install curl \
gcc \
g++ \
make \
pkg-config \
python3 \
python3-pip \
libpq-dev \
libssl-dev \
openssl \
libreoffice
```
### Install NodeJS and Yarn

You can use the following, but we recommend using [NVM](https://github.com/nvm-sh/nvm) and then running `yarn --cwd ./server-nodejs install` .

```
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g yarn && \
    yarn --cwd ./server-nodejs install
```

### Install node requirements

`yarn --cwd ./server/server-nodejs`

### Setup env's

```
cp .env.chat ./chat/.env
cp .env.search ./search/.env
cp .env.server ./server/.env
```

### Add your `OPENAI_API_KEY` to `./server/.env`

[Here is a guide for acquiring that](https://blog.streamlit.io/beginners-guide-to-openai-api/#get-your-own-openai-api-key).

#### Steps once you have the key

1. Open the `./server/.env` file
2. Replace the value for `OPENAI_API_KEY` to be your own OpenAI API key.

### Start docker container services needed for local dev

```
cat .env.chat .env.search .env.server .env.docker-compose > .env
./convenience.sh -l
```

### Start services for local dev

We know this is bad. Currently, We recommend managing this through tmux or VSCode terminal tabs.

```
cd server
cargo watch -x run
```

```
cd search
yarn
yarn dev
```

```
cd chat
yarn
yarn dev
```
