# ChatterSpace
#### Video Demo:  https://www.youtube.com/watch?v=aYNuidiCF6w
## Description

The ChatterSpace is a lightweight, real-time communication system built using **Rust**, leveraging **WebSocket** technology for instant message delivery and **SQLite** for persistent data storage. The application provides a robust, room-based chat system where users can create, join, and interact within unique chat rooms. Each room supports message broadcasting to all participants, ensuring seamless communication.

### Key Features

- **Real-time Messaging:** Utilizes WebSocket for low-latency, two-way communication.
- **Room Management:** Create, delete, and manage unique chat rooms with persistent storage.
- **Message History:** Stores messages in SQLite for easy retrieval and persistence.
- **Cache Optimization:** Combines in-memory caching with SQLite synchronization for efficient data management.
- **Scalable Backend:** Powered by Rust for high performance and safety.

### Use Cases

This application serves as a platform for:

- Team collaboration tools.
- Community chat services.
- Real-time communication in web and mobile applications.