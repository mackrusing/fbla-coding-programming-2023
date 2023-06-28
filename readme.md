# Activity Tracker

![language: TypeScript](https://img.shields.io/badge/language-TypeScript-3178c5)
![language: Sass](https://img.shields.io/badge/language-Sass-ce649a)
![library: React](https://img.shields.io/badge/library-React-149eca)
![framework: Next](https://img.shields.io/badge/framework-Next-black)
![framework: Express](https://img.shields.io/badge/framework-Express-white)

The repo contains two, separatly-run components: the [API](#api) (data handling) and the [web app](#web-app) (ui).

<!-- project overview here -->

## API

The API consists of all the logic for creating, reading, updating, and deleting
application data. It handles the validation, reading, and writing of student and
event data stored in json files.

> demo avalible on https://api.activity-tracker.mackk.dev/

### Endpoints

Below is an overview of the avalible API endpoints. All `POST`, `PUT`, `PATCH`, and `DELETE` all require authentication, more info on that [below](#authentication).

|                                          | GET | POST | PUT | PATCH | DELETE |
| ---------------------------------------- | --- | ---- | --- | ----- | ------ |
| `/students.json`                         | ✓   | ✓    | ✗   | ✗     | ✗      |
| `/students/[id].json`                  | ✓   | ✗    | ✓   | ✓     | ✓      |
| `/students/[id]/completed_events.json` | ✗   | ✓    | ✗   | ✓     | ✗      |
| `/events.json`                           | ✓   | ✓    | ✗   | ✗     | ✗      |
| `/events/[id].json`                    | ✓   | ✗    | ✓   | ✓     | ✓      |

<!-- For a complete overview of the API's functionality and a list of it's endpoints, check the [API Docs](./docs/index.md). -->

### Authentication

Authenticated actions (all `POST`, `PUT`, `PATCH`, and `DELETE` requests) require the user provide credentials using the Basic Auth scheme in the Authentication header.

For example a `POST` request to `/students.json` will always return a an error without providing the Authentication header.

<!-- add example with curl ? -->

## Web App

The Web application gives administrators and students an easy way to track participation in school events. The application consists of an "admin" site (`/admin/dashboard`) containting forms for adding students, creating events, and logging completed activities, and a "public" site (`/leaderboard`) which contains a leaderboard-like inteface.

> demo avalible on https://activity-tracker.mackk.dev/

### Site map

- `/leaderboard`: student leaderboard
- `/students/[id]`: info about a single student's rank, point accumulation, and completed events
- `/admin/dashboard`: a dashboard for administrators to add students create events and open event pages
- `/admin/events/[id]`: page for admins to log students as they complete events
