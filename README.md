Real-Time User Authentication and WebSocket Application
This is a backend project built with Rust using the Actix Web framework. It provides user authentication (registration and login) and supports real-time communication using WebSockets. The project uses Diesel ORM to interact with a PostgreSQL database and ensures secure password handling with bcrypt and JWT (JSON Web Tokens).

Features
User Registration: Allows users to register with a username and password. The password is securely hashed before being stored.
User Login: Users can log in using their username and password. Upon successful login, a JWT token is generated for further authentication.
Real-Time Communication: The server supports WebSocket connections for real-time messaging between clients.
Heartbeats: The server periodically checks the connection status and disconnects idle clients after a certain period.
Project Structure
src/main.rs: Main entry point of the application, handling HTTP routes for registration, login, and WebSocket connections.
src/auth.rs: Handles password hashing, verification, and JWT generation/decoding.
src/database.rs: Contains database interaction logic for user management (creating users, retrieving users by username).
src/models.rs: Defines the User model and the NewUser struct for interacting with the database.
src/websocket.rs: Implements WebSocket communication, including heartbeats and message handling.
Setup
Prerequisites
Rust: Ensure you have Rust installed. You can install it from here.
PostgreSQL: The project uses a PostgreSQL database. Make sure you have it installed and running.
Environment Setup
Clone the repository:

bash
Copy
Edit
git clone https://github.com/your-username/your-project-name.git
cd your-project-name
Install dependencies:

bash
Copy
Edit
cargo build
Set up the PostgreSQL database:

Create a PostgreSQL database and user with appropriate permissions.
Update the database connection details in src/database.rs.
Run database migrations (if applicable):

Use Diesel CLI to run migrations if you have them configured.
Start the server:

bash
Copy
Edit
cargo run
The server will start on http://127.0.0.1:8080.

Endpoints
POST /register: Register a new user.

Request body:
json
Copy
Edit
{
  "username": "example",
  "password": "password123"
}
Response:
201 Created: User successfully created.
500 Internal Server Error: Error creating user.
POST /login: Log in with an existing user.

Request body:
json
Copy
Edit
{
  "username": "example",
  "password": "password123"
}
Response:
200 OK: JWT token returned on successful login.
401 Unauthorized: Invalid credentials.
500 Internal Server Error: Error verifying password.
WebSocket Communication
Clients can establish a WebSocket connection to the server. The server will handle incoming messages and broadcast them to other connected clients.

WebSocket Events
Text Message: Clients can send text messages to the server, which will be broadcasted to all connected clients.
Pong Message: Clients can send a Pong message to reset the heartbeat timer.
Security
Password Hashing: Passwords are hashed using the bcrypt algorithm before being stored in the database.
JWT Authentication: JWT tokens are used for stateless authentication. Tokens are valid for 1 hour.
Dependencies
actix-web: Web framework for Rust.
actix-web-actors: WebSocket support for Actix.
diesel: ORM for interacting with PostgreSQL.
bcrypt: Library for password hashing.
jsonwebtoken: Library for creating and verifying JWT tokens.
serde: Library for serializing and deserializing data.
License
This project is licensed under the MIT License - see the LICENSE file for details.

Feel free to adjust the instructions and details to better fit your project!





## Project Progress and Remaining

### Backend Development
- **Progress**: 70%
  - Completed API routes for user registration and login.
  - JWT authentication implemented.
  - Database integration using Diesel.

- **Remaining**:
  - Implement user profile management.
  - Add password reset functionality.
  - Integrate additional security features (e.g., rate limiting, logging).

### Frontend Development
- **Progress**: 50%
  - Basic UI setup for registration and login.
  - UI components for user interaction in progress.

- **Remaining**:
  - Finalize UI for login, registration, and error handling.
  - Implement responsive design.
  - Add animations and improve UX.

### Error Handling & Testing
- **Progress**: 30%
  - Error handling implemented in API routes.
  - Basic testing done for authentication flow.

- **Remaining**:
  - Improve error handling for edge cases (e.g., invalid inputs, server errors).
  - Write unit and integration tests.
  - Set up automated testing for the backend and frontend.

### Future Plans
- **Progress**: 0%
  - Integrate real-time chat feature.
  - Implement advanced user management (e.g., user roles, permissions).
  - Finalize deployment for production.

---

**Total Progress**: 55% (Approximate)
