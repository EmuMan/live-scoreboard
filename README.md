# Live Scoreboard

Live Scoreboard is an application to easily manage graphics and information for live broadcasts of sports/esports-like events. It aims to be game-agnostic, meaning all information about games is stored in the project files themselves and can be easily manipulated.

## Installation

Navigate to the [releases](https://github.com/EmuMan/live-scoreboard/releases) page, and under the most recent version (should be the top result), expand the `Assets` submenu and click on the `.msi` installer to download it.

After it is downloaded, run the installer. It should set everything up for you, allowing you to run the Live Scoreboard application by simply typing it into the start menu search bar.
## Usage

### Projects

Each project is defined by three things: a configuration file, templates for rendering, and a directory of assets. For example, here is my folder for our school's Overwatch broadcasts (you can ignore the `obs` folder, that's just for the scene collection):

![image](https://github.com/user-attachments/assets/8f0e07af-428b-4c6b-be6f-262c0e78aebf)

In this folder, there are three projects listed in three configuration files (`collegiate.json`, `slowl_2024.json`, and `spring_showdown.json`). All of these projects share the same assets and templates, meaning they all use the same resources (images, audio, etc.) and visual layout, but contain different information as far as teams go. To be as flexible as possible, **all information for the game is also stored in the .json config**. This means you can have different character lineups for different tournaments you are running, but it also means that if a new character releases, you will have to add it to all of your active projects.

Another important note is that **ALL of the assets you use MUST be located within the project's `assets` folder**. They WILL NOT LOAD otherwise. This is an intentional design choice to ensure that everything is localized and can be easily transfered between computers by simply copying the project directory and not needing to worry about anything else.

In the future, I will upload starter projects for different games, starting with Overwatch. For now, if you want to set one up, let me know individually.

You can load an existing project by navigating to the `Settings` page and clicking the `Load` button, then selecting the `.json` file for the project you would like to load.

### Running the Graphics

The graphics will not display in your OBS application until you run the webserver. I'm working to improve this interface, but for now, if you just want to get things started, you can navigate into the `Settings` tab (marked by the gear) and click on `Start Webserver`. **This will only work if you have a config loaded already!** More information is down below with the rest of the settings page.

### Updating the Graphics

As of now, graphics need to be refreshed to display changes made within the Live Scoreboard application. The application updates its information automatically, but the graphic does not know to go back and look at the new information. I am working to change that at a later date, but for now, it is a limitation that needs to be respected.

## Application Tabs

### Teams

The first tab in the application (denoted by the group of people) is the teams tab. In here, you can create teams, assign icons to them, and create new players within those teams. To edit the players of a team, simply select the team in the `Teams` list and its players will show up in the `Players` list, allowing you to add, edit, remove, or reorder them. You need to have a team selected to edit the players; otherwise, your actions will not go through.

### Current Match

In this page, you can set information for the current match as displayed in the waiting screen, on the in-game scoreboard, and in the rounds overview graphic. The two team dropdowns on the top allow you to pick teams from any of the ones listed in the `Teams` page (see above). If the teams have visually switched sides within the game's scoreboard (e.g. in Overwatch, if one team prefers defense), you can simply check the `Swap Scoreboard?` option to visually flip the sides so it aligns with the game. This checkbox does not affect anything else; it simply sends an extra signal to the scoreboard.

The rounds category is where you can manage the progress of the match, including previously played maps and their scores. Gamemodes and maps can be selected from their respective pools, and the scores for team 1 and team 2 can be adjusted below. The `Completed?` checkbox marks the match as completed, which causes it to display one team or the other as the winner when viewing the rounds graphic.

The score is also derived from this rounds graphic. For each round, the team with the higher score is awarded one point. Draws result in neither team receiving a point. Therefore, the default scores of all zero result in neither team having any points. I am aware that this should take into account the completed status of each round, but for now, it does not. This will be in the next release.

If you need to change the number of rounds (either due to tournament rules or due to draws extending a match), you can add them in the `Settings` page as described below.

### Bracket

Here, you can format the information contained inside the bracket graphic. This page is separated into multiple stages, the number of which can be configured in the `Settings` page as described below. A matchup can be created by clicking the `+` button, and removed by clicking the trash button. Inside these matchups, you can assign two teams and their respective scores. If the `Completed?` checkbox is marked, it will visually indicate the team with the higher score as the winner on the graphic.

### Resources

These are configurable values that can be accessed directly within the graphics. Each resource is defined by a name and a value, where the name must be a single, continuous word containing only alphanumeric characters and underscores. This is not enforced by the software as of now, but ignoring this limitation will cause your data to display incorrectly.

These resources can be accessed within templates by accessing a variable prefixed with either `image_` or `string_`. For example, to get the main logo as a path, you could write `{{ image_main_logo }}` and the templating engine (Tera) will automatically replace it with your specified value.

### Settings

At the top of this page, you can load or save the current loaded config. This must be done in order for the webserver to run, which can be initiated in the next box down. Note that if you change one of the templates, you will have to restart the webserver; otherwise, this should just be a single button press per session. This is a little cumbersome, so I'm looking to rework it in the future.

The box below those describes all of the event information, including its name, the number of rounds in each match, and the number of stages within the bracket. The name will appear in various graphics, and changing the numbers will automatically reflect in their respective pages.

The rest of the page is dedicated to game-specific information. Here, you can create different gamemodes, and to each of those gamemodes, you can assign different maps (this information is used in the `Current Match` page to describe rounds). In this system, each map is unique to one gamemode, and each gamemode has its own set of maps. The roles and characters, however, follow a different system. The character list exists independently of the roles list, allowing you to assign any combination of role and character to a given player. This was inspired by games such as League of Legends, where roles and characters are somewhat associated but not directly intertwined.

## Known Issues

As this software is in early development, there are many areas that I have on my radar to improve. This includes:

- A better webserver status system
- Automatic graphics updating
- Better error messages
- Fix round completion system
- Example projects to use as a base
- Support for audio and video streams

There are probably others I'm forgetting, but that's what I could think of off the top of my head.
