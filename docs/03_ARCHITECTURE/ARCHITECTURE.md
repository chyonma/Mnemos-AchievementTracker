---

# Current Status

The architectural design for Mnemos v1 is complete.

The system has been divided into clear, independent modules with well-defined responsibilities:

- **Frontend (TypeScript)** handles the user interface only.
- **Backend (Rust)** contains all application logic.
- **Detection** is responsible for game discovery and process monitoring.
- **Providers** retrieve achievement data from external sources (Steam in v1).
- **Storage** manages local persistence.
- **Bridge** connects the frontend and backend through Tauri.

The architecture follows a modular, extensible design so future additions—such as Linux support, new achievement providers, and plugins—can be implemented without requiring major changes to the core application.

## Next Step

With the architecture finalized, development moves from planning to implementation.

The first milestone is to initialize the Tauri project, create the frontend and backend structure, and begin implementing the modules defined in this document.