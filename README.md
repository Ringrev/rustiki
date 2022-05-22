# Rustiki - A Rust Wiki Framework
Authors: Joachim Hauso Amundsen, Linda Gjerde Hageselle, Jenny Skjeret Valderhaug.

## Description
The open-source project Rustiki is a wiki framework for Rust programmers. The project is built using the Rust fullstack framework [MoonZoon](https://github.com/MoonZoon/MoonZoon). The project Rustiki makes it possible to get a wiki up and running without having to build the entire underlying structure. Feel free to use Rustiki whether that's because you want Rust a project to play around with, or because you need a wiki. If you want to contribute to improvements/changes to Rustiki, then that's awesome! Feel free to discuss ideas for changes via issues too!

## Motivation
The project was a bachelor's assignment at the Norwegian University of Science and Technology in Ålesund, Norway. 
The project was given by Ringrev (owner of this repo) to engage students in the Rust and MoonZoon community. 
The goal was to develop a wiki framework in Rust to provide a place to share knowledge for Rust developers,
and turn it into an open-source project to further develop and expand with the development of the MoonZoon framework. 
We hope this project can serve as a stepping stone for novice Rust developers.

## Installation

### Rust
[Download Rust from here](https://www.rust-lang.org). Make sure to run `rustup update` after installation to ensure you have the latest version.

### IDE
We are using IntelliJ/Clion from JetBrains. If you do choose one of these IDEs, 
you need to search for the "Experimental features" setting in the IDE, 
and enable all experimental features that have "Rust" as part of their name.
If you do not do this, the IDE will claim there are errors in the MoonZoon code. 

**Required plugins for Jetbrains IDE:** Rust, Toml.

**Recommended linter:** Clippy.

You can also use VSCode as your IDE. If you do, remember to install the plugin rust-analyzer.

### Clone or fork Rustiki
Either clone this repo, make a fork of the repo, or download the code as a zip-file, and then open the project in your IDE.

To clone the repo, navigate to desired directory in a terminal and run the command: 

`git clone https://github.com/Ringrev/rustiki.git`

Cloning the project can also be done using a tool like GitHub Desktop, Sourcetree etc.

### Download mzoon.exe
Follow the instructions for downloading mzoon.exe as explained in [this MoonZoon demo project](https://github.com/MoonZoon/demo).
Make sure you place this file in the root of your project.

### ArangoDB
This framework uses ArangoDB to store articles and users. 

#### Necessary steps: 
1. Download the Community Edition from [ArangoDB's official site](https://www.arangodb.com).
2. Follow [instructions](https://www.arangodb.com/docs/stable/getting-started-installation.html) for installation on your device.
3. [Create a new database](https://aragog.rs/book/arangodb.html) you want to use for you project, and a user that can access it.
4. Open ArangoDB web interface at http://localhost:8529. If this does not work see [ArangoDB documentation](https://www.arangodb.com/docs/stable/troubleshooting-arangod.html).
5. Choose the database you created and log in with the user you created.
6. Once logged in, create one collection called `Article` and one called `User`.
7. On your computer, create the following environment variables:

   - `DB_HOST` containing value `http://localhost:8529`. Unless you have hosted ArangoDB elsewhere - then the value should be the link to this.
   - `DB_NAME` containing the name of the database you created.
   - `DB_USER` containing the name of the user you created.
   - `DB_PASSORD` containing the password you made for the user.
   
### Firebase
This framework uses Firebase Auth for registration and login. You can of course use a different method of authentication if you wish, but below you will see the steps needed to use Firebase in your Rustiki project.

#### Necessary steps:
1. [Sign up to firebase](https://firebase.google.com/).
2. In the console, create a new project.
3. In Build -> Authentication, click "Get started" option.
4. Firebase will ask you to add your first sign-in method. Choose "Email/Password", enable it, and click Save button.
5. Now go to Project Settings, and you will see a "Web API Key" listed. Copy the value of this key.
6. Create an environment variable on your computer called `API_KEY` with the value you just copied.

**If Rustiki with Firebase authentication is used by a business:** Remember to include Firebase's terms of service in your own terms of service.

### Aragog
This framework uses [Aragog](https://aragog.rs/book/) to communicate with ArangoDB.
It generates a schema file based on the collections in your database.

#### Necessary steps:
1. In your Rustiki project, run command `cargo install aragog_cli` in the terminal.
2. On your computer, create environment variable `SCHEMA_PATH` and set its value to `./backend/config/db`
3. Restart your computer.

If you do not create `User` and `Article` collections, but different ones, then you will need to run `aragog discover` in the project terminal after restarting you computer. See [Aragog's official documentation](https://aragog.rs/book/) to learn more about Aragog.

### Final step
From project terminal run `./mzoon start –o` or `mzoon start -o`
Assuming all previous steps were set up correctly, the build should result in a running instance of Rustiki. Yay!

### Customizing your Rustiki project
Since Rustiki uses the Rust fullstack framework [MoonZoon](https://github.com/MoonZoon/MoonZoon), you should get familiar with how it works by looking at [its GitHub](https://github.com/MoonZoon/MoonZoon).  

### Some tips for contributors
#### How to implement new handlers in backend, responding to a request from frontend:
- Create a new constant for UpMsg in shared/lib.rs, which is the contents of the request sent from frontend to backend. 
- In backend/up_msg_handler/mod.rs, add the missing UpMsg constant you just made to the match expression (compiler probably already told you to do this)
- Create a new constant for DownMsg in shared/lib.rs, which is the contents of the response sent from backend to frontend. 
- In frontend/connection.rs, add the missing DownMsg constant you just made to the match expression (compiler probably already told you to do this)
- In backend/up_msg_handler/mod.rs, create a new mod that will be your new handler, e.g. "mod search_db;". Look at the other handlers in the project to understand how a handler can be built. 

#### Deployment
The MoonZoon framework has some discussion on deploying standalone Zoon apps in [their documentation](https://github.com/MoonZoon/MoonZoon/blob/main/docs/frontend.md). 
The Rustiki team has been able to deploy an instance of Rustiki in a Docker container on DigitalOcean, 
but experienced issues regarding the connection between frontend and backend, which remain unsolved. 
The Dockerfile used by the team can be found on the branch feature_docker if anyone wants to try it out. 
